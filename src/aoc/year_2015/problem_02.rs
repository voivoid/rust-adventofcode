type DimT = u64;
struct Dims(DimT, DimT, DimT);

fn str_to_int(input: &str) -> Result<DimT, std::num::ParseIntError> {
    DimT::from_str_radix(input, 10)
}

fn parse_dim(input: &str) -> nom::IResult<&str, DimT> {
    nom::combinator::map_res(
        nom::bytes::complete::take_while(|c: char| c.is_digit(10)),
        str_to_int,
    )(input)
}

fn parse_dims(input: &str) -> nom::IResult<&str, Dims> {
    use nom::bytes::complete::tag;

    let (input, length) = parse_dim(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, width) = parse_dim(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, height) = parse_dim(input)?;

    Ok((input, Dims(length, width, height)))
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
            let (_, dims) = parse_dims(&line).unwrap();
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
    #[test]
    fn check_a() {
        assert_eq!(58, super::solve_a("2x3x4".as_bytes()));
        assert_eq!(43, super::solve_a("1x1x10".as_bytes()));
    }

    #[test]
    fn check_b() {
        assert_eq!(34, super::solve_b("2x3x4".as_bytes()));
        assert_eq!(14, super::solve_b("1x1x10".as_bytes()));
    }
}
