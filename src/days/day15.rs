use std::time::{Duration, Instant};

const INPUT: [usize; 6] = [1, 2, 16, 19, 18, 0];

#[derive(Clone)]
enum Seen {
    Never,
    Once(usize),
    Twice(usize, usize),
}

impl Seen {
    fn insert(&mut self, val: usize) {
        *self = match self {
            Seen::Never => Seen::Once(val),
            Seen::Once(a) => Seen::Twice(*a, val),
            Seen::Twice(_, a) => Seen::Twice(*a, val),
        }
    }
}

fn process(nums: &[usize], lim: usize) -> usize {
    let mut spoken: Vec<Seen> = vec![Seen::Never; lim];
    let mut last_spoken = 0;
    for i in 1..=lim {
        let next = if i <= nums.len() {
            nums[i - 1]
        } else {
            match spoken[last_spoken] {
                Seen::Never => panic!("no thanks"),
                Seen::Once(_) => 0,
                Seen::Twice(a, b) => b - a,
            }
        };

        spoken[next].insert(i);
        last_spoken = next;
    }

    last_spoken
}

fn part1(nums: &[usize]) -> usize {
    process(nums, 2020)
}

fn part2(nums: &[usize]) -> usize {
    process(nums, 30_000_000)
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let p1 = part1(&INPUT);
    let p2 = part2(&INPUT);

    (p1, p2, start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day15::{part1, part2, INPUT};

    #[test]
    fn test_actual() {
        assert_eq!(part1(&INPUT), 536);
        assert_eq!(part2(&INPUT), 24065124);
    }

    #[test]
    fn test_part1_example() {
        let a = [1, 3, 2];
        assert_eq!(part1(&a), 1);

        let a = [2, 1, 3];
        assert_eq!(part1(&a), 10);

        let a = [3, 1, 2];
        assert_eq!(part1(&a), 1836);
    }
}
