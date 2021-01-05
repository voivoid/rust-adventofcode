type DimT = u64;

#[derive(Debug, PartialEq, Eq)]
struct Dims(DimT, DimT, DimT);

use crate::aoc::utils::parsing::parse_decimal;
use nom::character::complete::char;

fn parse_dims_impl(input: &str) -> nom::IResult<&str, Dims> {
    let (input, length) = parse_decimal::<DimT>(input)?;
    let (input, _) = char('x')(input)?;
    let (input, width) = parse_decimal::<DimT>(input)?;
    let (input, _) = char('x')(input)?;
    let (input, height) = parse_decimal::<DimT>(input)?;

    Ok((input, Dims(length, width, height)))
}

fn parse_dims(input: &str) -> Dims {
    match nom::combinator::all_consuming(parse_dims_impl)(input) {
        Ok((_, dims)) => dims,
        Err(e) => panic!(format!("Failed to parse dims: {:?}", e)),
    }
}

fn calc_area_a(Dims(l, w, h): Dims) -> DimT {
    let sides = [l * w, w * h, h * l];
    let min_side = sides.iter().min().unwrap();

    let box_area: DimT = sides.iter().map(|s| s * 2).sum();
    box_area + min_side
}

fn calc_area_b(Dims(l, w, h): Dims) -> DimT {
    let mut sides = [l, w, h];
    sides.sort();

    let wrap_ribbon = sides[0] * 2 + sides[1] * 2;
    let bow_ribbon = l * w * h;

    wrap_ribbon + bow_ribbon
}

fn solve(input: impl std::io::BufRead, calc_area: fn(Dims) -> DimT) -> DimT {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let dims = parse_dims(&line);
            calc_area(dims)
        })
        .sum()
}

pub fn solve_a(input: impl std::io::BufRead) -> DimT {
    solve(input, calc_area_a)
}

pub fn solve_b(input: impl std::io::BufRead) -> DimT {
    solve(input, calc_area_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parsing() {
        assert_eq!(Dims(1, 22, 333), parse_dims("1x22x333"));
    }

    #[test]
    fn check_a() {
        assert_eq!(58, solve_a("2x3x4".as_bytes()));
        assert_eq!(43, solve_a("1x1x10".as_bytes()));
    }

    #[test]
    fn check_b() {
        assert_eq!(34, solve_b("2x3x4".as_bytes()));
        assert_eq!(14, solve_b("1x1x10".as_bytes()));
    }
}
