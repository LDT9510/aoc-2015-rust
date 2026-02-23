use advent_of_code::utils::parsing::IterInts;

advent_of_code::solution!(16);

#[derive(Copy, Clone)]
enum Compound {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl Compound {
    fn from_name(text: &str) -> Option<Compound> {
        use Compound::*;
        match text {
            "children" => Some(Children),
            "cats" => Some(Cats),
            "samoyeds" => Some(Samoyeds),
            "pomeranians" => Some(Pomeranians),
            "akitas" => Some(Akitas),
            "vizslas" => Some(Vizslas),
            "goldfish" => Some(Goldfish),
            "trees" => Some(Trees),
            "cars" => Some(Cars),
            "perfumes" => Some(Perfumes),
            _ => None,
        }
    }
}

const MESSAGE: [i64; 10] = {
    use Compound::*;
    let mut msg = [0; 10];

    msg[Children as usize] = 3;
    msg[Cats as usize] = 7;
    msg[Samoyeds as usize] = 2;
    msg[Pomeranians as usize] = 3;
    msg[Akitas as usize] = 0;
    msg[Vizslas as usize] = 0;
    msg[Goldfish as usize] = 5;
    msg[Trees as usize] = 3;
    msg[Cars as usize] = 2;
    msg[Perfumes as usize] = 1;

    msg
};

pub fn part_one(input: &str) -> Option<i64> {
    for line in input.lines() {
        let mut it = line.iter_named_ints::<i64>();
        let mut matches = 0;
        if let Some((_, sue_num)) = it.next() {
            for (compound_name, amount) in it {
                if let Some(compound) = Compound::from_name(compound_name)
                    && MESSAGE[compound as usize] == amount
                {
                    matches += 1
                }
            }

            if matches == 3 {
                return Some(sue_num);
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<i64> {
    for line in input.lines() {
        let mut it = line.iter_named_ints::<i64>();
        let mut matches = 0;
        if let Some((_, sue_num)) = it.next() {
            for (compound_name, amount) in it {
                if let Some(compound) = Compound::from_name(compound_name) {
                    let message_amount = MESSAGE[compound as usize];
                    use Compound::*;
                    matches += match compound {
                        Cats | Trees => amount > message_amount,
                        Pomeranians | Goldfish => amount < message_amount,
                        _ => amount == message_amount,
                    } as i32
                }
            }

            if matches == 3 {
                return Some(sue_num);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
