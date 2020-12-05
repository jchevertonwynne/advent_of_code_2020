use std::time::Instant;

#[derive(Default, Debug)]
struct Record {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

fn number_in_range(year: &str, min_year: usize, max_year: usize) -> bool {
    match year.parse::<usize>() {
        Ok(year) => year >= min_year && year <= max_year,
        Err(_) => false,
    }
}

impl Record {
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
            Some(y) => number_in_range(y, 1920, 2002),
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
                    _ => return false,
                }
            }
            None => return false,
        }
    }

    fn valid_hair_colour(&self) -> bool {
        let colour = match &self.hair_colour {
            Some(colour) => colour,
            None => return false,
        };

        let hex = &colour[1..];
        colour.chars().nth(0) == Some('#')
            && hex.len() == 6
            && hex
                .chars()
                .all(|c| (c >= '0' && c <= '9') || c >= 'a' && c <= 'f')
    }

    fn valid_eye_colour(&self) -> bool {
        match &self.eye_colour {
            Some(colour) => match colour.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            None => return false,
        }
    }

    fn valid_passport_id(&self) -> bool {
        match &self.passport_id {
            Some(id) => id.len() == 9 && id.chars().all(|c| c >= '0' && c <= '9'),
            None => return false,
        }
    }
}

fn load_records() -> Vec<Record> {
    std::fs::read_to_string("files/04.txt")
        .expect("should exist")
        .trim()
        .split("\n\n")
        .map(|record| {
            let mut curr = Record::default();
            for line in record.lines() {
                for characteristic in line.split(' ') {
                    let status = Some(characteristic[4..].to_string());
                    match &characteristic[..3] {
                        "byr" => curr.birth_year = status,
                        "iyr" => curr.issue_year = status,
                        "eyr" => curr.expiration_year = status,
                        "hgt" => curr.height = status,
                        "hcl" => curr.hair_colour = status,
                        "ecl" => curr.eye_colour = status,
                        "pid" => curr.passport_id = status,
                        "cid" => curr.country_id = status,
                        _ => panic!("bad input: {}", characteristic),
                    }
                }
            }
            curr
        })
        .collect()
}

fn part1(records: &Vec<Record>) -> usize {
    records.iter().filter(|r| r.has_fields()).count()
}

fn part2(records: &Vec<Record>) -> usize {
    records.iter().filter(|r| r.valid_fields()).count()
}

pub fn run() {
    let start = Instant::now();
    let records = load_records();
    let data_loaded = Instant::now();
    let p1 = part1(&records);
    let done_part1 = Instant::now();
    let p2 = part2(&records);
    let done_part2 = Instant::now();

    println!("--------------------");
    println!("day 4");
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
        let records = load_records();
        assert_eq!(part1(&records), 228)
    }

    #[test]
    fn part2_test() {
        let records = load_records();
        assert_eq!(part2(&records), 175)
    }

    #[test]
    fn part2_tests() {
        let mut record = Record::default();
        record.passport_id = Some("087499704".to_string());
        record.height = Some("74in".to_string());
        record.eye_colour = Some("grn".to_string());
        record.issue_year = Some("2012".to_string());
        record.expiration_year = Some("2030".to_string());
        record.birth_year = Some("1980".to_string());
        record.hair_colour = Some("#623a2f".to_string());

        assert_eq!(record.valid_fields(), true)
    }
}
