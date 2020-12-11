use std::time::{Duration, Instant};

const GOAL: usize = 2020;

lazy_static! {
    static ref STATIC_NUMBERS: Vec<usize> = load_numbers(INPUT);
}

const INPUT: &str = include_str!("../../files/01.txt");

fn load_numbers(input: &str) -> Vec<usize> {
    input
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
            Some(&ij) if ij != 0 => Some(ij * k),
            _ => None,
        })
        .next()
        .expect("should be an answer")
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let p1 = part1(&STATIC_NUMBERS);
    let p2 = part2(&STATIC_NUMBERS);

    (p1, p2, start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let numbers = load_numbers(INPUT);
        assert_eq!(part1(&numbers), 319531)
    }

    #[test]
    fn part2_test() {
        let numbers = load_numbers(INPUT);
        assert_eq!(part2(&numbers), 244300320)
    }
}
