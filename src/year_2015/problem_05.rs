static NAUGHTY: [&[u8; 2]; 4] = [b"ab", b"cd", b"pq", b"xy"];

fn is_vowel(c: &u8) -> bool {
    let vowels = b"aeiou";
    vowels.contains(&c)
}

fn has_three_vowels(cs: &[u8]) -> bool {
    cs.iter().filter(|c| is_vowel(c)).count() >= 3
}

fn has_letter_twice_in_a_row(cs: &[u8]) -> bool {
    match cs {
        [] => false,
        [a, b, ..] if a == b => true,
        [_, rest @ ..] => has_letter_twice_in_a_row(rest),
    }
}

fn has_no_naughty_strings(cs: &[u8]) -> bool {
    match cs {
        [] => true,
        [a, b, ..] if NAUGHTY.contains(&&[*a, *b]) => false,
        [_, rest @ ..] => has_no_naughty_strings(rest),
    }
}

fn has_pair_of_letters_twice(cs: &[u8]) -> bool {
    fn has_second_pair_of_letters(l1: &u8, l2: &u8, cs: &[u8]) -> bool {
        match cs {
            [] => false,
            [a, b, ..] if a == l1 && b == l2 => true,
            [_, rest @ ..] => has_second_pair_of_letters(l1, l2, rest),
        }
    }

    match cs {
        [] => false,
        [a, b, rest @ ..] if has_second_pair_of_letters(a, b, rest) => true,
        [_, rest @ ..] => has_pair_of_letters_twice(rest),
    }
}

fn has_double_letter_with_a_letter_between(cs: &[u8]) -> bool {
    match cs {
        [] => false,
        [a, _, c, ..] if a == c => true,
        [_, rest @ ..] => has_double_letter_with_a_letter_between(rest),
    }
}

fn solve(input: impl std::io::BufRead, is_naughty: fn(&[u8]) -> bool) -> usize {
    input
        .lines()
        .filter(|line| {
            let line = line.as_ref().unwrap();
            let chars = line.as_bytes();
            is_naughty(chars)
        })
        .count()
}

pub fn solve_a(input: impl std::io::BufRead) -> usize {
    solve(input, |chars| {
        has_three_vowels(chars) && has_letter_twice_in_a_row(chars) && has_no_naughty_strings(chars)
    })
}

pub fn solve_b(input: impl std::io::BufRead) -> usize {
    solve(input, |chars| {
        has_pair_of_letters_twice(chars) && has_double_letter_with_a_letter_between(chars)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_impl() {
        assert!(has_three_vowels(b"aei"));
        assert!(has_three_vowels(b"xazegov"));
        assert!(has_three_vowels(b"aeiouaeiouaeiou"));
        assert!(!has_three_vowels(b"aci"));

        assert!(has_letter_twice_in_a_row(b"xx"));
        assert!(has_letter_twice_in_a_row(b"abcdde"));
        assert!(has_letter_twice_in_a_row(b"aabbccdd"));
        assert!(!has_letter_twice_in_a_row(b"abc"));

        assert!(!has_no_naughty_strings(b"ab"));
        assert!(!has_no_naughty_strings(b"cd"));
        assert!(has_no_naughty_strings(b"ef"));

        assert!(has_double_letter_with_a_letter_between(b"xyx"));
        assert!(has_double_letter_with_a_letter_between(b"abcdefeghi"));
        assert!(has_double_letter_with_a_letter_between(b"aaa"));
        assert!(!has_double_letter_with_a_letter_between(b"aab"));

        assert!(has_pair_of_letters_twice(b"xyxy"));
        assert!(has_pair_of_letters_twice(b"aabcdefgaa"));
        assert!(!has_pair_of_letters_twice(b"aaa"));
    }

    #[test]
    fn check_a() {
        assert_eq!(1, solve_a("ugknbfddgicrmopn".as_bytes()));
        assert_eq!(1, solve_a("aaa".as_bytes()));
        assert_eq!(0, solve_a("jchzalrnumimnmhp".as_bytes()));
        assert_eq!(0, solve_a("haegwjzuvuyypxyu".as_bytes()));
        assert_eq!(0, solve_a("dvszwmarrgswjxmb".as_bytes()));
    }

    #[test]
    fn check_b() {
        assert_eq!(1, solve_b("qjhvhtzxzqqjkmpb".as_bytes()));
        assert_eq!(1, solve_b("xxyxx".as_bytes()));
        assert_eq!(0, solve_b("uurcxstgmygtbstg".as_bytes()));
        assert_eq!(0, solve_b("ieodomkazucvgmuy".as_bytes()));
    }
}
