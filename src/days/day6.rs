use std::time::Instant;
use std::ops::{BitOr, BitAnd};

type GroupResults = Vec<usize>;

fn load_groups() -> Vec<GroupResults> {
    include_str!("../../files/06.txt")
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| {
                    line.chars()
                        .fold(0, |acc, c| acc | (1 << (c as usize - 'a' as usize)))
                })
                .collect()
        })
        .collect()
}

fn part1(groups: &Vec<GroupResults>) -> usize {
    groups
        .iter()
        .map(|group| group.iter().fold(0, |acc, v| acc.bitor(v)).count_ones() as usize)
        .sum()
}

fn part2(groups: &Vec<GroupResults>) -> usize {
    groups
        .iter()
        .map(|group| group.iter().fold(group[0], |acc, v| acc.bitand(v)).count_ones() as usize)
        .sum()
}

pub fn run() {
    let start = Instant::now();
    let groups = load_groups();
    let data_loaded = Instant::now();
    let p1 = part1(&groups);
    let done_part1 = Instant::now();
    let p2 = part2(&groups);
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
        let groups = load_groups();
        assert_eq!(part1(&groups), 6297);
    }

    #[test]
    fn test_part2() {
        let groups = load_groups();
        assert_eq!(part2(&groups), 3158);
    }
}
