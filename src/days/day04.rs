use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/04.txt");

#[derive(Default, Debug)]
struct Record<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiration_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_colour: Option<&'a str>,
    eye_colour: Option<&'a str>,
    passport_id: Option<&'a str>,
}

fn number_in_range(year: &str, min_year: usize, max_year: usize) -> bool {
    match year.parse::<usize>() {
        Ok(year) => year >= min_year && year <= max_year,
        Err(_) => false,
    }
}

impl Record<'_> {
    fn has_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_colour.is_some()
            && self.eye_colour.is_some()
            && self.passport_id.is_some()
    }

    fn valid_fields(&self) -> bool {
        self.valid_birth_year()
            && self.valid_issue_year()
            && self.valid_expiration_year()
            && self.valid_height()
            && self.valid_hair_colour()
            && self.valid_eye_colour()
            && self.valid_passport_id()
    }

    fn valid_birth_year(&self) -> bool {
        match &self.birth_year {
            Some(year) => number_in_range(year, 1920, 2002),
            None => false,
        }
    }

    fn valid_issue_year(&self) -> bool {
        match &self.issue_year {
            Some(year) => number_in_range(year, 2010, 2020),
            None => false,
        }
    }

    fn valid_expiration_year(&self) -> bool {
        match &self.expiration_year {
            Some(year) => number_in_range(year, 2020, 2030),
            None => false,
        }
    }

    fn valid_height(&self) -> bool {
        match &self.height {
            Some(height) => {
                if height.len() <= 2 {
                    return false;
                }
                let unit = &height[height.len() - 2..];
                match unit {
                    "cm" => number_in_range(&height[..height.len() - 2], 150, 193),
                    "in" => number_in_range(&height[..height.len() - 2], 59, 76),
                    _ => false,
                }
            }
            None => false,
        }
    }

    fn valid_hair_colour(&self) -> bool {
        let colour = match &self.hair_colour {
            Some(colour) => colour,
            None => return false,
        };

        let hex = &colour[1..];
        colour.starts_with('#')
            && hex.len() == 6
            && hex
                .chars()
                .all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
    }

    fn valid_eye_colour(&self) -> bool {
        match &self.eye_colour {
            Some(colour) => matches!(
                colour,
                &"amb" | &"blu" | &"brn" | &"gry" | &"grn" | &"hzl" | &"oth"
            ),
            None => false,
        }
    }

    fn valid_passport_id(&self) -> bool {
        match &self.passport_id {
            Some(id) => id.len() == 9 && id.chars().all(|c| ('0'..='9').contains(&c)),
            None => false,
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    input
        .split("\n\n")
        .map(|record| {
            let mut curr = Record::default();
            for line in record.lines() {
                for characteristic in line.split(' ') {
                    let status = Some(&characteristic[4..]);
                    match &characteristic[..3] {
                        "byr" => curr.birth_year = status,
                        "iyr" => curr.issue_year = status,
                        "eyr" => curr.expiration_year = status,
                        "hgt" => curr.height = status,
                        "hcl" => curr.hair_colour = status,
                        "ecl" => curr.eye_colour = status,
                        "pid" => curr.passport_id = status,
                        "cid" => (),
                        _ => panic!("bad input: {}", characteristic),
                    }
                }
            }
            curr
        })
        .map(|record| match record.has_fields() {
            false => (0, 0),
            true => match record.valid_fields() {
                true => (1, 1),
                false => (1, 0),
            },
        })
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d))
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let (p1, p2) = solve(INPUT);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts_test() {
        assert_eq!(solve(INPUT), (228, 175));
    }

    #[test]
    fn part2_tests() {
        let mut record = Record::default();
        record.passport_id = Some("087499704");
        record.height = Some("74in");
        record.eye_colour = Some("grn");
        record.issue_year = Some("2012");
        record.expiration_year = Some("2030");
        record.birth_year = Some("1980");
        record.hair_colour = Some("#623a2f");

        assert_eq!(record.valid_fields(), true)
    }
}
