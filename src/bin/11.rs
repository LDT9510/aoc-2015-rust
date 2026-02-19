use itertools::Itertools;

advent_of_code::solution!(11);

fn increment_password(mut step: u8, password: &mut [u8]) {
    for c in password.iter_mut().rev() {
        *c += step;

        if *c > b'z' {
            *c = b'a' + (*c - b'z' - 1);
            step = 1;
        } else {
            break;
        }
    }
}

fn is_password_valid(password: &[u8]) -> bool {
    let req1 = password
        .iter()
        .tuple_windows()
        .any(|(a, b, c)| a + 1 == *b && b + 1 == *c);

    let req2 = !password.iter().any(|c| [b'i', b'o', b'l'].contains(c));

    let req3 = (0..password.len() - 2).any(|i| {
        let pair = &password[i..i + 2];

        if pair.iter().all_equal() {
            // non-overlapping
            password[i + 2..]
                .windows(2)
                .any(|next_pair| next_pair.iter().all_equal())
        } else {
            false
        }
    });

    req1 && req2 && req3
}

pub fn part_one(input: &str) -> Option<String> {
    let mut password = [0u8; 8];
    password.copy_from_slice(input.as_bytes());

    while !is_password_valid(&password) {
        increment_password(1, &mut password);
    }

    // SAFETY: the password bytes are kept in the ascii range
    Some(unsafe { String::from_utf8_unchecked(Vec::from(password)) })
}

pub fn part_two(input: &str) -> Option<String> {
    let mut password = [0u8; 8];
    password.copy_from_slice(input.as_bytes());

    while !is_password_valid(&password) {
        increment_password(1, &mut password);
    }
    
    increment_password(1, &mut password);

    while !is_password_valid(&password) {
        increment_password(1, &mut password);
    }

    // SAFETY: the password bytes are kept in the ascii range
    Some(unsafe { String::from_utf8_unchecked(Vec::from(password)) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_password() {
        let passwords = [(1, "a"), (4, "xx"), (1, "abdz"), (10, "oa"), (3, "zzzz")];
        let results = ["b", "yb", "abea", "ok", "aaac"];

        for (i, (step, pass)) in passwords.iter().enumerate() {
            let mut out = pass.to_string();
            increment_password(*step, unsafe { out.as_bytes_mut() });

            assert_eq!(results[i], out);
        }
    }

    #[test]
    fn test_is_password_valid() {
        assert!(is_password_valid("abcdffaa".as_bytes()));
        assert!(is_password_valid("ghjaabcc".as_bytes()));
        assert!(!is_password_valid("ghijklmn".as_bytes()));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.as_deref(), Some("ghjaabcc"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.as_deref(), Some("ghjbbcdd"));
    }
}
