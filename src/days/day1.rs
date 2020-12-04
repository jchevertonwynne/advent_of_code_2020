use std::cmp::Ordering;
use std::time::Instant;

const GOAL: usize = 2020;

fn load_numbers() -> Vec<usize> {
    std::fs::read_to_string("files/01.txt")
        .expect("should be file")
        .trim()
        .lines()
        .map(|i| i.parse().expect("should be valid usize"))
        .collect()
}

fn part1(nums: &Vec<usize>) -> usize {
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut t = nums[i] + nums[j];

    loop {
        match t.cmp(&GOAL) {
            Ordering::Less => i += 1,
            Ordering::Equal => return nums[i] * nums[j],
            Ordering::Greater => j -= 1,
        }
        t = nums[i] + nums[j];
    }
}

fn part2(nums: &Vec<usize>) -> usize {
    let smallest = nums[0] + nums[1];
    let max_needed = GOAL - smallest;
    let top = match nums.binary_search_by(|&i| i.cmp(&max_needed)) {
        Ok(i) => i,
        Err(i) => i,
    };

    let min = nums[0];
    let nums = &nums[..top];
    let mut places = [(0, 0); GOAL];
    for (ind, &i) in nums.iter().enumerate() {
        for &j in &nums[(ind + 1)..] {
            if i + j + min > GOAL {
                break;
            }
            places[i + j] = (i, j);
        }
    }

    nums.iter()
        .filter_map(|&k| match places.get(GOAL - k) {
            Some(&(0, 0)) => None,
            Some(&(i, j)) => Some(i * j * k),
            _ => None,
        })
        .next()
        .expect("should be an answer")
}

pub fn run() {
    let start = Instant::now();
    let nums = {
        let mut n = load_numbers();
        n.sort();
        n
    };
    let data_loaded = Instant::now();
    let p1 = part1(&nums);
    let done_part1 = Instant::now();
    let p2 = part2(&nums);
    let done_part2 = Instant::now();

    println!("--------------------");
    println!("day 1");
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
        let numbers = {
            let mut r = load_numbers();
            r.sort();
            r
        };
        assert_eq!(part1(&numbers), 319531)
    }

    #[test]
    fn part2_test() {
        let numbers = {
            let mut r = load_numbers();
            r.sort();
            r
        };
        assert_eq!(part2(&numbers), 244300320)
    }
}
