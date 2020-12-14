use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/13.txt");

fn load_busses(input: &str) -> (usize, Vec<Option<usize>>) {
    let mut lines = input.lines();
    let timestamp = lines.next().expect("pls").parse().expect("pls");
    let busses = lines
        .next()
        .expect("pls")
        .split(',')
        .map(|b| {
            if b == "x" {
                None
            } else {
                Some(b.parse().expect("go on"))
            }
        })
        .collect();

    (timestamp, busses)
}

fn part1(timestamp: usize, busses: &[Option<usize>]) -> usize {
    let first = busses
        .iter()
        .filter_map(|b| *b)
        .min_by(|&a, &b| Ord::cmp(&(a - (timestamp % a)), &(b - (timestamp % b))))
        .expect("more than one bus");
    first * (first - (timestamp % first))
}

fn part2(busses: &[Option<usize>]) -> usize {
    let (indices, bus_ids): (Vec<_>, Vec<_>) = busses
        .iter()
        .enumerate()
        .filter_map(|(offset, bus_id)| bus_id.map(|id| (offset as i128, id as i128)))
        .unzip();

    let mut res = ring_algorithm::chinese_remainder_theorem(&indices, &bus_ids).expect("pls");

    let product = bus_ids.iter().product::<i128>();

    if res < 0 {
        res += product;
    }

    (product - res) as usize
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let (timestamp, busses) = load_busses(INPUT);
    let p1 = part1(timestamp, &busses);
    let p2 = part2(&busses);

    (p1, p2, start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day13::{load_busses, part1, part2, INPUT};

    #[test]
    fn test_actual() {
        let (timestamp, busses) = load_busses(INPUT);
        assert_eq!(part1(timestamp, &busses), 174);
        assert_eq!(part2(&busses), 780601154795940)
    }

    #[test]
    fn test_example() {
        let s = "939
7,13,x,x,59,x,31,19";
        let (t, b) = load_busses(s);
        assert_eq!(part1(t, &b), 295);
        assert_eq!(part2(&b), 1_068_781);
    }

    #[test]
    fn test_part2_example() {
        let s = "123
17,x,13,19";
        let (_, b) = load_busses(s);
        assert_eq!(part2(&b), 3_417);

        let s = "123
67,7,59,61";
        let (_, b) = load_busses(s);
        assert_eq!(part2(&b), 754_018);

        let s = "123
67,x,7,59,61";
        let (_, b) = load_busses(s);
        assert_eq!(part2(&b), 779_210);

        let s = "123
67,7,x,59,61";
        let (_, b) = load_busses(s);
        assert_eq!(part2(&b), 1_261_476);

        let s = "123
1789,37,47,1889";
        let (_, b) = load_busses(s);
        assert_eq!(part2(&b), 1_202_161_486);
    }
}
