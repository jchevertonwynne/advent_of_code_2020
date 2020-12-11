use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/09.txt");

lazy_static! {
    static ref STATIC_INPUT: Vec<usize> = load_numbers(INPUT);
}

fn load_numbers(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|n| {
            n.parse::<usize>()
                .expect("input must be list of integers, one per line")
        })
        .collect()
}

fn part1(nums: &[usize], check: usize) -> usize {
    for window_and_goal in nums.windows(check + 1) {
        let goal = window_and_goal[check];
        let window = &window_and_goal[..check];

        let has_match = window
            .iter()
            .enumerate()
            .any(|(i, a)| window[i + 1..].iter().any(|b| a + b == goal));

        if !has_match {
            return goal;
        }
    }

    panic!("pls find an answer")
}

fn part2(nums: &[usize], goal: usize) -> usize {
    for i in 0.. {
        let mut sum = 0;
        for j in i..nums.len() {
            let next = nums[j];
            sum += next;
            if sum > goal {
                break;
            }
            if sum == goal {
                let (smallest, largest) = nums[i..=j].iter().fold(
                    (usize::max_value(), usize::min_value()),
                    |(low, high), &v| (low.min(v), high.max(v)),
                );
                return smallest + largest;
            }
        }
    }

    panic!("lol")
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let p1 = part1(&STATIC_INPUT, 26);
    let p2 = part2(&STATIC_INPUT, p1);

    (p1, p2, start.elapsed())
}

#[cfg(test)]
mod test {
    use crate::days::day09::{load_numbers, part1, part2, INPUT};

    #[test]
    fn actual_answers() {
        let nums = load_numbers(INPUT);
        assert_eq!(part1(&nums, 25), 15690279);
        assert_eq!(part2(&nums, 15690279), 2174232);
    }

    #[test]
    fn part2_test() {
        let nums = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let nums = load_numbers(nums);
        let p1 = part1(&nums, 5);
        assert_eq!(p1, 127);

        let p2 = part2(&nums, p1);
        assert_eq!(p2, 62);
    }
}
