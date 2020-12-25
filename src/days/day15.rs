use fnv::FnvBuildHasher;
use std::collections::HashMap;
use std::time::{Duration, Instant};

const INPUT: [u32; 6] = [1, 2, 16, 19, 18, 0];

fn process(nums: &[u32], lim: u32) -> usize {
    let mut spoken = vec![0u32; (lim + 1) as usize];

    for (i, next) in (1..).zip(nums) {
        spoken[*next as usize] = i;
    }
    let mut last_spoken = *nums.last().unwrap();
    for i in (nums.len() + 1) as u32..=lim {
        let mut result = spoken[last_spoken as usize];
        if result != 0 {
            result = (i - 1) - result;
        }

        spoken[last_spoken as usize] = i - 1;
        last_spoken = result;
    }

    last_spoken as usize
}

fn part1(nums: &[u32]) -> usize {
    process(nums, 2020)
}

fn part2(nums: &[u32]) -> usize {
    const LIM: u32 = 1 << 22;
    let mut spoken_small = vec![0u32; LIM as usize];
    let mut spoken_large: HashMap<u32, u32, FnvBuildHasher> =
        HashMap::with_capacity_and_hasher(1_400_000, FnvBuildHasher::default());
    for (i, next) in (1..).zip(nums) {
        spoken_small[*next as usize] = i;
    }

    let mut last_spoken = *nums.last().unwrap();
    for i in (nums.len()) as u32..30_000_000 {
        let d = if last_spoken < LIM {
            spoken_small.get_mut(last_spoken as usize).unwrap()
        } else {
            spoken_large.entry(last_spoken).or_insert(0)
        };
        let mut result = *d;
        if result != 0 {
            result = i - result;
        }

        *d = i;
        last_spoken = result;
    }

    last_spoken as usize
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let p1 = part1(&INPUT);
    let p2 = part2(&INPUT);

    (p1.to_string(), p2.to_string(), start.elapsed())
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
