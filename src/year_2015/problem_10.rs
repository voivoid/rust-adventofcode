use itertools::Itertools;

fn usize_to_digit(n: usize) -> u8 {
    assert!(n <= 9);
    b'0' + (n as u8)
}

fn look_and_say(digits: &Vec<u8>) -> Vec<u8> {
    digits
        .iter()
        .map(|digit| (1, digit))
        .coalesce(|e1, e2| match (e1, e2) {
            ((n1, digit1), (n2, digit2)) if digit1 == digit2 => Ok((n1 + n2, digit1)),
            _ => Err((e1, e2)),
        })
        .flat_map(|(n, digit)| std::iter::once(usize_to_digit(n)).chain(std::iter::once(*digit)))
        .collect()
}

fn solve(input: impl std::io::BufRead, iterations: usize) -> usize {
    let input_digits: Vec<u8> = input.bytes().try_collect().unwrap();

    itertools::iterate(input_digits, look_and_say)
        .nth(iterations)
        .unwrap()
        .len()
}

pub fn solve_a(input: impl std::io::BufRead) -> usize {
    solve(input, 40)
}

pub fn solve_b(input: impl std::io::BufRead) -> usize {
    solve(input, 50)
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_impl() {
        assert_eq!(b"11".to_vec(), super::look_and_say(&b"1".to_vec()));
        assert_eq!(b"21".to_vec(), super::look_and_say(&b"11".to_vec()));
        assert_eq!(b"1211".to_vec(), super::look_and_say(&b"21".to_vec()));
        assert_eq!(b"111221".to_vec(), super::look_and_say(&b"1211".to_vec()));
        assert_eq!(b"312211".to_vec(), super::look_and_say(&b"111221".to_vec()));
    }
    #[test]
    fn check_a() {
        assert_eq!(6, super::solve("1".as_bytes(), 4));
    }
}
