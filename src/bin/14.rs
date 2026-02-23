use advent_of_code::utils::parsing::UnwrapNextInt;
use std::env;

advent_of_code::solution!(14);

// this is a workaround due to this framework not having a way to pass different parameters
// to tests and normal input solutions
fn get_after_value_workaround() -> i64 {
    if let Ok(var_value) = env::var("AFTER") {
        var_value.parse().unwrap()
    } else {
        2503
    }
}

struct Deer {
    speed: i64,
    stamina: i64,
    rest: i64,
}

impl Deer {
    fn distance_after(&self, seconds: i64) -> i64 {
        let burst_len = self.stamina + self.rest;
        let bursts = seconds as f64 / burst_len as f64;
        let complete_burst_dist = bursts as i64 * self.speed * self.stamina;
        let stamina_left = (bursts.fract() * burst_len as f64).min(self.stamina as f64) as i64;
        let partial_burst_dist = stamina_left * self.speed;

        complete_burst_dist + partial_burst_dist
    }
}

fn parse_deer_list(input: &str) -> Vec<Deer> {
    let mut deer_list = Vec::new();

    for line in input.lines() {
        let split = line.split_ascii_whitespace();

        // <name> can fly
        let mut split = split.skip(3);

        let speed = split.unwrap_next_int();

        // km/s for
        let mut split = split.skip(2);

        let stamina = split.unwrap_next_int();

        // seconds, but then must rest for
        let mut split = split.skip(6);

        let rest = split.unwrap_next_int();

        deer_list.push(Deer {
            speed,
            rest,
            stamina,
        })
    }

    deer_list
}

pub fn part_one(input: &str) -> Option<i64> {
    let after = get_after_value_workaround();

    let deer_list = parse_deer_list(input);

    Some(
        deer_list
            .into_iter()
            .map(|deer| deer.distance_after(after))
            .max()
            .unwrap(),
    )
}

enum DeerStatus {
    WithStamina(i64),
    Exhausted(i64),
}

pub fn part_two(input: &str) -> Option<i64> {
    let after = get_after_value_workaround();
    
    let deer_list = parse_deer_list(input);
    let mut deer_statuses: Vec<DeerStatus> = deer_list
        .iter()
        .map(|deer| DeerStatus::WithStamina(deer.stamina))
        .collect();
    let mut points = vec![0; deer_list.len()];
    let mut distances = vec![0; deer_list.len()];
    let mut current_max_dist = 0;

    for _ in 0..after {
        for (i, deer) in deer_list.iter().enumerate() {
            let deer_status = &deer_statuses[i];

            if let DeerStatus::WithStamina(_) = deer_status {
                distances[i] += deer.speed;
                current_max_dist = current_max_dist.max(distances[i]);
            }

            deer_statuses[i] = match deer_status {
                DeerStatus::WithStamina(1) => DeerStatus::Exhausted(deer.rest),
                DeerStatus::Exhausted(1) => DeerStatus::WithStamina(deer.stamina),
                DeerStatus::WithStamina(stamina) => DeerStatus::WithStamina(stamina - 1),
                DeerStatus::Exhausted(rest) => DeerStatus::Exhausted(rest - 1),
            }
        }

        for (i, d) in distances.iter().enumerate() {
            if *d == current_max_dist {
                points[i] += 1
            }
        }
    }

    Some(*points.iter().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        unsafe {
            env::set_var("AFTER", "1000");
        };
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1120));
    }

    #[test]
    fn test_part_two() {
        unsafe {
            env::set_var("AFTER", "1000");
        };
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(689));
    }
}
