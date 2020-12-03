use std::time::Instant;

type Trees = Vec<Vec<bool>>;

fn load_trees() -> Trees {
    let contents = std::fs::read_to_string("files/03.txt").expect("should be there");
    contents
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn part1(trees: &Trees, right: usize, down: usize) -> usize {
    let width = trees.first().expect("non zero entries").len();
    (0..trees.len())
        .step_by(down)
        .zip((0..).step_by(right).map(|v| v % width))
        .filter(|(y, x)| trees[*y][*x])
        .count()
}

fn part2(trees: &Trees) -> usize {
    let options: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    options
        .into_iter()
        .map(|(r, d)| part1(trees, r, d))
        .fold(1, |a, b| a * b)
}

pub fn run() {
    let start = Instant::now();
    let trees = load_trees();
    let data_loaded = Instant::now();
    let p1 = part1(&trees, 3, 1);
    let done_part1 = Instant::now();
    let p2 = part2(&trees);
    let done_part2 = Instant::now();

    println!("--------------------");
    println!("day 3");
    println!("    part 1: {}", p1);
    println!("    part 2: {}", p2);
    println!("time taken:");
    println!("    total: {:?}", done_part2.duration_since(start));
    println!("    data load: {:?}", data_loaded.duration_since(start));
    println!("    part 1: {:?}", done_part1.duration_since(data_loaded));
    println!("    part 2: {:?}", done_part2.duration_since(done_part1));
}
