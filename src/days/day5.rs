use std::time::Instant;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Pass {
    row: usize,
    column: usize,
    id: usize
}

impl FromStr for Pass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(format!("invalid input length: was {}, expected 10", s.len()))
        }

        let row = &s[..7];
        let column = &s[7..];

        if !row.chars().all(|c| c == 'F' || c == 'B') {
            return Err(format!("invalid row identifier {}", row));
        }

        if !column.chars().all(|c| c == 'L' || c == 'R') {
            return Err(format!("invalid column identifier {}", row));
        }

        let row = row.chars().into_iter().fold(0, shunt('B'));
        let column = column.chars().into_iter().fold(0, shunt('R'));
        let id = row * 8 + column;

        Ok(Pass { row, column, id })
    }
}

fn shunt(delim: char) -> Box<dyn Fn(usize, char) -> usize> {
    Box::new(move |mut acc, c| {
        acc <<= 1;
        if c == delim {
            acc += 1;
        }
        acc
    })
}

fn load_passes() -> Vec<Pass> {
    std::fs::read_to_string("files/05.txt")
        .expect("file exists")
        .trim()
        .lines()
        .map(|line| line.parse().expect("should work"))
        .collect()
}

fn part1(passes: &Vec<Pass>) -> usize {
    passes
        .iter()
        .map(|pass| pass.id)
        .max()
        .expect("non zero entries")
}

fn part2(passes: &Vec<Pass>) -> usize {
    for w in passes.windows(2) {
        let a = w[0].id;
        let b = w[1].id;
        if b - a != 1 {
            return a + 1
        }
    }
    panic!("lol")
}

pub fn run() {
    let start = Instant::now();
    let mut passes = load_passes();
    passes.sort_by(|a, b| a.id.cmp(&b.id));
    let data_loaded = Instant::now();
    let p1 = part1(&passes);
    let done_part1 = Instant::now();
    let p2 = part2(&passes);
    let done_part2 = Instant::now();

    println!("--------------------");
    println!("day 5");
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
    fn test_part1() {
        let passes = load_passes();
        assert_eq!(part1(&passes), 885);
    }

    #[test]
    fn test_part2() {
        let mut passes = load_passes();
        passes.sort_by(|a, b| a.id.cmp(&b.id));
        assert_eq!(part2(&passes), 623);
    }

    #[test]
    fn pass_generation() {
        let pass = "FBFBBFFRLR".parse::<Pass>().expect("valid pass");
        assert_eq!(pass, Pass{row: 44, column: 5, id: 44 * 8 + 5});

        let pass = "BFFFBBFRRR".parse::<Pass>().expect("valid pass");
        assert_eq!(pass, Pass{row: 70, column: 7, id: 70 * 8 + 7});

        let pass = "FFFBBBFRRR".parse::<Pass>().expect("valid pass");
        assert_eq!(pass, Pass{row: 14, column: 7, id: 14 * 8 + 7});

        let pass = "BBFFBBFRLL".parse::<Pass>().expect("valid pass");
        assert_eq!(pass, Pass{row: 102, column: 4, id: 102 * 8 + 4});
    }
}