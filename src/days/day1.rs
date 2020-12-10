use std::time::Instant;

const GOAL: usize = 2020;

fn load_numbers() -> Vec<usize> {
    include_str!("../../files/01.txt")
        .lines()
        .map(|i| i.parse().expect("should be valid usize"))
        .collect()
}

fn part1(nums: &[usize]) -> usize {
    let mut seen = [0usize; GOAL / 64 + 1];
    for &num in nums {
        let short = GOAL - num;
        if seen[short / 64] & (1 << (short % 64)) != 0 {
            return num * short;
        }
        seen[num / 64] |= 1 << (num % 64);
    }
    panic!("lol")
}

fn part2(nums: &[usize]) -> usize {
    let mut places = [0; GOAL];
    for (ind, &i) in nums.iter().enumerate() {
        for &j in &nums[(ind + 1)..] {
            if i + j >= GOAL {
                continue;
            }
            places[i + j] = i * j;
        }
    }

    nums.iter()
        .filter_map(|&k| match places.get(GOAL - k) {
            Some(0) => None,
            Some(ij) => Some(ij * k),
            _ => None,
        })
        .next()
        .expect("should be an answer")
}

pub fn run() {
    let start = Instant::now();
    let nums = load_numbers();
    let data_loaded = Instant::now();
    let p1 = part1(&nums);
    let done_part1 = Instant::now();
    let p2 = part2(&nums);
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
        let numbers = load_numbers();
        assert_eq!(part1(&numbers), 319531)
    }

    #[test]
    fn part2_test() {
        let numbers = load_numbers();
        assert_eq!(part2(&numbers), 244300320)
    }
}
