fn count_str_memory_len(cs: &[u8]) -> usize {
    fn count_str_memory_len_impl(cs: &[u8], acc: usize) -> usize {
        match cs {
            [] => acc,
            [b'\\', c, rest @ ..] if *c == b'\\' || *c == b'"' => {
                count_str_memory_len_impl(rest, acc + 1)
            }
            [b'\\', b'x', _, _, rest @ ..] => count_str_memory_len_impl(rest, acc + 1),
            [_, rest @ ..] => count_str_memory_len_impl(rest, 1 + acc),
        }
    };

    cs.len() - (count_str_memory_len_impl(cs, 0) - 2)
}

fn count_str_code_len(cs: &[u8]) -> usize {
    fn count_str_code_len_impl(cs: &[u8], acc: usize) -> usize {
        match cs {
            [] => acc,
            [c, rest @ ..] if *c == b'\\' || *c == b'"' => count_str_code_len_impl(rest, acc + 2),
            [_, rest @ ..] => count_str_code_len_impl(rest, acc + 1),
        }
    }

    2 + count_str_code_len_impl(cs, 0) - cs.len()
}

fn solve(input: impl std::io::BufRead, count_len: fn(&[u8]) -> usize) -> usize {
    input
        .lines()
        .map(|line| count_len(line.unwrap().as_bytes()))
        .sum()
}

pub fn solve_a(input: impl std::io::BufRead) -> usize {
    solve(input, count_str_memory_len)
}

pub fn solve_b(input: impl std::io::BufRead) -> usize {
    solve(input, count_str_code_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_a() {
        assert_eq!(2, solve_a(r#""""#.as_bytes()));
        assert_eq!(2, solve_a(r#""abc""#.as_bytes()));
        assert_eq!(3, solve_a(r#""aaa\"aaa""#.as_bytes()));
        assert_eq!(5, solve_a(r#""\x27""#.as_bytes()));
    }

    #[test]
    fn check_b() {
        assert_eq!(4, solve_b(r#""""#.as_bytes()));
        assert_eq!(4, solve_b(r#""abc""#.as_bytes()));
        assert_eq!(6, solve_b(r#""aaa\"aaa""#.as_bytes()));
        assert_eq!(5, solve_b(r#""\x27""#.as_bytes()));
    }
}
