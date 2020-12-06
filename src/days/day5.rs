use std::time::Instant;

fn load_ids() -> Vec<usize> {
    include_str!("../../files/05.txt")
        .lines()
        .map(|line| {
            line.chars().fold(0, |acc, v| {
                (acc << 1)
                    + match v {
                        'R' | 'B' => 1,
                        _ => 0,
                    }
            })
        })
        .collect()
}

fn part1(passes: &Vec<usize>) -> usize {
    *passes.iter().max().expect("non zero entries")
}

fn part2(passes: &Vec<usize>) -> usize {
    let mut smallest = usize::max_value();
    let mut largest = 0;
    let mut curr = 0;
    passes.iter().for_each(|&id| {
        smallest = usize::min(smallest, id);
        largest = usize::max(largest, id);
        curr ^= id;
    });
    let dist = largest - smallest;
    let poss_dist = largest.next_power_of_two() - largest;

    let iter: Box<dyn Iterator<Item = usize>> = match dist < poss_dist {
        true => Box::new(smallest..=largest),
        false => Box::new((1..smallest).chain((largest + 1)..largest.next_power_of_two())),
    };

    iter.for_each(|i| curr ^= i);
    curr
}

pub fn run() {
    let start = Instant::now();
    let passes = load_ids();
    let data_loaded = Instant::now();
    let p1 = part1(&passes);
    let done_part1 = Instant::now();
    let p2 = part2(&passes);
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
    fn test_part1() {
        let passes = load_ids();
        assert_eq!(part1(&passes), 885);
    }

    #[test]
    fn test_part2() {
        let passes = load_ids();
        assert_eq!(part2(&passes), 623);
    }

    #[test]
    fn part_2_scenarios() {
        for i in 20..80 {
            for start in 5..10 {
                for end in 90..123 {
                    let args = (start..end).filter(|&v| v != i).collect();
                    assert_eq!(part2(&args), i);
                }
            }
        }
    }

    #[test]
    fn to_id() {
        let acc = |acc, v| {
            (acc << 1)
                + match v {
                    'R' | 'B' => 1,
                    _ => 0,
                }
        };

        let line = "BFFFBBFRRR";
        let id = line.chars().fold(0, acc);
        assert_eq!(id, 567);

        let line = "FFFBBBFRRR";
        let id = line.chars().fold(0, acc);
        assert_eq!(id, 119);

        let line = "BBFFBBFRLL";
        let id = line.chars().fold(0, acc);
        assert_eq!(id, 820);
    }
}
