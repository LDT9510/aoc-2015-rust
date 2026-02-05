use advent_of_code::utils::coord_2d::Coord;
use advent_of_code::utils::geom::PositionalRectangle;
use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::string_utils::StrExtensionsUtils;
use std::str::FromStr;

advent_of_code::solution!(6);

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
    Unknown,
}

type BigBoolGrid = Grid<bool, 1_000_000>;
type BigIntGrid = Grid<i32, 1_000_000>;

fn parse_action_and_rectangle(instruction: &str) -> (Action, PositionalRectangle) {
    let mut element = instruction.split(' ');

    // <<turn on>> 0,0 through 999,999
    let action = match element.next() {
        Some("turn") => match element.next() {
            Some("on") => Action::TurnOn,
            Some("off") => Action::TurnOff,
            _ => Action::Unknown,
        },
        Some("toggle") => Action::Toggle,
        _ => Action::Unknown,
    };

    // turn on <<0,0>> through 999,999
    let start_coord = Coord::from_str(element.next().unwrap()).unwrap();

    // turn on 0,0 <<through>> 999,999
    element.next();

    // turn on 0,0 through <<999,999>>
    let end_coord = Coord::from_str(element.next().unwrap()).unwrap();

    let rect = PositionalRectangle::new(start_coord, end_coord);

    (action, rect)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = BigBoolGrid::new_boxed();

    for instruction in input.split_lines() {
        let (action, rect) = parse_action_and_rectangle(instruction);

        match action {
            Action::TurnOn => {
                grid.fill_in_rectangle(rect, true);
            }
            Action::TurnOff => {
                grid.fill_in_rectangle(rect, false);
            }
            Action::Toggle => {
                grid.toggle_in_rectangle(rect);
            }
            Action::Unknown => {
                unreachable!("Bad input")
            }
        }
    }

    Some(grid.count_on())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = BigIntGrid::new_boxed();

    for instruction in input.split_lines() {
        let (action, rect) = parse_action_and_rectangle(instruction);

        match action {
            Action::TurnOn => grid.apply_in_rectangle(rect, |light| *light += 1),
            Action::TurnOff => grid.apply_in_rectangle(rect, |light| {
                if *light > 0 {
                    *light -= 1
                }
            }),
            Action::Toggle => grid.apply_in_rectangle(rect, |light| *light += 2),
            Action::Unknown => {
                unreachable!("Bad input")
            }
        }
    }

    Some(grid.iter().map(|it| it.value as usize).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(998_996));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2_000_001));
    }
}
