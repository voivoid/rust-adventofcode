fn direction_to_step(c: u8) -> i64 {
    match c {
        b'(' => 1,
        b')' => -1,
        _ => 0,
    }
}

pub fn solve_a(input: impl std::io::BufRead) -> i64 {
    input.bytes().map(|c| direction_to_step(c.unwrap())).sum()
}

pub fn solve_b(input: impl std::io::BufRead) -> usize {
    input
        .bytes()
        .scan(0, |state, c| {
            *state = *state + direction_to_step(c.unwrap());
            Some(*state)
        })
        .position(|n| n == -1)
        .expect("")
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_a() {
        assert_eq!(0, solve_a("(())".as_bytes()));
        assert_eq!(0, solve_a("()()".as_bytes()));
        assert_eq!(3, solve_a("(((".as_bytes()));
        assert_eq!(3, solve_a("(()(()(".as_bytes()));
        assert_eq!(3, solve_a("))(((((".as_bytes()));
        assert_eq!(-1, solve_a("())".as_bytes()));
        assert_eq!(-1, solve_a("))(".as_bytes()));
        assert_eq!(-3, solve_a(")))".as_bytes()));
        assert_eq!(-3, solve_a(")())())".as_bytes()));
    }

    #[test]
    fn check_b() {
        assert_eq!(1, solve_b(")".as_bytes()));
        assert_eq!(5, solve_b("()())".as_bytes()));
    }
}
