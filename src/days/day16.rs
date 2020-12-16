use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/16.txt");

#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct Row<'a> {
    name: &'a str,
    a: Range,
    b: Range,
}

type Ticket = Vec<usize>;

impl Range {
    fn valid(&self, val: usize) -> bool {
        self.min <= val && val <= self.max
    }
}

impl Row<'_> {
    fn valid(&self, val: usize) -> bool {
        self.a.valid(val) || self.b.valid(val)
    }
}

fn load_tickets(input: &str) -> (Vec<Row>, Ticket, Vec<Ticket>) {
    let mut input = input.split('\n');

    let rows = (0..20)
        .map(|_| {
            let row = input.next().expect("pls");
            let colon = row.find(':').expect("should have colon");
            let name = &row[..colon];
            let nums = &row[colon + 2..];
            let or = nums.find(' ').expect("should have spaces");
            let first_nums = &nums[..or];
            let second_nums = &nums[or + 4..];
            let hypa = first_nums.find('-').expect("should be hyphen seperated");
            let mina = first_nums[..hypa].parse::<usize>().expect("shoud be int");
            let maxa = first_nums[hypa + 1..]
                .parse::<usize>()
                .expect("shoud be int");
            let hypb = second_nums.find('-').expect("should be hyphen seperated");
            let minb = second_nums[..hypb].parse::<usize>().expect("shoud be int");
            let maxb = second_nums[hypb + 1..]
                .parse::<usize>()
                .expect("shoud be int");
            Row {
                name,
                a: Range {
                    min: mina,
                    max: maxa,
                },
                b: Range {
                    min: minb,
                    max: maxb,
                },
            }
        })
        .collect();

    let mut input = input.skip(2);

    let ticket = input
        .next()
        .expect("should exist in input")
        .split(',')
        .map(|v| v.parse().expect("should be valid int"))
        .collect::<Vec<_>>();

    input.next();
    input.next();

    let tickets = input
        .map(|t| {
            t.split(',')
                .map(|v| v.parse().expect("should be valid int"))
                .collect()
        })
        .collect();

    (rows, ticket, tickets)
}

fn part1<'a>(rows: &[Row], tickets: &'a [Ticket]) -> (usize, Vec<&'a Ticket>) {
    let mut valid = Vec::new();
    let mut res = 0;

    for ticket in tickets {
        let mut v = true;
        for &field in ticket {
            if !rows.iter().any(|r| r.valid(field)) {
                res += field;
                v = false
            }
        }
        if v {
            valid.push(ticket);
        }
    }

    (res, valid)
}

fn generate_keys<'a>(rows: &'a [Row], tickets: &'_ [&Ticket]) -> Vec<&'a str> {
    let mut fixing_possibilities = HashMap::new();

    for row in rows {
        let entry = fixing_possibilities
            .entry(row.name)
            .or_insert_with(HashSet::new);

        for i in 0..tickets[0].len() {
            let mut valid = true;
            for ticket in tickets {
                if !row.valid(ticket[i]) {
                    valid = false;
                    break;
                }
            }
            if valid {
                entry.insert(i);
            }
        }
    }

    let mut fixed: HashMap<usize, &str> = HashMap::new();

    while !fixing_possibilities.is_empty() {
        let singulars = fixing_possibilities
            .iter_mut()
            .filter_map(|(k, v)| {
                if v.len() == 1 {
                    let index = *v.iter().next().expect("should have one value");
                    Some((*k, index))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for (label, index) in singulars {
            fixing_possibilities.remove(label);
            for possibilities in fixing_possibilities.values_mut() {
                possibilities.remove(&index);
            }
            fixed.insert(index, label);
        }
    }

    (0..rows.len())
        .map(|i| *fixed.get(&i).expect("exist"))
        .collect()
}

fn part2(rows: &[Row], valid: &[&Ticket], ticket: Ticket) -> usize {
    let keys = generate_keys(rows, &valid);

    keys.iter()
        .zip(ticket.iter())
        .filter_map(|(k, v)| {
            if k.starts_with("departure") {
                Some(v)
            } else {
                None
            }
        })
        .product()
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();

    let (rows, ticket, tickets) = load_tickets(INPUT);
    let (p1, valid) = part1(&rows, &tickets);
    let p2 = part2(&rows, &valid, ticket);

    (p1, p2, start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day16::{generate_keys, part1, Range, Row};

    #[test]
    fn test_part1() {
        let rows = vec![
            Row {
                name: "class",
                a: Range { min: 1, max: 3 },
                b: Range { min: 5, max: 7 },
            },
            Row {
                name: "row",
                a: Range { min: 6, max: 11 },
                b: Range { min: 33, max: 44 },
            },
            Row {
                name: "seat",
                a: Range { min: 13, max: 40 },
                b: Range { min: 45, max: 50 },
            },
        ];

        let tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];

        assert_eq!(part1(&rows, &tickets), (71, vec![&vec![7, 3, 47]]));
    }
    #[test]
    fn test_generation() {
        let rows = vec![
            Row {
                name: "class",
                a: Range { min: 0, max: 1 },
                b: Range { min: 4, max: 19 },
            },
            Row {
                name: "row",
                a: Range { min: 0, max: 5 },
                b: Range { min: 8, max: 19 },
            },
            Row {
                name: "seat",
                a: Range { min: 0, max: 13 },
                b: Range { min: 16, max: 19 },
            },
        ];

        let tickets = vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]];

        generate_keys(&rows, &(tickets.iter().collect::<Vec<_>>()));
    }
}
