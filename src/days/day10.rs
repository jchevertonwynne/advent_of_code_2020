use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/10.txt");

lazy_static! {
    static ref PRECOMPUTED_INPUT: Vec<usize> = load_input(INPUT);
}

fn load_input(input: &str) -> Vec<usize> {
    let mut res = input
        .lines()
        .map(|line| line.parse().expect("int val pls"))
        .collect::<Vec<_>>();
    res.sort_unstable();
    res
}

fn solve(nums: &[usize]) -> (usize, usize) {
    let mut ones = 0;
    let mut threes = 0;
    let mut p2 = 1;

    let mut last = 0;
    let mut consec = 0;
    for num in nums {
        match num - last {
            1 => {
                ones += 1;
                consec += 1;
            }
            3 => {
                threes += 1;
                p2 *= match consec {
                    0 => 1,
                    1 => 1,
                    2 => 2,
                    3 => 4,
                    4 => 7,
                    5 => 13,
                    _ => panic!("lol1"),
                };
                consec = 0;
            }
            _ => panic!("lol2"),
        }
        last = *num;
    }

    p2 *= match consec {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        5 => 13,
        _ => panic!("lol1"),
    };

    threes += 1;
    (ones * threes, p2)
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let (p1, p2) = solve(&PRECOMPUTED_INPUT);
    let done = Instant::now();

    (p1, p2, done - start)
}

#[cfg(test)]
mod tests {
    use crate::days::day10::{load_input, solve, INPUT};

    #[test]
    fn test_actual() {
        let nums = load_input(INPUT);
        assert_eq!(solve(&nums), (2484, 15790581481472));
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
        assert_eq!(solve(&nums), (35, 8));

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
        assert_eq!(solve(&nums), (220, 19208));
    }
}
