advent_of_code::solution!(10);

fn look_and_say(input: &str) -> String {
    let mut output = String::new();
    let mut counter = 1;

    fn close_group(b: u8, o: &mut String, cnt: &mut i32) {
        o.push_str(&cnt.to_string());
        o.push(b as char);
        *cnt = 1;
    }

    let bytes = input.as_bytes();

    for i in 0..bytes.len() {
        let b = bytes[i];

        if i + 1 < bytes.len() {
            let next_b = bytes[i + 1];

            if b == next_b {
                counter += 1;
            } else {
                close_group(b, &mut output, &mut counter);
            }
        } else {
            close_group(b, &mut output, &mut counter);
        }
    }

    output
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut output = input.to_string();

    for _ in 0..40 {
        output = look_and_say(&output);
    }

    Some(output.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut output = input.to_string();

    for _ in 0..50 {
        output = look_and_say(&output);
    }

    Some(output.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(237746));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3369156));
    }
}
