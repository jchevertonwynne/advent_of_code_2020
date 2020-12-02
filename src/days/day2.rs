use std::num::ParseIntError;
use std::ops::Try;
use std::str::FromStr;

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
            return Err(String::from(format!(
                "{} does not have 2 separating spaces",
                s
            )));
        }

        let limits = parts[0];
        let lims = limits.split('-').collect::<Vec<_>>();
        if lims.len() != 2 {
            return Err(String::from(format!(
                "{} does not have 1 separating hyphen",
                limits
            )));
        }
        let min = lims[0]
            .parse()
            .map_err(|err: ParseIntError| err.to_string())?;
        let max = lims[1]
            .parse()
            .map_err(|err: ParseIntError| err.to_string())?;

        let char = parts[1]
            .chars()
            .nth(0)
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
        let mut seen = 0;
        for char in self.password.chars() {
            if char == self.req.char {
                seen += 1;
            }
        }

        return seen >= self.req.min && seen <= self.req.max;
    }
    fn alt_valid(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        let first_contains =
            self.req.min < self.password.len() + 1 && chars[self.req.min - 1] == self.req.char;
        let second_valid = self.req.max < self.password.len() + 1;
        if second_valid {
            let second_contains = chars[self.req.max - 1] == self.req.char;
            first_contains ^ second_contains
        } else {
            first_contains
        }
    }
}

fn load_entries() -> Vec<Entry> {
    let contents = std::fs::read_to_string("files/02.txt").expect("should be there");

    let mut res = Vec::new();

    for line in contents.trim().lines() {
        res.push(line.parse().expect("should be valid input"));
    }

    res
}

fn part1(entries: &Vec<Entry>) -> usize {
    entries.iter().filter(|e| e.valid()).count()
}

fn part2(entries: &Vec<Entry>) -> usize {
    entries.iter().filter(|e| e.alt_valid()).count()
}

pub fn run() {
    let start = std::time::Instant::now();
    let entries = load_entries();
    let data_loaded = std::time::Instant::now();
    let p1 = part1(&entries);
    let done_part1 = std::time::Instant::now();
    let p2 = part2(&entries);
    let done_part2 = std::time::Instant::now();

    println!("day 2");
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
    fn part2() {
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
