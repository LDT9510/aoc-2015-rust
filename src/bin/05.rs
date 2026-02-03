use advent_of_code::utils::string_utils::StrExtensionsUtils;
use itertools::Itertools;

advent_of_code::solution!(5);

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const DISALLOWED: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn is_nice1(s: &str) -> bool {
    let cond1 = s.chars().filter(|c| VOWELS.contains(c)).count() >= 3;
    let cond2 = s.as_bytes().iter().tuple_windows().any(|(a, b)| a == b);
    let cond3 = !DISALLOWED.iter().any(|d| s.contains(d));

    cond1 && cond2 && cond3
}

pub fn is_nice2(s: &str) -> bool {
    let bytes = s.as_bytes();

    let cond1 = (0..bytes.len() - 3).any(|i| {
        let pair = &bytes[i..i + 2];
        s.as_bytes()[i + 2..].windows(2).any(|w| w == pair)
    });

    let cond2 = bytes.iter().tuple_windows().any(|(a, _, c)| a == c);

    cond1 && cond2
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.split_lines().filter(|s| is_nice1(s)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.split_lines().filter(|s| is_nice2(s)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
