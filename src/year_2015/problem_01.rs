fn direction_to_step(c: u8) -> isize {
    match c {
        b'(' => 1,
        b')' => -1,
        _ => 0,
    }
}

pub fn solve_a<I: std::io::BufRead>(input: I) -> isize {
    input.bytes().map(|c| direction_to_step(c.unwrap())).sum()
}

pub fn solve_b<I: std::io::BufRead>(input: I) -> usize {
    input
        .bytes()
        .scan(0, |state, c| {
            *state = *state + direction_to_step(c.unwrap());
            if *state != -1 {
                Some(*state)
            } else {
                None
            }
        })
        .count()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_a() {
        assert_eq!(0, solve_a(&b"(())"[..]));
        assert_eq!(0, solve_a(&b"()()"[..]));
        assert_eq!(3, solve_a(&b"((("[..]));
        assert_eq!(3, solve_a(&b"(()(()("[..]));
        assert_eq!(3, solve_a(&b"))((((("[..]));
        assert_eq!(-1, solve_a(&b"())"[..]));
        assert_eq!(-1, solve_a(&b"))("[..]));
        assert_eq!(-3, solve_a(&b")))"[..]));
        assert_eq!(-3, solve_a(&b")())())"[..]));
    }

    #[test]
    fn check_b() {
        assert_eq!(1, solve_b(&b")"[..]));
        assert_eq!(5, solve_b(&b"()())"[..]));
    }
}
