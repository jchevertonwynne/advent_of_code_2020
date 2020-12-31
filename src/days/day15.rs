use std::time::{Duration, Instant};

const INPUT: [u32; 6] = [1, 2, 16, 19, 18, 0];

fn solve(nums: &[u32]) -> (usize, usize) {
    let mut spoken = vec![0; 30_000_001];
    for (i, next) in (1..).zip(nums) {
        spoken[*next as usize] = i;
    }

    let mut p1_ans = 0;
    let mut last_spoken = *nums.last().unwrap();
    for i in (nums.len()) as u32..30_000_000 {
        let d = &mut spoken[last_spoken as usize];
        let mut result = *d;
        if result != 0 {
            result = i - result;
        }

        *d = i;
        last_spoken = result;

        if i == 2019 {
            p1_ans = last_spoken as usize;
        }
    }

    (p1_ans, last_spoken as usize)
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let (p1, p2) = solve(&INPUT);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day15::{solve, INPUT};

    #[test]
    fn test_actual() {
        assert_eq!(solve(&INPUT), (536, 24_065_124));
    }
}
