use std::ops::{BitAnd, BitOr};
use std::time::Instant;

const INPUT: &str =  include_str!("../../files/06.txt");

fn solve_both(input: &str) -> (usize, usize) {
   input
        .split("\n\n")
        .map(|group| {
            let mut group = group
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| (1 << (c as usize - 'a' as usize)))
                        .fold(0usize, |acc, i| acc.bitor(i))
                });
            let first = group.next().expect("pls");
            let (a, b) = group.fold((first, first), |(a, b), v| {
                (a.bitor(v), b.bitand(v))
            });
            (a.count_ones() as usize, b.count_ones() as usize)
        })
       .fold((0, 0), |(a, b), (c, d)| {
        (a + c, b + d)
    })
}

pub fn run() {
    let start = Instant::now();
    let (p1, p2) = solve_both(INPUT);
    let end = Instant::now();

    println!("    part 1: {}", p1);
    println!("    part 2: {}", p2);
    println!("time taken:");
    println!("    total: {:?}", end.duration_since(start));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both() {
        assert_eq!(solve_both(INPUT), (6297, 3158));
    }

}
