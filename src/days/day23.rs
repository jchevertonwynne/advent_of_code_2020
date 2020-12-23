use std::time::{Duration, Instant};
use std::convert::TryInto;

fn load_cups(mut input: usize) -> [u8; 9] {
    let mut res = [0; 9];

    let mut ind = 9;
    while input > 0 {
        ind -= 1;
        let i = input % 10;
        input /= 10;
        res[ind] = i as u8;
    }

    res.try_into().unwrap()
}

fn part1(cups: &[u8], turns: usize) -> usize {
    let mut tiles = vec![0; 10];
    for (p, n) in cups.iter().skip(1).zip(cups.iter()) {
        tiles[*n as usize] = *p as usize;
    }

    let first = *cups.first().unwrap() as usize;
    let last = *cups.last().unwrap() as usize;

    tiles[last] = first;

    let mut curr = *cups.first().unwrap() as usize;
    for _ in 0..turns {
        let a = tiles[curr];
        let b = tiles[a];
        let c = tiles[b];

        let mut destination = curr - 1;
        if destination == 0 {
            destination = 9;
        }
        while destination == a || destination == b || destination == c {
            destination -= 1;
            if destination == 0 {
                destination = 9;
            }
        }

        let now_after_curr = tiles[c];
        tiles[curr] = now_after_curr;

        tiles[c] = tiles[destination];
        tiles[destination] = a;
        curr = tiles[curr];
    }

    let mut res = 0;
    let mut c = 1;
    for _ in 0..8 {
        c = tiles[c];
        res *= 10;
        res += c;
    }
    res
}

fn part2(cups: &[u8]) -> usize {
    let mut tiles = vec![0; 1_000_001];
    for (p, n) in cups.iter().skip(1).zip(cups.iter()) {
        tiles[*n as usize] = *p as usize;
    }

    let first = *cups.first().unwrap() as usize;
    let last = *cups.last().unwrap() as usize;

    for i in 10..=1_000_000 {
        if i == 10 {
            tiles[last] = 10;
        }
        if i == 1_000_000 {
            tiles[i] = first;
        } else {
            tiles[i] = i + 1;
        }
    }

    let mut curr = *cups.first().unwrap() as usize;
    for _ in 0..10_000_000 {
        let a = tiles[curr];
        let b = tiles[a];
        let c = tiles[b];

        let mut destination = curr - 1;
        if destination == 0 {
            destination = 1_000_000;
        }
        while destination == a || destination == b || destination == c {
            destination -= 1;
            if destination == 0 {
                destination = 1_000_000;
            }
        }

        let now_after_curr = tiles[c];
        tiles[curr] = now_after_curr;
        tiles[c] = tiles[destination];
        tiles[destination] = a;

        curr = tiles[curr];
    }

    let a = tiles[1];
    let b = tiles[a];

    a * b
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let cups = load_cups(219347865);
    let p1 = part1(&cups, 100);
    let p2 = part2(&cups);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day23::{load_cups, part1, part2};

    #[test]
    fn test_part1() {
        let cups = load_cups(389125467);
        assert_eq!(part1(&cups, 10), 92658374);
        assert_eq!(part1(&cups, 100), 67384529);
    }

    #[test]
    fn test_part2() {
        let cups = load_cups(389125467);
        assert_eq!(part2(&cups), 149245887792);
    }
}
