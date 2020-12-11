use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/03.txt");

struct Trees {
    width: usize,
    rows: Vec<usize>,
}

fn load_trees(input: &str) -> Trees {
    let mut width = 0;
    let rows = input
        .lines()
        .map(|line| {
            width = line.len();
            line.chars()
                .zip(0..)
                .filter_map(|(c, i)| if c == '#' { Some(i) } else { None })
                .fold(0, |acc, v| acc | (1 << v))
        })
        .collect();
    Trees { width, rows }
}

fn part1(trees: &Trees, right: usize, down: usize) -> usize {
    (0..trees.rows.len() / down)
        .map(|i| i * down)
        .zip((0..).map(|x| (x * right) % trees.width))
        .filter(|&(y, x)| (trees.rows[y] & (1 << x)) != 0)
        .count()
}

fn part2(trees: &Trees) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(right, down)| part1(trees, right, down))
        .product()
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let trees = load_trees(INPUT);
    let p1 = part1(&trees, 3, 1);
    let p2 = part2(&trees);
    let done = Instant::now();

    (p1, p2, done - start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let trees = load_trees(INPUT);
        assert_eq!(part1(&trees, 3, 1), 205)
    }

    #[test]
    fn part2_test() {
        let trees = load_trees(INPUT);
        assert_eq!(part2(&trees), 3952146825)
    }
}
