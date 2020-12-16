use std::time::{Duration, Instant};
use std::collections::{HashMap, HashSet};

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

fn part1(rows: &[Row], tickets: &[Ticket]) -> usize {
    let mut res = 0;

    for ticket in tickets {
        for &field in ticket {
            if !rows.iter().any(|r| r.valid(field)) {
                res += field;
            }
        }
    }

    res
}

fn generate_keys<'a>(rows: &'a [Row], tickets: &'_ [Ticket]) -> Vec<&'a str> {
    let mut fixing_possibilities = HashMap::new();

    for row in rows {
        let entry = fixing_possibilities.entry(row.name).or_insert(Vec::new());


        for i in 0..tickets[0].len() {
            let mut valid = true;
            for ticket in tickets {
                if !row.valid(ticket[i]) {
                    valid = false;
                    println!("entry {:?} - {} not valid for rule  {:?}", ticket, ticket[i], row);
                    break
                }
            }
            if valid {
                entry.push(i);
            }
        }
    }

    println!("{:?}", fixing_possibilities);

    while fixing_possibilities.values().any(|v| v.len() > 1) {
        let single = fixing_possibilities.iter().filter(|(k, v)| v.len() == 1).next().expect("to solve pls");
        let to_remove = single.1[0];
        let keep = *single.0;
        fixing_possibilities.iter_mut().for_each(|(&key, val)| {
            if key != keep {
                *val = val.iter().filter_map(|&v| if v != to_remove { Some(v) } else {None }).collect()
            }
        });
    }

    println!("{:?}", fixing_possibilities);

    Vec::new()
}

fn part2(rows: &[Row], tickets: &[Ticket], ticket: Ticket) -> usize {
    let mut valid = Vec::new();

    for ticket in tickets {
        if ticket.iter().all(|&field| rows.iter().any(|r| r.valid(field))) {
            valid.push(ticket.clone());
        }
    }

    let mut fixing_possibilities = HashMap::new();

    for row in rows {
        let entry = fixing_possibilities.entry(row.name).or_insert(Vec::new());
        for i in 0..tickets[0].len() {
            if tickets.iter().all(|t| row.valid(t[i])) {
                entry.push(i);
            }
        }
    }

    println!("{:?}", fixing_possibilities);

    0
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();

    let (rows, ticket, tickets) = load_tickets(INPUT);

    println!("{:?}", rows);

    let p1 = part1(&rows, &tickets);
    let p2 = part2(&rows, &tickets, ticket);

    (p1, p2, start.elapsed())
}


#[cfg(test)]
mod tests {
    use crate::days::day16::{Row, Range, part1, generate_keys};

    #[test]
    fn test_part1() {
        let c = "class";
        let r = "row";
        let s = "seat";

        let rows = vec![
            Row{
                name: c,
                a: Range { min: 1, max: 3 },
                b: Range { min: 5, max: 7 }
            },
            Row{
                name: r,
                a: Range { min: 6, max: 11 },
                b: Range { min: 33, max: 44 }
            },
            Row{
                name: s,
                a: Range { min: 13, max: 40 },
                b: Range { min: 45, max: 50 }
            },
        ];

        let tickets = vec![
            vec![7,3,47],
            vec![40,4,50],
            vec![55,2,20],
            vec![38,6,12]
        ];

        assert_eq!(part1(&rows, &tickets), 71);
    }
    #[test]
    fn test_generation() {
        let c = "class";
        let r = "row";
        let s = "seat";

        let rows = vec![
            Row{
                name: c,
                a: Range { min: 0, max: 1 },
                b: Range { min: 4, max: 19 }
            },
            Row{
                name: r,
                a: Range { min: 0, max: 5 },
                b: Range { min: 8, max: 19 }
            },
            Row{
                name: s,
                a: Range { min: 0, max: 13 },
                b: Range { min: 16, max: 19 }
            },
        ];

        let tickets = vec![
            vec![3, 9, 18],
            vec![15, 1, 5],
            vec![5, 14, 9],
        ];

        generate_keys(&rows, &tickets);
    }
}