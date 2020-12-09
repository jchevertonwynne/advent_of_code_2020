use std::time::Instant;

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

pub fn run() {
    let start = Instant::now();
    let trees = load_trees(INPUT);
    let data_loaded = Instant::now();
    let p1 = part1(&trees, 3, 1);
    let done_part1 = Instant::now();
    let p2 = part2(&trees);
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
