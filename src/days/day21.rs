use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/21.txt");

#[derive(Debug, Clone)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

fn load_foods(input: &str) -> Vec<Food> {
    let mut res = Vec::new();
    for line in input.lines() {
        let end = line.find("contains").expect("always has allergens");

        let ingredients = line[..end - 2].split(' ').collect::<HashSet<_>>();
        let allergens = line[end + 9..line.len() - 1]
            .split(", ")
            .collect::<HashSet<_>>();
        res.push(Food {
            ingredients,
            allergens,
        });
    }
    res
}

fn solve(foods: Vec<Food>) -> (usize, String) {
    let mut known_allergen: HashMap<&str, &str> = HashMap::new();

    let mut working_foods = foods.to_vec();

    while working_foods.iter().any(|f| !f.allergens.is_empty()) {
        let singular_allergens = working_foods
            .iter()
            .filter(|food| food.allergens.len() == 1)
            .map(|f| *f.allergens.iter().next().unwrap())
            .collect::<HashSet<_>>();

        let mut solved = Vec::new();

        for allergen in singular_allergens {
            let containing = working_foods
                .iter()
                .filter(|f| f.allergens.contains(&allergen))
                .collect::<Vec<_>>();
            let possible_ingredients =
                containing
                    .iter()
                    .fold(containing[0].ingredients.clone(), |acc, food| {
                        HashSet::intersection(&acc, &food.ingredients)
                            .copied()
                            .collect()
                    });
            if possible_ingredients.len() != 1 {
                continue;
            }
            let ingredient = *possible_ingredients.iter().next().unwrap();
            solved.push((allergen, ingredient));
        }

        for (allergen, ingredient) in solved {
            known_allergen.insert(ingredient, allergen);
            for food in working_foods.iter_mut() {
                food.ingredients.remove(ingredient);
                food.allergens.remove(allergen);
            }
        }
    }

    let p1 = foods
        .iter()
        .map(|f| {
            f.ingredients
                .iter()
                .filter(|i| !known_allergen.contains_key(*i))
                .count()
        })
        .sum();

    let mut reversed = known_allergen
        .iter_mut()
        .map(|(k, v)| (v, k))
        .collect::<Vec<_>>();
    reversed.sort_by(|(aa, _), (ba, _)| aa.cmp(ba));
    let p2 = reversed.iter().map(|r| *r.1).collect::<Vec<_>>().join(",");

    (p1, p2)
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let foods = load_foods(INPUT);
    let (p1, p2) = solve(foods);

    (p1.to_string(), p2, start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day21::{load_foods, solve};

    #[test]
    fn test_part1() {
        let s = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let foods = load_foods(s);
        assert_eq!(solve(foods), (5, "mxmxvkd,sqjhc,fvjkl".to_string()));
    }
}
