use std::time::Instant;

const INPUT: &str = include_str!("../../files/05.txt");

fn solve(input: &str) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut smallest = usize::max_value();
    let mut largest = usize::min_value();
    input
        .lines()
        .map(|line| {
            line.chars().fold(0, |acc, v| {
                (acc << 1)
                    + match v {
                    'R' | 'B' => 1,
                    _ => 0,
                }
            })
        }).for_each(|i| {
        p1 = p1.max(i);
        p2 ^= i;
        smallest = usize::min(smallest, i);
        largest = usize::max(largest, i);
    });

    let dist = largest - smallest;
    let poss_dist = largest.next_power_of_two() - largest;

    if dist > poss_dist {
        (1..smallest).for_each(|i| p2 ^= i);
        ((largest + 1)..largest.next_power_of_two()).for_each(|i| p2 ^= i);
    } else {
        (smallest..=largest).for_each(|i| p2 ^= i);
    }

    (p1, p2)
}

pub fn run() {
    let start = Instant::now();
    let (p1, p2) = solve(INPUT);
    let end = Instant::now();

    println!("    part 1: {}", p1);
    println!("    part 2: {}", p2);
    println!("time taken:");
    println!("    total: {:?}", end.duration_since(start));
}

#[cfg(test)]
mod tests {
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
