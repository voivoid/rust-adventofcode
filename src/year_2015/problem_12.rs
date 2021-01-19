use serde_json::value::Value;
use serde_json::Value as JsonValue;

fn sum_numbers_a(json_val: &JsonValue) -> i64 {
    match json_val {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(arr) => arr.iter().map(sum_numbers_a).sum(),
        Value::Object(obj_map) => obj_map.values().map(sum_numbers_a).sum(),
    }
}

fn has_red(json_val: &JsonValue) -> bool {
    match json_val {
        Value::String(s) if s == "red" => true,
        _ => false,
    }
}

fn sum_numbers_b(json_val: &JsonValue) -> i64 {
    match json_val {
        Value::Array(arr) => arr.iter().map(sum_numbers_b).sum(),
        Value::Object(obj_map) if obj_map.values().any(has_red) => 0,
        Value::Object(obj_map) => obj_map.values().map(sum_numbers_b).sum(),
        _ => sum_numbers_a(json_val),
    }
}

pub fn solve(mut input: impl std::io::BufRead, sum_numbers: fn(&JsonValue) -> i64) -> i64 {
    let mut input_str = String::new();
    let bytes_read = input.read_line(&mut input_str).unwrap();
    assert!(bytes_read > 0);

    let json_val: JsonValue = serde_json::from_str(&input_str).unwrap();
    sum_numbers(&json_val)
}

pub fn solve_a(input: impl std::io::BufRead) -> i64 {
    solve(input, sum_numbers_a)
}

pub fn solve_b(input: impl std::io::BufRead) -> i64 {
    solve(input, sum_numbers_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_a() {
        assert_eq!(6, solve_a(&br#"[1,2,3]"#[..]));
        assert_eq!(6, solve_a(&br#"{"a":2,"b":4}"#[..]));
        assert_eq!(3, solve_a(&br#"[[[3]]]"#[..]));
        assert_eq!(3, solve_a(&br#"{"a":{"b":4},"c":-1}"#[..]));
        assert_eq!(0, solve_a(&br#"{"a":[-1,1]}"#[..]));
        assert_eq!(0, solve_a(&br#"[-1,{"a":1}]"#[..]));
        assert_eq!(0, solve_a(&br#"[]"#[..]));
        assert_eq!(0, solve_a(&br#"{}"#[..]));
    }

    #[test]
    fn check_b() {
        assert_eq!(6, solve_b(&br#"[1,2,3]"#[..]));
        assert_eq!(4, solve_b(&br#"[1,{"c":"red","b":2},3]"#[..]));
        assert_eq!(0, solve_b(&br#"{"d":"red","e":[1,2,3,4],"f":5}"#[..]));
        assert_eq!(6, solve_b(&br#"[1,"red",5]"#[..]));
    }
}
