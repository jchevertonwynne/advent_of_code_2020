use std::time::{Duration, Instant};

const INPUT: [u32; 6] = [1, 2, 16, 19, 18, 0];

fn process(nums: &[u32], lim: u32) -> usize {
    let mut spoken: Vec<(u32, u32)> = vec![(0, 0); lim as usize];
    let mut last_spoken = 0u32;

    for i in 1u32..=lim {
        let next = if i <= nums.len() as u32 {
            nums[(i - 1) as usize]
        } else {
            let r = spoken[last_spoken as usize];
            if r.0 == 0 {
                0
            } else {
                r.1 - r.0
            }
        };

        let r = &mut spoken[next as usize];
        r.0 = std::mem::replace(&mut r.1, i);

        last_spoken = next;
    }

    last_spoken as usize
}

fn part1(nums: &[u32]) -> usize {
    process(nums, 2020)
}

fn part2(nums: &[u32]) -> usize {
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
        assert_eq!(part2(&INPUT), 24_065_124);
    }

    #[test]
    fn test_part1_example() {
        let a = [0, 3, 6];
        assert_eq!(part1(&a), 436);

        let a = [1, 3, 2];
        assert_eq!(part1(&a), 1);

        let a = [2, 1, 3];
        assert_eq!(part1(&a), 10);

        let a = [3, 1, 2];
        assert_eq!(part1(&a), 1_836);
    }
}
