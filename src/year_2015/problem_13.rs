use crate::utils::parsing::{parse_decimal, parse_str_alpha, parse_ws};
use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, combinator::map};

type Name = String;
type Happiness = isize;
type NomResult<'a, T> = nom::IResult<&'a str, T>;
type SeatsMap<'a> = std::collections::HashMap<(&'a Name, &'a Name), Happiness>;

static ME: String = String::new();

#[derive(Debug, Eq, PartialEq)]
struct Seat {
    name: Name,
    happiness: Happiness,
    neighbour: Name,
}

fn parse_name(input: &str) -> NomResult<Name> {
    parse_ws(parse_str_alpha)(input)
}

fn parse_happiness(input: &str) -> NomResult<Happiness> {
    let pos = map(parse_ws(tag("gain")), |_| 1);
    let neg = map(parse_ws(tag("lose")), |_| -1);

    let (input, sign) = alt((pos, neg))(input)?;
    let (input, happiness) = parse_ws(parse_decimal::<Happiness>)(input)?;

    Ok((input, happiness * sign))
}

fn parse_seat(input: &str) -> Seat {
    fn parse_seat_impl(input: &str) -> NomResult<Seat> {
        let (input, name) = parse_name(input)?;
        let (input, _) = tag("would")(input)?;
        let (input, happiness) = parse_happiness(input)?;
        let (input, _) = tag("happiness units by sitting next to")(input)?;
        let (input, neighbour) = parse_name(input)?;
        let (input, _) = tag(".")(input)?;

        Ok((
            input,
            Seat {
                name,
                happiness,
                neighbour,
            },
        ))
    }

    match nom::combinator::all_consuming(parse_seat_impl)(input) {
        Ok((_, seat)) => seat,
        Err(e) => panic!(format!("Failed to parse seat: {:?}", e)),
    }
}

fn make_seat_map(seats: &[Seat]) -> SeatsMap {
    seats
        .iter()
        .map(|seat| ((&seat.name, &seat.neighbour), seat.happiness))
        .collect()
}

fn calc_happiness(seats_map: &SeatsMap, names: &[&Name]) -> Happiness {
    let neighbours = names
        .iter()
        .cycle()
        .tuple_windows::<(_, _, _)>()
        .take(names.len());

    neighbours
        .map(|(&left, &mid, &right)| {
            seats_map.get(&(mid, left)).unwrap() + seats_map.get(&(mid, right)).unwrap()
        })
        .sum()
}

fn solve(input: impl std::io::BufRead, add_me: bool) -> Happiness {
    let seats: Vec<Seat> = input
        .lines()
        .map(|line| parse_seat(&line.unwrap()))
        .collect();

    let mut names: std::collections::HashSet<Name> =
        seats.iter().map(|seat| seat.name.clone()).collect();

    let mut seats_map = make_seat_map(&seats);

    if add_me {
        names.insert(ME.clone());
        for name in names.iter() {
            seats_map.insert((&ME, name), 0);
            seats_map.insert((name, &ME), 0);
        }
    }

    let max_happiness = names
        .iter()
        .permutations(names.len())
        .map(|names_permutatitions| calc_happiness(&seats_map, &names_permutatitions))
        .max()
        .unwrap();

    max_happiness
}

pub fn solve_a(input: impl std::io::BufRead) -> Happiness {
    solve(input, false)
}

pub fn solve_b(input: impl std::io::BufRead) -> Happiness {
    solve(input, true)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_parsing() {
        assert_eq!(
            Seat {
                name: "Alice".to_owned(),
                happiness: 54,
                neighbour: "Bob".to_owned()
            },
            parse_seat("Alice would gain 54 happiness units by sitting next to Bob.")
        );

        assert_eq!(
            Seat {
                name: "Alice".to_owned(),
                happiness: -79,
                neighbour: "Carol".to_owned()
            },
            parse_seat("Alice would lose 79 happiness units by sitting next to Carol.")
        );
    }

    #[test]
    fn check_a() {
        assert_eq!(
            330,
            solve_a(
                &b"Alice would gain 54 happiness units by sitting next to Bob.
                   Alice would lose 79 happiness units by sitting next to Carol.
                   Alice would lose 2 happiness units by sitting next to David.
                   Bob would gain 83 happiness units by sitting next to Alice.
                   Bob would lose 7 happiness units by sitting next to Carol.
                   Bob would lose 63 happiness units by sitting next to David.
                   Carol would lose 62 happiness units by sitting next to Alice.
                   Carol would gain 60 happiness units by sitting next to Bob.
                   Carol would gain 55 happiness units by sitting next to David.
                   David would gain 46 happiness units by sitting next to Alice.
                   David would lose 7 happiness units by sitting next to Bob.
                   David would gain 41 happiness units by sitting next to Carol."[..]
            )
        )
    }

    #[test]
    fn check_b() {}
}
