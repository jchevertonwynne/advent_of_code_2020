use core::str::Split;
use std::convert::TryFrom;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/02.txt");

struct Requirement {
    char: char,
    min: usize,
    max: usize,
}

struct Entry<'a> {
    req: Requirement,
    password: &'a [u8],
}

fn get_next_int(source: &mut Split<char>) -> Result<usize, String> {
    match source.next() {
        Some(v) => v,
        None => return Err("not enough values found".to_string()),
    }
    .parse::<usize>()
    .map_err(|err| err.to_string())
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut parts = s.split(' ');

        let limits = parts.next().ok_or("pls be a thing")?;
        let mut limits = limits.split('-');

        let min = get_next_int(&mut limits)?;
        let max = get_next_int(&mut limits)?;

        let char = parts
            .next()
            .ok_or("pls be a thing")?
            .chars()
            .next()
            .ok_or_else(|| "char mustn't be 0 length".to_string())?;
        let password = parts.next().ok_or("pls be a thing")?.as_ref();

        Ok(Entry {
            req: Requirement { char, min, max },
            password,
        })
    }
}

impl Entry<'_> {
    fn valid(&self) -> bool {
        let seen = bytecount::count(self.password, self.req.char as u8);
        seen >= self.req.min && seen <= self.req.max
    }

    fn alt_valid(&self) -> bool {
        match self.password.get(self.req.min - 1) {
            Some(&first) => match self.password.get(self.req.max - 1) {
                Some(&second) => (first == self.req.char as u8) ^ (second == self.req.char as u8),
                None => first == self.req.char as u8,
            },
            None => false,
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    input
        .lines()
        .map(|line| Entry::try_from(line).expect("should be valid input"))
        .map(|entry| (entry.valid(), entry.alt_valid()))
        .fold((0, 0), |(a, b), status| match status {
            (true, true) => (a + 1, b + 1),
            (true, false) => (a + 1, b),
            (false, true) => (a, b + 1),
            (false, false) => (a, b),
        })
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let (p1, p2) = solve(INPUT);

    (p1, p2, start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts_test() {
        assert_eq!(solve(INPUT), (483, 482))
    }

    #[test]
    fn part2_tests() {
        let entry = Entry::try_from("1-3 a: abcde").expect("should be valid entry");
        assert_eq!(entry.alt_valid(), true);

        let entry = Entry::try_from("1-3 a: zbade").expect("should be valid entry");
        assert_eq!(entry.alt_valid(), true);

        let entry = Entry::try_from("1-3 b: cdefg").expect("should be valid entry");
        assert_eq!(entry.alt_valid(), false);

        let entry = Entry::try_from("2-9 c: ccccccccc").expect("should be valid entry");
        assert_eq!(entry.alt_valid(), false)
    }
}
