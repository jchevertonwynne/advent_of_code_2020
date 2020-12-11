use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ptr::slice_from_raw_parts;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

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

unsafe fn combine_name_parts<'a>(a: &'a str, b: &'a str) -> &'a str {
    let s = slice_from_raw_parts(a.as_ptr(), a.len() + b.len() + 1);
    std::str::from_utf8_unchecked(&*s)
}

unsafe fn get_rest_of_string<'a>(full: &'a str, start: &'a str) -> &'a str {
    let full_p = full.as_ptr();
    let base_p = start.as_ptr();
    let seen = base_p as usize - full_p as usize;
    let s = slice_from_raw_parts(base_p, full.len() - seen - 1);
    std::str::from_utf8_unchecked(&*s)
}

impl BagTree<'_> {
    fn new(input: &str) -> BagTree {
        let colour_and_children: Vec<(&str, &str)> = input
            .lines()
            .map(|rule| {
                let mut parts = rule.split(' ');
                let quality = parts.next().expect("be a thing");
                let colour = parts.next().expect("be a thing");
                let colour = unsafe { combine_name_parts(quality, colour) };
                parts.next();
                parts.next();
                let start = parts.next().expect("be a thing");
                let res = unsafe { get_rest_of_string(rule, start) };
                (colour, res)
            })
            .collect();

        let mut nodes = HashMap::with_capacity(colour_and_children.len());
        colour_and_children.iter().for_each(|&(colour, _)| {
            nodes.insert(
                colour,
                Rc::new(Bag {
                    colour,
                    parents: RefCell::new(Vec::new()),
                    children: RefCell::new(Vec::new()),
                }),
            );
        });

        colour_and_children
            .into_iter()
            .for_each(|(colour, children)| {
                if children == "no other bags" {
                    return;
                }

                let parent = nodes.get(colour).expect("should be found");

                let children = children.split(", ").map(|line| {
                    let mut words = line.split(' ');
                    let count = words
                        .next()
                        .expect("be a thing")
                        .parse::<usize>()
                        .expect("parse pls");
                    let quality = words.next().expect("be a thing");
                    let colour = words.next().expect("be a thing");
                    let colour = unsafe { combine_name_parts(quality, colour) };
                    let child = nodes.get(colour).expect("should be put in");

                    child.parents.borrow_mut().push(Rc::downgrade(parent));

                    ChildBagInfo {
                        count,
                        child: Rc::downgrade(child),
                    }
                });

                parent.children.borrow_mut().extend(children);
            });

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

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let tree = BagTree::new(INPUT);
    let p1 = part1(&tree);
    let p2 = part2(&tree);
    let done = Instant::now();

    (p1, p2, done - start)
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
