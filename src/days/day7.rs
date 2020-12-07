use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};
use std::time::Instant;

const INPUT: &str = include_str!("../../files/07.txt");

struct BagTree<'a> {
    nodes: HashMap<&'a str, Rc<Bag<'a>>>,
}

struct Bag<'a> {
    colour: &'a str,
    parents: RefCell<Vec<Weak<Bag<'a>>>>,
    children: RefCell<Vec<ChildBagInfo<'a>>>,
}

struct ChildBagInfo<'a> {
    count: usize,
    child: Weak<Bag<'a>>,
}

impl BagTree<'_> {
    fn new(input: &str) -> BagTree {
        let first_colours: Vec<(&str, &str)> = input
            .lines()
            .map(|rule| {
                let mut parts = rule.split(" bags contain ");
                (parts.next().expect("pls"), parts.next().expect("pls"))
            })
            .collect::<Vec<_>>();

        let nodes: HashMap<&str, Rc<Bag>> = first_colours
            .iter()
            .map(|&(colour, _)| {
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

        for (colour, children) in first_colours {
            if children == "no other bags." {
                continue;
            }

            let parent = nodes.get(colour).expect("should be found");

            let children = children
                .split(',')
                .map(|line| {
                    let line = line.trim();
                    let line = match line.strip_suffix('.') {
                        Some(line) => line,
                        None => line
                    };

                    let count_ind = line.chars().take_while(|c| ('0'..='9').contains(c)).count();
                    let count = line[..count_ind].parse::<usize>().expect("parse pls");

                    let colour = if count == 1 {
                        &line[count_ind + 1..line.len() - 4]
                    } else {
                        &line[count_ind + 1..line.len() - 5]
                    };

                    let child = nodes.get(colour).expect("should be put in");
                    child.parents.borrow_mut().push(Rc::downgrade(parent));

                    ChildBagInfo {
                        count,
                        child: Rc::downgrade(child),
                    }
                });

            nodes
                .get(colour)
                .expect("should be put in")
                .children
                .borrow_mut()
                .extend(children);
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

    fn children(&self) -> usize {
        self.children
            .borrow()
            .iter()
            .map(|c| c.count + (c.count * c.child.upgrade().expect("pls upgrade").children()))
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
        .children()
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
