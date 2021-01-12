use crate::utils::parsing::{parse_decimal, parse_str_alpha, parse_ws};
use itertools::Itertools;
use nom::bytes::complete::tag;

type NomResult<'a, T> = nom::IResult<&'a str, T>;
type City = String;
type Distance = usize;

type CitySet = std::collections::BTreeSet<City>;
type RoutesMap<'a> = std::collections::HashMap<(&'a City, &'a City), Distance>;

#[derive(Debug, Eq, PartialEq)]
struct Route {
    from: City,
    to: City,
    distance: Distance,
}

fn parse_route(input: &str) -> Route {
    fn parse_city(input: &str) -> NomResult<City> {
        parse_ws(parse_str_alpha)(input)
    }

    fn parse_distance(input: &str) -> NomResult<Distance> {
        parse_ws(parse_decimal)(input)
    }

    fn parse_route_impl(input: &str) -> NomResult<Route> {
        let (input, from) = parse_city(input)?;
        let (input, _) = tag("to")(input)?;
        let (input, to) = parse_city(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, distance) = parse_distance(input)?;

        Ok((input, Route { from, to, distance }))
    }

    match nom::combinator::all_consuming(parse_route_impl)(input) {
        Ok((_, instruction)) => instruction,
        Err(e) => panic!(format!("Failed to parse route: {:?}", e)),
    }
}

fn make_cities_set(routes: &[Route]) -> CitySet {
    routes
        .iter()
        .flat_map(|r| std::iter::once(r.to.clone()).chain(std::iter::once(r.from.clone())))
        .collect()
}

fn make_routes_map(routes: &[Route]) -> RoutesMap {
    routes
        .iter()
        .map(|route| ((&route.from, &route.to), route.distance))
        .collect()
}

fn calc_total_distance(cities: &[&City], routes_map: &RoutesMap) -> usize {
    cities
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(&c1, &c2)| routes_map.get(&(c1, c2)).unwrap())
        .sum()
}

fn solve(input: impl std::io::BufRead) -> Vec<usize> {
    let routes = input
        .lines()
        .flat_map(|line| {
            let r1 = parse_route(line.unwrap().as_str());
            let r2 = Route {
                from: r1.to.clone(),
                to: r1.from.clone(),
                distance: r1.distance,
            };
            std::iter::once(r1).chain(std::iter::once(r2))
        })
        .collect::<Vec<Route>>();

    let routes_map = make_routes_map(&routes);
    let cities = make_cities_set(&routes);

    let combinations = cities
        .iter()
        .permutations(cities.len())
        .map(|c| calc_total_distance(&c, &routes_map));

    combinations.collect()
}

pub fn solve_a(input: impl std::io::BufRead) -> usize {
    solve(input).into_iter().min().unwrap()
}

pub fn solve_b(input: impl std::io::BufRead) -> usize {
    solve(input).into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parsing() {
        assert_eq!(
            Route {
                from: String::from("London"),
                to: String::from("Dublin"),
                distance: 464
            },
            parse_route("London to Dublin = 464")
        );
    }

    #[test]
    fn check_a() {
        assert_eq!(
            605,
            solve_a(
                "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141"
                    .as_bytes()
            )
        );
    }

    #[test]
    fn check_b() {}
}
