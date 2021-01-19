use md5::Digest;

fn inc_number(mut digits: Vec<u8>) -> Vec<u8> {
    let mut digits_rev_iter = digits.iter_mut().rev();

    let carry = digits_rev_iter
        .try_for_each(|digit| match digit {
            b'9' => {
                *digit = b'0';
                Ok(())
            }
            _ => {
                *digit = *digit + 1;
                Err(())
            }
        })
        .is_ok();

    if carry {
        digits.insert(0, b'1');
    }

    digits
}

pub fn solve(input: impl std::io::BufRead, leading_zeroes: usize) -> usize {
    let check = match leading_zeroes {
        5 => |x: u8| x <= 15,
        6 => |x: u8| x == 0,
        _ => panic!("unexpected"),
    };

    let mut key: Vec<u8> = input.bytes().map(|c| c.unwrap()).collect();
    let key_len = key.len();
    let mut n = vec![b'1'];
    let mut hasher = md5::Md5::new();

    loop {
        key.extend_from_slice(&n);
        hasher.update(&key);

        let result = hasher.finalize_reset();
        if result[0] == 0 && result[1] == 0 && check(result[2]) {
            let num_str = std::str::from_utf8(&n).unwrap();
            return num_str.parse::<usize>().unwrap();
        }

        key.truncate(key_len);
        n = inc_number(n);
    }
}

pub fn solve_a(input: impl std::io::BufRead) -> usize {
    solve(input, 5)
}

pub fn solve_b(input: impl std::io::BufRead) -> usize {
    solve(input, 6)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_impl() {
        assert_eq!(b"1".to_vec(), inc_number(b"0".to_vec()));
        assert_eq!(b"10".to_vec(), inc_number(b"9".to_vec()));
        assert_eq!(b"11".to_vec(), inc_number(b"10".to_vec()));
        assert_eq!(b"20".to_vec(), inc_number(b"19".to_vec()));
        assert_eq!(b"100".to_vec(), inc_number(b"99".to_vec()));
    }

    #[test]
    fn check_a() {
        assert_eq!(43, solve_a(&b"abcdef6090"[..]));
        assert_eq!(70, solve_a(&b"pqrstuv10489"[..]));
    }
}
