use advent_of_code::utils::parsing::IterInts;
use itertools::Itertools;

advent_of_code::solution!(15);

struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn from_str(text: &str) -> Option<Ingredient> {
        text.iter_ints()
            .collect_tuple()
            .map(
                |(capacity, durability, flavor, texture, calories)| Ingredient {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                },
            )
    }
}

fn parse_ingredients(input: &str) -> Vec<Ingredient> {
    let mut ingredients = Vec::new();

    for line in input.lines() {
        ingredients.push(Ingredient::from_str(line).unwrap())
    }

    ingredients
}

fn calculate_total_score(
    ingredients: &[Ingredient],
    teaspoons: &[i64],
    target_calories: Option<i64>,
) -> i64 {
    let (capacity_score, durability_score, texture_score, flavor_score, total_calories) =
        ingredients
            .iter()
            .zip(teaspoons)
            .fold((0, 0, 0, 0, 0), |(c, d, t, f, ca), (ig, &ts)| {
                (
                    c + ig.capacity * ts,
                    d + ig.durability * ts,
                    t + ig.texture * ts,
                    f + ig.flavor * ts,
                    ca + ig.calories * ts,
                )
            });

    if let Some(calories) = target_calories
        && total_calories != calories
    {
        return 0;
    }

    capacity_score.max(0) * durability_score.max(0) * texture_score.max(0) * flavor_score.max(0)
}

fn solve(ingredients: &[Ingredient], target_calories: Option<i64>) -> Option<i64> {
    const TOTAL_INGREDIENTS: i64 = 100;

    (0..ingredients.len())
        .map(|_| 0..TOTAL_INGREDIENTS)
        .multi_cartesian_product()
        .filter(|teaspoons| teaspoons.iter().sum::<i64>() == TOTAL_INGREDIENTS)
        .map(|teaspoons| calculate_total_score(ingredients, &teaspoons, target_calories))
        .max()
}

pub fn part_one(input: &str) -> Option<i64> {
    let ingredients = parse_ingredients(input);

    solve(&ingredients, None)
}
pub fn part_two(input: &str) -> Option<i64> {
    let ingredients = parse_ingredients(input);

    solve(&ingredients, Some(500))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62_842_880));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(57_600_000));
    }
}
