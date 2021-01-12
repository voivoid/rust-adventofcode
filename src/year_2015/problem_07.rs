use crate::utils::parsing::{parse_decimal, parse_str_alpha, parse_ws};
use nom::{branch::alt, bytes::complete::tag, combinator::map};

type Signal = u16;
type WireId = String;
type UnaryOp = fn(Signal) -> Signal;
type BinaryOp = fn(Signal, Signal) -> Signal;

type NomResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug)]
enum Value {
    Signal(Signal),
    Wire(WireId),
}

#[derive(Debug)]
enum Source {
    Value(Value),
    UnaryGate(UnaryOp, Value),
    BinaryGate(BinaryOp, Value, Value),
}

#[derive(Debug)]
struct Instruction {
    source: Source,
    wire: WireId,
}

type WireMap = std::collections::HashMap<WireId, Source>;
type SignalCache = std::collections::HashMap<WireId, Signal>;

fn get_value_signal(
    value: &Value,
    wiremap: &WireMap,
    signal_cache: &mut SignalCache,
) -> Option<Signal> {
    match value {
        Value::Signal(signal) => Some(*signal),
        Value::Wire(wire) => get_wire_signal(wire, wiremap, signal_cache),
    }
}

fn calc_source_signal(
    source: &Source,
    wiremap: &WireMap,
    signal_cache: &mut SignalCache,
) -> Option<Signal> {
    match source {
        Source::Value(value) => get_value_signal(value, wiremap, signal_cache),
        Source::UnaryGate(op, arg) => get_value_signal(arg, wiremap, signal_cache).map(op),
        Source::BinaryGate(op, arg1, arg2) => get_value_signal(arg1, wiremap, signal_cache)
            .zip(get_value_signal(arg2, wiremap, signal_cache))
            .map(|(a, b)| op(a, b)),
    }
}

fn get_wire_signal(
    wire: &WireId,
    wiremap: &WireMap,
    signal_cache: &mut SignalCache,
) -> Option<Signal> {
    if let Some(&signal) = signal_cache.get(wire) {
        return Some(signal);
    }

    let result = wiremap
        .get(wire)
        .and_then(|source| calc_source_signal(&source, wiremap, signal_cache));

    if let Some(signal) = result {
        signal_cache.insert(wire.to_string(), signal);
    }

    result
}

fn parse_wiremap(input: impl std::io::BufRead) -> WireMap {
    input
        .lines()
        .map(|line| {
            let Instruction { source, wire } = parse_instruction(line.unwrap().as_str());
            (wire, source)
        })
        .collect()
}

fn parse_wire(input: &str) -> NomResult<WireId> {
    parse_str_alpha(input)
}

fn parse_signal(input: &str) -> NomResult<Value> {
    map(parse_ws(parse_decimal), |s| Value::Signal(s))(input)
}

fn parse_value(input: &str) -> NomResult<Value> {
    alt((parse_signal, map(parse_wire, |w| Value::Wire(w))))(input)
}

fn parse_unary_gate(input: &str) -> NomResult<Source> {
    let (input, _) = tag("NOT")(input)?;
    let (input, arg1) = parse_value(input)?;

    Ok((input, Source::UnaryGate(|param1| !param1, arg1)))
}

fn parse_binary_gate(input: &str) -> NomResult<Source> {
    fn lshift_op(a: Signal, b: Signal) -> Signal {
        a << b
    }
    fn rshift_op(a: Signal, b: Signal) -> Signal {
        a >> b
    }
    fn and_op(a: Signal, b: Signal) -> Signal {
        a & b
    }
    fn or_op(a: Signal, b: Signal) -> Signal {
        a | b
    }

    let parse_lshift = map(tag("LSHIFT"), |_| lshift_op as BinaryOp);
    let parse_rshift = map(tag("RSHIFT"), |_| rshift_op as BinaryOp);
    let parse_and = map(tag("AND"), |_| and_op as BinaryOp);
    let parse_or = map(tag("OR"), |_| or_op as BinaryOp);

    let (input, arg1) = parse_value(input)?;
    let (input, op) = alt((parse_lshift, parse_rshift, parse_and, parse_or))(input)?;
    let (input, arg2) = parse_value(input)?;

    Ok((input, Source::BinaryGate(op, arg1, arg2)))
}

fn parse_source(input: &str) -> NomResult<Source> {
    let parse_value_as_source = map(parse_value, |value| Source::Value(value));
    alt((parse_unary_gate, parse_binary_gate, parse_value_as_source))(input)
}

fn parse_instruction(input: &str) -> Instruction {
    fn parse_instruction_impl(input: &str) -> NomResult<Instruction> {
        let (input, source) = parse_source(input)?;
        let (input, _) = parse_ws(tag("->"))(input)?;
        let (input, wire) = parse_wire(input)?;

        Ok((input, Instruction { source, wire }))
    }

    match nom::combinator::all_consuming(parse_instruction_impl)(input) {
        Ok((_, instruction)) => instruction,
        Err(e) => panic!(format!("Failed to parse instructions: {:?}", e)),
    }
}

pub fn solve_a(input: impl std::io::BufRead) -> Signal {
    let wiremap = parse_wiremap(input);
    get_wire_signal(&String::from("a"), &wiremap, &mut SignalCache::new()).unwrap()
}

pub fn solve_b(input: impl std::io::BufRead) -> Signal {
    let wiremap = parse_wiremap(input);
    let signal_a = get_wire_signal(&String::from("a"), &wiremap, &mut SignalCache::new()).unwrap();

    let mut overriden_signal_cache = SignalCache::new();
    overriden_signal_cache.insert(String::from("b"), signal_a);

    get_wire_signal(&String::from("a"), &wiremap, &mut overriden_signal_cache).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parsing() {
        matches::assert_matches!(parse_value("42"), Ok(("", Value::Signal(42))));
        matches::assert_matches!(
            parse_value("abc"),
            Ok(("", Value::Wire(w))) if w == "abc"
        );

        matches::assert_matches!(
            parse_unary_gate("NOT x"),
            Ok(("", Source::UnaryGate(_, Value::Wire(w)))) if w == "x"
        );

        matches::assert_matches!(
            parse_binary_gate("123 AND 456"),
            Ok(("", Source::BinaryGate(_, Value::Signal(123), Value::Signal(456))))
        );

        matches::assert_matches!(
            parse_binary_gate("123 OR 456"),
            Ok(("", Source::BinaryGate(_, Value::Signal(123), Value::Signal(456))))
        );

        matches::assert_matches!(
            parse_binary_gate("123 LSHIFT 2"),
            Ok(("", Source::BinaryGate(_, Value::Signal(123), Value::Signal(2))))
        );

        matches::assert_matches!(
            parse_binary_gate("456 RSHIFT 2"),
            Ok(("", Source::BinaryGate(_, Value::Signal(456), Value::Signal(2))))
        );

        matches::assert_matches!(
            parse_instruction("123 -> x"),
            Instruction{ source: Source::Value(Value::Signal(123)), wire } if wire == "x"
        );
    }

    #[test]
    fn check_a() {
        let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
        let wiremap = parse_wiremap(input.as_bytes());
        let mut signal_cache = SignalCache::new();

        assert_eq!(
            Some(123),
            get_wire_signal(&String::from("x"), &wiremap, &mut signal_cache)
        );
        assert_eq!(
            Some(456),
            get_wire_signal(&String::from("y"), &wiremap, &mut signal_cache)
        );
        assert_eq!(
            Some(72),
            get_wire_signal(&String::from("d"), &wiremap, &mut signal_cache)
        );
        assert_eq!(
            Some(507),
            get_wire_signal(&String::from("e"), &wiremap, &mut signal_cache)
        );
        assert_eq!(
            Some(492),
            get_wire_signal(&String::from("f"), &wiremap, &mut signal_cache)
        );
        assert_eq!(
            Some(65412),
            get_wire_signal(&String::from("h"), &wiremap, &mut signal_cache)
        );
        assert_eq!(
            Some(65079),
            get_wire_signal(&String::from("i"), &wiremap, &mut signal_cache)
        );

        assert!(!signal_cache.is_empty());

        assert_eq!(
            None,
            get_wire_signal(&String::from("nonexisting"), &wiremap, &mut signal_cache)
        );
    }
}
