use std::time::Instant;

const INPUT: &str = include_str!("../../files/10.txt");

fn load_input(input: &str) -> Vec<usize> {
    let mut res = input
        .lines()
        .map(|line| line.parse().expect("int val pls"))
        .collect::<Vec<_>>();
    res.sort_unstable();
    res
}

fn part1(jolts: &[usize]) -> usize {
    let mut ones = 1;
    let mut threes = 1;

    for (first, second) in jolts.iter().zip(jolts.iter().skip(1)) {
        match second - first {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }

    ones * threes
}

fn part2(jolts: &[usize]) -> usize {
    let mut ways_to_next = Vec::with_capacity(jolts.len() + 1);
    ways_to_next.push(jolts.iter().take_while(|&j| *j <= 3).count());

    for i in 0..jolts.len() - 1 {
        ways_to_next.push(
            jolts[i + 1..]
                .iter()
                .take_while(|j| *j - jolts[i] <= 3)
                .count(),
        );
    }
    ways_to_next.push(1);

    (0..=ways_to_next.len() - 2)
        .rev()
        .for_each(|i| ways_to_next[i] = ways_to_next[i + 1..i + 1 + ways_to_next[i]].iter().sum());

    ways_to_next[0]
}

pub fn run() {
    let start = Instant::now();
    let jolts = load_input(INPUT);
    let data_loaded = Instant::now();
    let p1 = part1(&jolts);
    let done_part1 = Instant::now();
    let p2 = part2(&jolts);
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
mod tests {
    use crate::days::day10::{load_input, part2, INPUT, part1};

    #[test]
    fn test_actual() {
        let nums = load_input(INPUT);
        assert_eq!(part1(&nums), 2484);
        assert_eq!(part2(&nums), 15790581481472);
    }

    #[test]
    fn test_part2() {
        let mut nums = load_input(
            "16
10
15
5
1
11
7
19
6
12
4",
        );
        nums.sort_unstable();
        assert_eq!(part2(&nums), 8);

        let mut nums = load_input(
            "1
2
3",
        );
        nums.sort_unstable();
        assert_eq!(part2(&nums), 4);

        let mut nums = load_input(
            "1
2
3
4",
        );
        nums.sort_unstable();
        assert_eq!(part2(&nums), 7);

        println!("------------");

        let mut nums = load_input(
            "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3",
        );
        nums.sort_unstable();
        println!("{:?}", nums);
        assert_eq!(part2(&nums), 19208);
    }
}
