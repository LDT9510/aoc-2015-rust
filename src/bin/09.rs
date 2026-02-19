use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use advent_of_code::utils::parsing::UnwrapNextInt;

advent_of_code::solution!(9);

type City<'a> = &'a str;
type Path<'a> = (City<'a>, u64);
type Neighbors<'a> = HashSet<Path<'a>>;
type Map<'a> = HashMap<City<'a>, Neighbors<'a>>;

struct Graph<'a> {
    _map: Map<'a>,
}

#[derive(Copy, Clone)]
enum VisitOrder {
    ShortestToLongest,
    LongestToShortest,
}

impl<'a> Graph<'a> {
    fn parse(input: &'a str) -> Graph<'a> {
        let mut graph = Map::new();

        for line in input.lines() {
            let mut split = line.split(' ');
            let start_city = split.next().unwrap();
            split.next(); // skip "to"
            let end_city = split.next().unwrap();
            split.next(); // skip "="
            let distance = split.unwrap_next_int();

            Self::add_path(start_city, end_city, distance, &mut graph);
            Self::add_path(end_city, start_city, distance, &mut graph);
        }

        Graph { _map: graph }
    }

    fn add_path(from: City<'a>, to: City<'a>, dist: u64, map: &mut Map<'a>) {
        if let Some(neighbours) = map.get_mut(from) {
            neighbours.insert((to, dist));
        } else {
            let mut neighbours = HashSet::new();
            neighbours.insert((to, dist));
            map.insert(from, neighbours);
        }
    }

    fn solve(&self, order: VisitOrder) -> u64 {
        let candidate_distances = self
            ._map
            .keys()
            .map(|starting_city| self.visit(starting_city, &mut vec![], 0, order));

        match order {
            VisitOrder::ShortestToLongest => candidate_distances.min(),
            VisitOrder::LongestToShortest => candidate_distances.max(),
        }
        .unwrap()
    }

    fn visit(
        &self,
        city: City<'a>,
        visited: &mut Vec<City<'a>>,
        distance_acc: u64,
        order: VisitOrder,
    ) -> u64 {
        visited.push(city);

        if visited.len() == self._map.len() {
            return distance_acc;
        }

        let next_neighbours = self._map[city]
            .iter()
            .filter(|(neighbour_city, _)| !visited.contains(neighbour_city));

        let comparison_func =
            |(_, distance_a): &&Path, (_, distance_b): &&Path| distance_a.cmp(distance_b);

        let (next_city, distance_to_next_city) = match order {
            VisitOrder::ShortestToLongest => next_neighbours.k_smallest_by(1, comparison_func),
            VisitOrder::LongestToShortest => next_neighbours.k_largest_by(1, comparison_func),
        }
        .next()
        .unwrap();

        self.visit(
            next_city,
            visited,
            distance_acc + distance_to_next_city,
            order,
        )
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = Graph::parse(input);

    Some(graph.solve(VisitOrder::ShortestToLongest))
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = Graph::parse(input);

    Some(graph.solve(VisitOrder::LongestToShortest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(982));
    }
}
