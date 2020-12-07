use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

const INPUT: &str = include_str!("../../files/07.txt");

#[derive(Hash, Eq, PartialEq, Debug)]
struct Rule<'a> {
    colour: &'a str,
    count: usize,
}

type RuleSet<'a> = HashMap<&'a str, HashSet<Rule<'a>>>;

fn load_rules<'a>(input: &'a str) -> RuleSet<'a> {
    let ends_extractor = Regex::new("^(.*) bags contain (.*).$").expect("should be valid regex");
    let colour_matcher = Regex::new("^(\\d+) (.*) bags?$").expect("should be valid regex");

    input
        .lines()
        .map(|line| {
            let parts = ends_extractor
                .captures(line)
                .unwrap_or_else(|| panic!("valid for input {}", line));
            let colour = parts.get(1).expect("should exist").as_str();
            let contained_bags = parts.get(2).expect("should exist").as_str();

            let paired_with = match contained_bags {
                "no other bags" => HashSet::new(),
                _ => contained_bags
                    .split(',')
                    .map(|colour_string| {
                        let colour_string = colour_string.trim();
                        let colour_info =
                            colour_matcher.captures(colour_string).unwrap_or_else(|| {
                                panic!("not valid colour string: {}", colour_string)
                            });
                        let count = colour_info
                            .get(1)
                            .unwrap_or_else(|| panic!("should have count"))
                            .as_str()
                            .parse()
                            .unwrap_or_else(|_| panic!("should be a valid integer"));
                        let required_colour = colour_info
                            .get(2)
                            .unwrap_or_else(|| panic!("should have required colour"))
                            .as_str();

                        Rule {
                            colour: required_colour,
                            count,
                        }
                    })
                    .collect(),
            };

            (colour, paired_with)
        })
        .collect()
}

fn part1(rules: &RuleSet) -> usize {
    let mut inverse: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (colour, to_colours) in rules {
        for rule in to_colours {
            inverse
                .entry(rule.colour)
                .or_insert_with(HashSet::new)
                .insert(colour);
        }
    }

    let mut res = 0;
    let mut seen: HashSet<&str> = HashSet::new();
    let mut stack: VecDeque<&str> = VecDeque::from(vec!["shiny gold"]);

    while let Some(colour) = stack.pop_front() {
        if let Some(opts) = inverse.get(colour) {
            for opt in opts {
                if seen.insert(opt) {
                    res += 1;
                    stack.push_back(opt);
                }
            }
        }
    }

    res
}

fn bags_inside(rules: &RuleSet, colour: &str) -> usize {
    let inside = rules.get(colour).expect("pls exist");

    inside
        .iter()
        .map(|inside| inside.count + (inside.count * bags_inside(rules, inside.colour)))
        .sum()
}

fn part2(rules: &RuleSet) -> usize {
    bags_inside(rules, "shiny gold")
}

pub fn run() {
    let start = Instant::now();
    let rules = load_rules(INPUT);
    let data_loaded = Instant::now();
    let p1 = part1(&rules);
    let done_part1 = Instant::now();
    let p2 = part2(&rules);
    let done_part2 = Instant::now();

    println!("    part 1: {}", p1);
    println!("    part 2: {}", p2);
    println!("time taken:");
    println!("    total: {:?}", done_part2.duration_since(start));
    println!("    data load: {:?}", data_loaded.duration_since(start));
    println!("    part 1: {:?}", done_part1.duration_since(data_loaded));
    println!("    part 2: {:?}", done_part2.duration_since(done_part1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let rules = load_rules(INPUT);
        assert_eq!(part1(&rules), 332);
    }

    #[test]
    fn test_part2() {
        let rules = load_rules(INPUT);
        assert_eq!(part2(&rules), 10875);
    }
}
