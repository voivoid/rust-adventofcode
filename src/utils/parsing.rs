use nom::{
    bytes::complete::take_while, character::complete::multispace0, combinator::map,
    combinator::map_res, sequence::delimited,
};

pub fn parse_decimal<T: std::str::FromStr>(input: &str) -> nom::IResult<&str, T> {
    map_res(take_while(|c: char| c.is_digit(10)), |s: &str| {
        s.parse::<T>()
    })(input)
}

pub fn parse_str_alpha(input: &str) -> nom::IResult<&str, String> {
    map(
        parse_ws(take_while(|c: char| c.is_alphabetic())),
        |s: &str| s.to_string(),
    )(input)
}

pub fn parse_ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> nom::IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
