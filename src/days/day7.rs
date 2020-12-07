use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};
use std::time::Instant;

const INPUT: &str = include_str!("../../files/07.txt");

#[derive(Debug)]
struct BagTree<'a> {
    nodes: HashMap<&'a str, Rc<Bag<'a>>>,
}

#[derive(Debug)]
struct Bag<'a> {
    colour: &'a str,
    parents: RefCell<Vec<Weak<Bag<'a>>>>,
    children: RefCell<Vec<ChildBagInfo<'a>>>,
}

#[derive(Debug)]
struct ChildBagInfo<'a> {
    count: usize,
    child: Weak<Bag<'a>>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Rule<'a> {
    colour: &'a str,
    count: usize,
}

impl BagTree<'_> {
    fn new(input: &str) -> BagTree {
        let ends_extractor =
            Regex::new("^(.*) bags contain (.*).$").expect("should be valid regex");
        let colour_matcher = Regex::new("^(\\d+) (.*) bags?$").expect("should be valid regex");

        let first_colours = input
            .lines()
            .map(|rule| ends_extractor.captures(rule).expect("should match"))
            .collect::<Vec<_>>();

        let nodes: HashMap<&str, Rc<Bag>> = first_colours
            .iter()
            .map(|colour_and_children| {
                let colour = colour_and_children
                    .get(1)
                    .expect("should be found")
                    .as_str();
                (
                    colour,
                    Rc::new(Bag {
                        colour,
                        parents: RefCell::new(vec![]),
                        children: RefCell::new(vec![]),
                    }),
                )
            })
            .collect();

        for colour_and_children in first_colours {
            let children = colour_and_children
                .get(2)
                .expect("shoud have found children")
                .as_str();

            if children == "no other bags" {
                continue;
            }

            let children: Vec<(usize, &str)> = children
                .split(',')
                .map(|line| {
                    let line = line.trim();
                    let colour_info = colour_matcher.captures(line).expect("should match");
                    let count = colour_info
                        .get(1)
                        .expect("should contain count")
                        .as_str()
                        .parse::<usize>()
                        .expect("should be valid number");
                    let colour = colour_info.get(2).expect("should contain colour").as_str();
                    (count, colour)
                })
                .collect();

            let colour = colour_and_children
                .get(1)
                .expect("should be found")
                .as_str();
            let parent = nodes.get(colour).expect("should be found");

            for (_, child) in &children {
                let entry = nodes.get(child).expect("should be put in");
                entry.parents.borrow_mut().push(Rc::downgrade(parent));
            }

            let children_info = children
                .iter()
                .map(|(count, child)| {
                    let child = nodes.get(child).expect("should be put in");
                    ChildBagInfo {
                        count: *count,
                        child: Rc::downgrade(child),
                    }
                })
                .collect();
            nodes
                .get(colour)
                .expect("should be put in")
                .children
                .replace(children_info);
        }

        BagTree { nodes }
    }
}

impl<'c> Bag<'c> {
    fn parents(&self) -> usize {
        self.parents_helper(&mut HashSet::new())
    }

    fn parents_helper<'a, 'b>(&'a self, seen: &'b mut HashSet<&'c str>) -> usize {
        self.parents
            .borrow()
            .iter()
            .filter_map(|parent| {
                let parent = parent.upgrade().expect("pls");
                if seen.insert(parent.colour) {
                    Some(1 + parent.parents_helper(seen))
                } else {
                    None
                }
            })
            .sum()
    }

    fn contains(&self) -> usize {
        self.children
            .borrow()
            .iter()
            .map(|c| c.count + (c.count * c.child.upgrade().expect("pls upgrade").contains()))
            .sum::<usize>()
    }
}

fn part1(tree: &BagTree) -> usize {
    tree.nodes
        .get("shiny gold")
        .expect("should exist")
        .parents()
}

fn part2(tree: &BagTree) -> usize {
    tree.nodes
        .get("shiny gold")
        .expect("should exist")
        .contains()
}

pub fn run() {
    let start = Instant::now();
    let tree = BagTree::new(INPUT);
    let data_loaded = Instant::now();
    let p1 = part1(&tree);
    let done_part1 = Instant::now();
    let p2 = part2(&tree);
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
        let tree = BagTree::new(INPUT);
        assert_eq!(part1(&tree), 332);
    }

    #[test]
    fn test_part2() {
        let tree = BagTree::new(INPUT);
        assert_eq!(part2(&tree), 10875);
    }
}
