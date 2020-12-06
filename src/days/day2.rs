use std::num::ParseIntError;
use std::ops::Try;
use std::str::FromStr;
use std::time::Instant;

struct Requirement {
    char: char,
    min: usize,
    max: usize,
}

struct Entry {
    req: Requirement,
    password: String,
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(format!("{} does not have 2 separating spaces", s));
        }

        let limits = parts[0];
        let lims = limits.split('-').collect::<Vec<_>>();
        if lims.len() != 2 {
            return Err(format!("{} does not have 1 separating hyphen", limits));
        }
        let min = lims[0]
            .parse()
            .map_err(|err: ParseIntError| err.to_string())?;
        let max = lims[1]
            .parse()
            .map_err(|err: ParseIntError| err.to_string())?;

        let char = parts[1]
            .chars()
            .next()
            .into_result()
            .map_err(|_| String::from("char mustn't be 0 length"))?;
        let password = parts[2].to_string();

        Ok(Entry {
            req: Requirement { char, min, max },
            password,
        })
    }
}

impl Entry {
    fn valid(&self) -> bool {
        let seen = self
            .password
            .chars()
            .filter(|&c| c == self.req.char)
            .count();
        seen >= self.req.min && seen <= self.req.max
    }

    fn alt_valid(&self) -> bool {
        let mut chars = self.password.chars();
        match chars.nth(self.req.min - 1) {
            Some(first) => match chars.nth(self.req.max - self.req.min - 1) {
                Some(second) => (first == self.req.char) ^ (second == self.req.char),
                None => first == self.req.char,
            },
            None => false,
        }
    }
}

fn load_entries() -> Vec<Entry> {
    include_str!("../../files/02.txt")
        .lines()
        .map(|line| line.parse().expect("should be valid input"))
        .collect()
}

fn part1(entries: &[Entry]) -> usize {
    entries.iter().filter(|e| e.valid()).count()
}

fn part2(entries: &[Entry]) -> usize {
    entries.iter().filter(|e| e.alt_valid()).count()
}

pub fn run() {
    let start = Instant::now();
    let entries = load_entries();
    let data_loaded = Instant::now();
    let p1 = part1(&entries);
    let done_part1 = Instant::now();
    let p2 = part2(&entries);
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
    use super::*;

    #[test]
    fn part1_test() {
        let entries = load_entries();
        assert_eq!(part1(&entries), 483)
    }

    #[test]
    fn part2_test() {
        let entries = load_entries();
        assert_eq!(part2(&entries), 482)
    }

    #[test]
    fn part2_tests() {
        let entry: Entry = "1-3 a: abcde".parse().expect("should be valid entry");
        assert_eq!(entry.alt_valid(), true);

        let entry: Entry = "1-3 a: zbade".parse().expect("should be valid entry");
        assert_eq!(entry.alt_valid(), true);

        let entry: Entry = "1-3 b: cdefg".parse().expect("should be valid entry");
        assert_eq!(entry.alt_valid(), false);

        let entry: Entry = "2-9 c: ccccccccc".parse().expect("should be valid entry");
        assert_eq!(entry.alt_valid(), false)
    }
}
