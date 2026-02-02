use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    pub x: i32,
    pub y: i32,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut location = Coord { x: 0, y: 0 };
    let mut set = HashSet::new();
    set.insert(location);
    let mut count = 1;

    for c in input.chars() {
        match c {
            '^' => location.y += 1,
            'v' => location.y -= 1,
            '>' => location.x += 1,
            '<' => location.x -= 1,
            _ => {}
        }

        count += set.insert(location) as usize;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut location_santa = Coord { x: 0, y: 0 };
    let mut location_robot_santa = Coord { x: 0, y: 0 };
    let mut set = HashSet::new();
    set.insert(location_santa);
    let mut count = 1;
    let mut santa_turn = true;

    for c in input.chars() {
        let location = if santa_turn {
            &mut location_santa
        } else {
            &mut location_robot_santa
        };

        match c {
            '^' => location.y += 1,
            'v' => location.y -= 1,
            '>' => location.x += 1,
            '<' => location.x -= 1,
            _ => {}
        }

        count += set.insert(*location) as usize;

        santa_turn = !santa_turn;
    }

    Some(count)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }
}
