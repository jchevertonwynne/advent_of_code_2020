use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

const GOAL: i64 = 2020;

fn load_numbers() -> Vec<i64> {
    std::fs::read_to_string("files/01.txt")
        .expect("should be file")
        .trim()
        .lines()
        .map(|i| i.parse().expect("should be valid int"))
        .collect()
}

fn part1(nums: &Vec<i64>) -> i64 {
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut t = nums[i] + nums[j];

    loop {
        match t.cmp(&GOAL) {
            Ordering::Less => i += 1,
            Ordering::Equal => return nums[i] * nums[j],
            Ordering::Greater => j -= 1,
        }
        t = nums[i] + nums[j];
    }
}

fn part2(nums: &Vec<i64>) -> i64 {
    let min = nums.first().expect("more than 0 items");
    let cache: HashMap<i64, (i64, i64)> = (0..nums.len())
        .flat_map(|i| {
            ((i + 1)..nums.len()).filter_map(move |j| {
                let i = nums[i];
                let j = nums[j];
                if i + j + min < GOAL {
                    Some((i + j, (i, j)))
                } else {
                    None
                }
            })
        })
        .collect();

    nums.iter()
        .filter_map(|&k| {
            let diff = GOAL - k;
            if let Some((i, j)) = cache.get(&diff) {
                return Some(i * j * k);
            } else {
                None
            }
        })
        .next()
        .expect("should have a value")
}

pub fn run() {
    let start = Instant::now();
    let nums = {
        let mut n = load_numbers();
        n.sort();
        n
    };
    let data_loaded = Instant::now();
    let p1 = part1(&nums);
    let done_part1 = Instant::now();
    let p2 = part2(&nums);
    let done_part2 = Instant::now();

    println!("--------------------");
    println!("day 1");
    println!("    part 1: {}", p1);
    println!("    part 2: {}", p2);
    println!("time taken:");
    println!("    total: {:?}", done_part2.duration_since(start));
    println!("    data load: {:?}", data_loaded.duration_since(start));
    println!("    part 1: {:?}", done_part1.duration_since(data_loaded));
    println!("    part 2: {:?}", done_part2.duration_since(done_part1));
}
