use std::ops::{BitAnd, BitOr};
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/06.txt");

fn solve_both(input: &str) -> (usize, usize) {
    input
        .split("\n\n")
        .map(|group| {
            let mut group = group.lines().map(|line| {
                line.chars()
                    .map(|c| (1 << (c as usize - 'a' as usize)))
                    .fold(0usize, |acc, i| acc.bitor(i))
            });
            let first = group.next().expect("pls");
            let (a, b) = group.fold((first, first), |(a, b), v| (a.bitor(v), b.bitand(v)));
            (a.count_ones() as usize, b.count_ones() as usize)
        })
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d))
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let (p1, p2) = solve_both(INPUT);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both() {
        assert_eq!(solve_both(INPUT), (6_297, 3_158));
    }
}
