pub fn parse_decimal<T: std::str::FromStr>(input: &str) -> nom::IResult<&str, T> {
    nom::combinator::map_res(
        nom::bytes::complete::take_while(|c: char| c.is_digit(10)),
        |s: &str| s.parse::<T>(),
    )(input)
}

pub fn parse_ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> nom::IResult<&'a str, O, E>,
{
    nom::sequence::delimited(
        nom::character::complete::multispace0,
        inner,
        nom::character::complete::multispace0,
    )
}
