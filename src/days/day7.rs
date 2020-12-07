use regex::Regex;
use smallvec::alloc::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::time::Instant;

const INPUT: &str = include_str!("../../files/07.txt");

#[derive(Hash, Eq, PartialEq, Debug)]
struct Rule<'a> {
    colour: &'a str,
    count: usize,
}

type RuleSet = HashMap<&'static str, HashSet<Rule<'static>>>;

fn load_rules() -> RuleSet {
    let ends_extractor = Regex::new("^(.*) bags contain (.*).$").expect("valid regex");
    let colour_matcher = Regex::new("^(\\d+) (.*) bags?$").expect("shoudl be valid regex");

    INPUT
        .lines()
        .map(|line| {
            let parts = ends_extractor
                .captures(line)
                .unwrap_or_else(|| panic!("valid for input {}", line));
            let colour = parts.get(1).expect("should exist").as_str();
            let matches_with = parts.get(2).expect("should exist").as_str();
            if matches_with == "no other bags" {
                return (colour, HashSet::new());
            }

            let mut paired_with = HashSet::new();

            for colour_string in matches_with.split(',') {
                let colour_string = colour_string.trim();
                let colour_info = colour_matcher
                    .captures(colour_string)
                    .unwrap_or_else(|| panic!("not valid colour string: {}", colour_string));
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
                paired_with.insert(Rule {
                    colour: required_colour,
                    count,
                });
            }

            (colour, paired_with)
        })
        .collect()
}

fn part1(rules: &RuleSet) -> usize {
    let mut inverse = HashMap::new();
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
        match inverse.get(colour) {
            Some(opts) => {
                for opt in opts {
                    if seen.insert(opt) {
                        res += 1;
                        stack.push_back(opt);
                    }
                }
            }
            None => continue,
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
    let rules = load_rules();
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
