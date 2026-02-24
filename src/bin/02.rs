use advent_of_code::utils::parsing::IterInts;
use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|str_dimensions| str_dimensions.iter_ints::<u64>().collect_tuple())
            .map(|(l, w, h)| {
                let sides = [l * w, w * h, h * l];
                sides.iter().map(|s| s * 2).sum::<u64>() + sides.iter().min().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|str_dimensions| str_dimensions.iter_ints::<u64>().collect_tuple())
            .map(|(l, w, h)| (l + l + w + w) + (l * w * h))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(101));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
