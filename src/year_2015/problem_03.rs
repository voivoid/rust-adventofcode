type PosSet = std::collections::BTreeSet<Pos>;

struct Step(isize, isize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(isize, isize);

fn dir_to_step(c: u8) -> Step {
    match c {
        b'<' => Step(-1, 0),
        b'>' => Step(1, 0),
        b'^' => Step(0, -1),
        b'v' => Step(0, 1),
        _ => panic!("unexpected move"),
    }
}

fn apply_step(pos: Pos, step: Step) -> Pos {
    Pos(pos.0 + step.0, pos.1 + step.1)
}

fn insert_pos(mut btree: PosSet, pos: Pos) -> PosSet {
    btree.insert(pos);
    btree
}

fn get_visited_locations(dirs: impl Iterator<Item = u8>) -> PosSet {
    let start_pos = Pos(0, 0);
    let visited_locations: PosSet = dirs
        .map(dir_to_step)
        .scan(start_pos, |current_pos, step| {
            *current_pos = apply_step(*current_pos, step);
            Some(*current_pos)
        })
        .collect();

    insert_pos(visited_locations, start_pos)
}

pub fn solve_a(input: impl std::io::BufRead) -> usize {
    let results = input
        .lines()
        .map(|line| get_visited_locations(line.unwrap().into_bytes().into_iter()).len());

    results.sum()
}

pub fn solve_b(input: impl std::io::BufRead) -> usize {
    let results = input.lines().map(|line| {
        let line = line.unwrap();
        let (santa, robot): (Vec<_>, Vec<_>) = line
            .into_bytes()
            .into_iter()
            .enumerate()
            .partition(|(i, _)| i % 2 == 0);

        let santa_visited = get_visited_locations(santa.into_iter().map(|(_, d)| d));
        let robot_visited = get_visited_locations(robot.into_iter().map(|(_, d)| d));

        santa_visited.union(&robot_visited).count()
    });

    results.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_a() {
        assert_eq!(2, solve_a(">".as_bytes()));
        assert_eq!(4, solve_a("^>v<".as_bytes()));
        assert_eq!(2, solve_a("^v^v^v^v^v".as_bytes()));
    }

    #[test]
    fn check_b() {
        assert_eq!(3, solve_b("^v".as_bytes()));
        assert_eq!(3, solve_b("^>v<".as_bytes()));
        assert_eq!(11, solve_b("^v^v^v^v^v".as_bytes()));
    }
}
