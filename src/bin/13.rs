use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(13);

type Pair<'a> = (&'a str, &'a str);
type GuestList<'a> = HashSet<&'a str>;
type GuestAffinity<'a> = HashMap<Pair<'a>, i64>;

fn parse_guest_list(input: &str) -> (GuestList<'_>, GuestAffinity<'_>) {
    let mut guest_list = HashSet::<&str>::new();
    let mut affinities = GuestAffinity::new();

    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();

        let guest1 = split.next().unwrap();

        split.next(); // "would"

        let sign = match split.next().unwrap() {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!("Bad verb"),
        };

        let value = split.next().unwrap().parse::<i64>().unwrap() * sign;

        for _ in 0..6 {
            split.next(); // "happiness units by sitting next to"
        }

        let guest2 = split.next().unwrap().strip_suffix('.').unwrap();

        let affinity_ab = affinities.get(&(guest1, guest2));
        let affinity_ba = affinities.get(&(guest2, guest1));

        _ = match (affinity_ab, affinity_ba) {
            (None, None) => affinities.insert((guest1, guest2), value),
            (None, Some(ba)) => affinities.insert((guest2, guest1), ba + value),
            _ => None,
        };

        guest_list.insert(guest1);
    }

    (guest_list, affinities)
}

fn calculate_affinity_for_arrangement(
    guest: &str,
    guest_list: &GuestList<'_>,
    affinities: &GuestAffinity<'_>,
) -> i64 {
    let mut next_guest = guest;
    let mut seated_guests = Vec::with_capacity(guest_list.len());
    let mut total_affinity = 0;

    while seated_guests.len() < guest_list.len() {
        if let Some(next_best_pair) = affinities
            .iter()
            .filter(|((g1, g2), _)| {
                *g1 == next_guest && !seated_guests.contains(g2)
                    || (*g2 == next_guest && !seated_guests.contains(g1))
            })
            .k_largest_by_key(1, |(_, affinity)| *affinity)
            .next()
        {
            let ((g1, g2), affinity) = next_best_pair;
            if *g1 == next_guest {
                seated_guests.push(g1);
                next_guest = g2;
            } else {
                seated_guests.push(g2);
                next_guest = g1;
            }

            total_affinity += affinity;
        } else {
            break;
        }
    }

    total_affinity += affinities
        .get(&(guest, next_guest))
        .or_else(|| affinities.get(&(next_guest, guest)))
        .unwrap();

    total_affinity
}

pub fn part_one(input: &str) -> Option<i64> {
    let (guest_list, affinities) = parse_guest_list(input);

    Some(
        guest_list
            .iter()
            .map(|guest| calculate_affinity_for_arrangement(guest, &guest_list, &affinities))
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let (mut guest_list, mut affinities) = parse_guest_list(input);
    let me = "Me";

    guest_list.insert(me);

    for guest in &guest_list {
        affinities.insert((me, *guest), 0);
    }

    Some(
        guest_list
            .iter()
            .map(|guest| calculate_affinity_for_arrangement(guest, &guest_list, &affinities))
            .max()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(668));
    }
}
