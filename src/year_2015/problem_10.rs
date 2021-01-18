use itertools::Itertools;
type Digit = u8;

fn usize_to_digit(n: usize) -> u8 {
    assert!(n <= 9);
    b'0' + (n as u8)
}

fn look_and_say(digits: &Vec<Digit>) -> Vec<Digit> {
    digits
        .iter()
        .map(|digit| (1, digit))
        .coalesce(|a, b| match (a, b) {
            ((occurrences, digit1), (_, digit2)) if digit1 == digit2 => {
                Ok((occurrences + 1, digit1))
            }
            pair @ _ => Err(pair),
        })
        .flat_map(|(n, &digit)| std::iter::once(usize_to_digit(n)).chain(std::iter::once(digit)))
        .collect()
}

fn solve(input: impl std::io::BufRead, iterations: usize) -> usize {
    let input_digits: Vec<Digit> = input.bytes().try_collect().unwrap();

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
