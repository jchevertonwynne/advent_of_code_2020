use std::convert::TryInto;
use std::time::{Duration, Instant};

fn load_cups(mut input: usize) -> [u8; 9] {
    let mut res = [0; 9];

    let mut ind = 9;
    while input > 0 {
        ind -= 1;
        let i = input % 10;
        input /= 10;
        res[ind] = i as u8;
    }

    res
}

fn part1(cups: &[u8], turns: usize) -> usize {
    let mut tiles = vec![0u8; 10];
    for (p, n) in cups.iter().skip(1).zip(cups.iter()) {
        tiles[*n as usize] = *p;
    }

    let first = *cups.first().unwrap();
    let last = *cups.last().unwrap() as usize;

    tiles[last] = first;

    let mut curr = *cups.first().unwrap();
    for _ in 0..turns {
        let a = tiles[curr as usize];
        let b = tiles[a as usize];
        let c = tiles[b as usize];

        let mut destination = curr;
        while destination == a || destination == b || destination == c || destination == curr {
            destination -= 1;
            if destination == 0 {
                destination = 9;
            }
        }

        let c = c as usize;
        let destination = destination as usize;

        let now_after_curr = tiles[c];
        tiles[curr as usize] = now_after_curr;
        tiles[c] = tiles[destination];
        tiles[destination] = a;

        curr = tiles[curr as usize];
    }

    let mut res = 0usize;
    let mut c = 1;
    for _ in 0..8 {
        c = tiles[c as usize];
        res *= 10;
        res += c as usize;
    }
    res
}

fn part2(cups: &[u8]) -> usize {
    let mut tiles: [u32; 1_000_001] = (1..=1_000_001).collect::<Vec<_>>().try_into().unwrap();
    for (p, n) in cups.iter().skip(1).zip(cups.iter()) {
        tiles[*n as usize] = *p as u32;
    }

    let first = *cups.first().unwrap() as u32;
    let last = *cups.last().unwrap() as usize;

    tiles[1_000_000] = first;
    tiles[last] = 10;

    let mut curr = cups[0] as u32;
    for _ in 0..10_000_000 {
        let a = tiles[curr as usize];
        let b = tiles[a as usize];
        let c = tiles[b as usize];

        let mut destination = curr;
        while destination == a || destination == b || destination == c || destination == curr {
            destination -= 1;
            if destination == 0 {
                destination = 1_000_000;
            }
        }

        let now_after_curr = tiles[c as usize];
        tiles[curr as usize] = now_after_curr;
        tiles[c as usize] = tiles[destination as usize];
        tiles[destination as usize] = a;

        curr = tiles[curr as usize];
    }

    let a = tiles[1] as usize;
    let b = tiles[a as usize] as usize;

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
