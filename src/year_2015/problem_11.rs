static PROHIBITED_LETTERS: &[u8; 3] = b"iol";

fn has_no_prohibited_letters(password: &[u8]) -> bool {
    password.iter().all(|c| !PROHIBITED_LETTERS.contains(c))
}

fn has_three_increasing_letters(password: &[u8]) -> bool {
    password.windows(3).any(|w| match w {
        &[a, b, c, ..] if a + 1 == b && a + 2 == c => true,
        _ => false,
    })
}

fn has_two_pairs_of_letters(password: &[u8]) -> bool {
    password.windows(2).any(|w1| match w1 {
        [a, b, ..] if a == b => password.windows(2).any(|w2| match w2 {
            [x, y, ..] if x == y && x != a => true,
            _ => false,
        }),
        _ => false,
    })
}

fn is_secure_password(password: &Vec<u8>) -> bool {
    has_no_prohibited_letters(password)
        && has_three_increasing_letters(password)
        && has_two_pairs_of_letters(password)
}

fn next_char(c: u8) -> u8 {
    if !PROHIBITED_LETTERS.contains(&(c + 1)) {
        c + 1
    } else {
        c + 2
    }
}

fn inc_password(mut password: Vec<u8>) -> Vec<u8> {
    let mut iter = password.iter_mut().rev();

    let carry = iter
        .try_for_each(|c| match c {
            b'z' => {
                *c = b'a';
                Ok(())
            }
            c => {
                *c = next_char(*c);
                Err(())
            }
        })
        .is_ok();

    if carry {
        password.insert(0, b'a');
    }

    password
}

fn next_secure_password(mut password: Vec<u8>) -> Vec<u8> {
    while !is_secure_password(&password) {
        password = inc_password(password);
    }

    password
}

fn remove_prohibited_letters(mut password: Vec<u8>) -> Vec<u8> {
    let len = password.len();
    let chars_to_keep = password
        .iter()
        .take_while(|c| !PROHIBITED_LETTERS.contains(&c))
        .count();

    if chars_to_keep != len {
        password.truncate(chars_to_keep + 1);
        password = inc_password(password);
        password.resize(len, b'a');
    }

    password
}

fn solve(input: impl std::io::BufRead, get_next_password: fn(Vec<u8>) -> Vec<u8>) -> String {
    let password: Vec<u8> = input.bytes().map(Result::unwrap).collect();
    let result = get_next_password(remove_prohibited_letters(password));
    std::str::from_utf8(&result).unwrap().to_string()
}

pub fn solve_a(input: impl std::io::BufRead) -> String {
    solve(input, next_secure_password)
}

pub fn solve_b(input: impl std::io::BufRead) -> String {
    solve(input, |password| {
        let secure_password1 = next_secure_password(password);
        let secure_password2 = next_secure_password(inc_password(secure_password1));

        secure_password2
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_impl() {
        assert!(!is_secure_password(&b"hijklmmn".to_vec()));
        assert!(!is_secure_password(&b"abbceffg".to_vec()));
        assert!(!is_secure_password(&b"abbcegjk".to_vec()));

        assert!(is_secure_password(&b"abcdffaa".to_vec()));
        assert!(is_secure_password(&b"ghjaabcc".to_vec()));

        assert_eq!(b"a".to_vec(), inc_password(b"".to_vec()));
        assert_eq!(b"b".to_vec(), inc_password(b"a".to_vec()));
        assert_eq!(b"aa".to_vec(), inc_password(b"z".to_vec()));
        assert_eq!(b"ab".to_vec(), inc_password(b"aa".to_vec()));
        assert_eq!(b"zb".to_vec(), inc_password(b"za".to_vec()));
        assert_eq!(b"ba".to_vec(), inc_password(b"az".to_vec()));
        assert_eq!(b"aaa".to_vec(), inc_password(b"zz".to_vec()));

        assert_eq!(b"".to_vec(), remove_prohibited_letters(b"".to_vec()));
        assert_eq!(b"a".to_vec(), remove_prohibited_letters(b"a".to_vec()));
        assert_eq!(b"m".to_vec(), remove_prohibited_letters(b"l".to_vec()));
        assert_eq!(b"paaa".to_vec(), remove_prohibited_letters(b"oabc".to_vec()));
        assert_eq!(b"abcjaaa".to_vec(), remove_prohibited_letters(b"abcidef".to_vec()));
    }

    #[test]
    fn check_a() {
        assert_eq!("abcdffaa", solve_a("abcdefgh".as_bytes()));
        assert_eq!("ghjaabcc", solve_a("ghijklmn".as_bytes()));
    }
}
