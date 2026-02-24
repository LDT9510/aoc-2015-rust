use advent_of_code::utils::parsing::IterInts;

advent_of_code::solution!(12);

//solution for part_two using serde_json, simpler but slower (about 5x slower in --release)
// fn sum_json(value: &Value) -> i64 {
//     match value {
//         Value::Number(n) => n.as_i64().unwrap_or(0),
//         Value::Array(arr) => arr.iter().map(sum_json).sum(),
//         Value::Object(map) => {
//             // If any property value is the string "red", ignore the entire object
//             if map
//                 .values()
//                 .any(|v| matches!(v, Value::String(s) if s == "red"))
//             {
//                 0
//             } else {
//                 map.values().map(sum_json).sum()
//             }
//         }
//         _ => 0, // strings (except checked above), bools, null -> 0
//     }
// }

pub fn part_one(input: &str) -> Option<i64> {
    Some(input.iter_ints::<i64>().sum())
}

struct RedFinder {
    buf: [char; 6],
    idx: usize,
}

impl RedFinder {
    pub fn new() -> Self {
        RedFinder {
            buf: [0 as char; 6],
            idx: 0,
        }
    }
    pub fn process_char(&mut self, c: char) -> bool {
        match self.idx {
            0 if c == ':' => self.assign_and_advance(c),
            1 if c == '"' => self.assign_and_advance(c),
            2 if c == 'r' => self.assign_and_advance(c),
            3 if c == 'e' => self.assign_and_advance(c),
            4 if c == 'd' => self.assign_and_advance(c),
            5 if c == '"' => {
                self.reset();
                return true;
            }
            _ => self.reset(),
        }

        false
    }

    fn assign_and_advance(&mut self, c: char) {
        self.buf[self.idx] = c;
        self.idx += 1
    }

    fn reset(&mut self) {
        self.idx = 0
    }
}

enum ObjectContext {
    Root,
    Child(usize),
    Out,
}

enum BadObject {
    None,
    Found(usize),
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut result = 0;
    let mut buf = String::with_capacity(10);
    let mut oc = ObjectContext::Out;
    let mut in_object_sums = [0; 100];
    let mut red_finder = RedFinder::new();
    let mut object_contrib = 0;
    let mut bad_object = BadObject::None;

    for c in input.chars() {
        if c == '{' {
            oc = match oc {
                ObjectContext::Root => {
                    in_object_sums[0] += object_contrib;
                    ObjectContext::Child(1)
                }
                ObjectContext::Child(depth) => {
                    in_object_sums[depth] += object_contrib;
                    ObjectContext::Child(depth + 1)
                }
                ObjectContext::Out => ObjectContext::Root,
            };

            object_contrib = 0;
            continue;
        }

        match c {
            '-' => buf.push(c),
            '0'..='9' => buf.push(c),
            _ => {
                if matches!(bad_object, BadObject::None) && red_finder.process_char(c) {
                    bad_object = match oc {
                        ObjectContext::Root => BadObject::Found(0),
                        ObjectContext::Child(depth) => BadObject::Found(depth),
                        _ => unreachable!("Bad object found in non-object context"),
                    };
                    object_contrib = 0;
                    buf.clear();
                    continue;
                }

                if !buf.is_empty()
                    && let Ok(num) = buf.parse::<i64>()
                {
                    match oc {
                        ObjectContext::Out => result += num,
                        _ => object_contrib += num,
                    }

                    buf.clear();
                }
            }
        }

        if c == '}' {
            oc = match oc {
                ObjectContext::Root => {
                    if let BadObject::None = bad_object {
                        result += in_object_sums[0] + object_contrib;
                    } else {
                        bad_object = BadObject::None;
                    }

                    in_object_sums[0] = 0;

                    ObjectContext::Out
                }
                ObjectContext::Child(depth) => {
                    if let BadObject::Found(bad_object_depth) = bad_object {
                        if bad_object_depth == depth {
                            bad_object = BadObject::None;
                        }
                    } else {
                        in_object_sums[depth - 1] += object_contrib + in_object_sums[depth];
                    }

                    in_object_sums[depth] = 0;

                    if depth == 1 {
                        ObjectContext::Root
                    } else {
                        ObjectContext::Child(depth - 1)
                    }
                }
                _ => panic!("Open bracket not found, bad input."),
            };

            object_contrib = 0;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(16));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(40));
    }
}
