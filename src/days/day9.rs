use std::time::Instant;

const INPUT: &str = include_str!("../../files/09.txt");

fn load_numbers(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|n| {
            n.parse()
                .expect("input must be list of integers, one per line")
        })
        .collect()
}

fn part1(nums: &[usize], check: usize) -> usize {
    for window in nums.windows(check + 1) {
        let checking = window[check];
        let window = &window[..check];

        let has_match = window
            .iter()
            .enumerate()
            .any(|(i, a)| window[i + 1..].iter().any(|b| a + b == checking));

        if !has_match {
            return checking;
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
                let (smallest, largest) = nums[i..j]
                    .iter()
                    .fold((usize::max_value(), usize::min_value()), |(a, b), &v| {
                        (a.min(v), b.min(v))
                    });
                return smallest + largest;
            }
        }
    }

    panic!("lol")
}

pub fn run() {
    let start = Instant::now();
    let nums = load_numbers(INPUT);
    let data_loaded = Instant::now();
    let p1 = part1(&nums, 26);
    let done_part1 = Instant::now();
    let p2 = part2(&nums, p1);
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
mod test {
    use crate::days::day9::{load_numbers, part1, part2, INPUT};

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