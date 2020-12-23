use std::collections::VecDeque;
use std::str::Chars;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/18.txt");

#[derive(PartialEq, Debug)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

fn evaluate(line: &mut Chars, evaluator: fn(VecDeque<usize>, VecDeque<Op>) -> usize) -> usize {
    let mut vals = VecDeque::new();
    let mut ops = VecDeque::new();

    while let Some(char) = line.next() {
        match char {
            ' ' => continue,
            '*' => ops.push_back(Op::Mul),
            '+' => ops.push_back(Op::Add),
            '(' => vals.push_back(evaluate(line, evaluator)),
            ')' => break,
            c if '0' <= c && '9' >= c => vals.push_back(c as usize - '0' as usize),
            _ => panic!("yooo"),
        }
    }

    evaluator(vals, ops)
}

fn simple_eval(mut vals: VecDeque<usize>, mut ops: VecDeque<Op>) -> usize {
    while let Some(op) = ops.pop_front() {
        let a = vals.pop_front().expect("pls");
        let b = vals.pop_front().expect("pls");
        vals.push_front(op.apply(a, b));
    }
    vals.pop_front().expect("shoud be one")
}

fn advanced_eval(mut vals: VecDeque<usize>, mut ops: VecDeque<Op>) -> usize {
    while let Some(ind) = ops
        .iter()
        .enumerate()
        .find(|(_, op)| **op == Op::Add)
        .map(|(i, _)| i)
    {
        let a = vals.remove(ind).expect("pls");
        let b = vals.remove(ind).expect("pls");
        vals.insert(ind, a + b);
        ops.remove(ind);
    }
    vals.iter().product()
}

fn part1(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| evaluate(&mut line.chars(), simple_eval))
        .sum()
}

fn part2(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| evaluate(&mut line.chars(), advanced_eval))
        .sum()
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let lines = INPUT.lines().collect::<Vec<_>>();
    let p1 = part1(&lines);
    let p2 = part2(&lines);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day18::{advanced_eval, evaluate, simple_eval};

    #[test]
    fn test_example_simple() {
        let s = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(evaluate(&mut s.chars(), simple_eval), 71);

        let s = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(evaluate(&mut s.chars(), simple_eval), 51);
    }

    #[test]
    fn test_example_adv() {
        let s = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(evaluate(&mut s.chars(), advanced_eval), 51);

        let s = "2 * 3 + (4 * 5)";
        assert_eq!(evaluate(&mut s.chars(), advanced_eval), 46);

        let s = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(evaluate(&mut s.chars(), advanced_eval), 231);
    }

    #[test]
    fn test_hamzaan() {
        let s = "2 * 3 + 5 + ((7 * 8 + 8 * 5 * 8 * 9) + 2 * 4 + 2 + 8)
(8 + (9 * 9)) + (6 + 2 * 7)
2 + 5 + 4 + 8 + 4
8 + 2 + ((5 * 5 + 5) + (6 * 8 + 6 * 3) * 8) * 7 + 6
9 * ((2 + 5) + 2 + 8 * (5 + 7 + 5 * 3) * 6) + 9
9 + 6 + ((9 * 7 * 7) + 5 * 9 + 8 * (4 * 9 + 3 + 3) * 5) + 7 * 2
(3 * 3 + 5 + 6 + 5 * 5) + 8 * 4 * 4 + 2
7 * 2 + 3 + 9 * 4 + 4
5 + (2 + 9) + 5 + 3 + 8 * 6
9 * 3 + 5 + 3 * (6 * (4 * 8 + 6 * 9 * 9 + 6) * 7 + (9 * 2 * 3 + 7))
7 * 8 + 3 + (7 + 3 + 9 + 7 * 2) + (9 + 6 * 3 * 3)
((9 * 7) + 2) + 9 * (4 + 4)
((9 + 5) * 2 + (2 + 4 * 4 * 3 + 9 * 9)) + (7 + (7 * 6 + 4) + (9 + 7) * 9) + 8 + 9
4 + 7 * 3 + 2 * (7 + 2 * 8 + 9 * 3 * 3)
5 + 3
(2 + 3 * 5 * 6 * 2 * 2) + (8 * 8 + 3 * 3 + 7 * (7 + 3 + 9 * 8)) + 7 + 5
(7 + 3 * 5) * 7
4 * 6 + (6 + 7 + 9 * (3 + 4 * 4) + 3 + 2) * (6 + 8 * 9 * 6 * 5 + (2 * 5 * 8 + 8 + 5 * 2))
8 * (7 + 5 * 8 * 7 + 6) * ((4 + 5) * 9 + 9 * 2 * 7) * 5 + 8
6 + 8 * 5
(8 * (7 * 4) + 5 * 9 * 9 + 3) * 7
8 + 5 + 7 + (8 + (6 + 5) + 7 + (7 * 9 * 2 * 6 + 4) * 8) * 2
((4 + 3 * 9 + 9) + 4 * (9 + 2 + 4 + 8 + 9) + 5 + 7 * 4) + 6 * 9
9 + (9 * 8 * 3) * (8 * 6 * 7 * 8 * 6)
5 * 7 * (9 * 3 + 6 + 8 + 8 + 5)
2 + (2 + 7 * (5 * 7 * 3 + 2 * 7 * 8) + 7 + (5 + 8 * 5)) * 8 * 3 * 3 + ((6 * 5 * 8 + 8 + 9) + 6 * (8 + 3) * 6)
9 * 5 * 9 * 8
6 + 5 + 7 + 2 + (8 + (5 * 5 * 7 * 3 + 7 + 7) + 2 + 7)
4 + 4 * 2 + 3 * 7 + 6
4 + (5 + 3 + 2 * 8 * 5) + 2
2 * 2 * 8 + (6 + 9 * 4 * 7 * (4 + 7 * 7 * 2) + 2) + 9
4 * 7 * 8 + (3 * (9 * 2 + 4 * 4 * 5 * 5) + 5 * 2 * 7 + 7) + 8 + 5
(8 + (7 + 9)) * 6
9 + ((8 * 7 + 5 * 3) + 7 + (6 * 4 * 6 + 8)) * 5
3 + (6 + 5 * 7 + 4 * 2) * (3 * 3 + 7 * 6 + 4 * (6 + 7 * 3)) * (4 + 8 * 9)
7 * (7 + (4 * 3) + 4 + 5 * 4)
8 * 2 * (9 * 7 + 5 + 8) + 4 * 8 + (3 * 3)
7 * (3 + 2 * 3) + ((2 * 8) * 4) * 8
((8 + 3 + 4 + 7 * 5 * 6) + (3 + 2) * (8 + 6 * 9 * 4 + 9 + 2)) + 3 + 8 * (8 + (4 + 9 * 2 + 8 + 6 + 9) * 9 * 3 * 3 + 3) + 9
4 * (7 * 4 * 8 * 5) + 7 * 8 * (2 * 4 * 5 * 4)
(3 + 8 + 6 * 6 * 4) * (4 * 6 + 8 + 3 * 6) * 5
2 + (3 + (6 + 9 + 6 * 3) * 7 + (9 + 7 + 8)) + 9 * 7
9 + 7 * (4 * 8 + 3 * 2 * 9 + 4) + ((6 * 8) + 3 + 7 + 7 * 8 * (9 + 5)) * 9 + ((3 * 9 * 8 * 8) + (5 + 2) + 2 * 4 * 3 + 8)
6 * 3 + 3 * (8 * 7 + (2 * 8 + 4 + 6 + 5 * 6) + 3 + 3 * (9 * 9 * 8 + 2 * 5)) + (3 + (3 + 5 + 6) + 8 + 9 * 4) + 8
9 * ((7 * 6) * 9 * 2) * ((8 * 9 * 3 + 8 * 2) * 6 + 8 + 9 * 5 + 3) + (5 * 4) * (7 + 4) * 7
(4 * 7 + 8 * 9) + (7 + (3 + 9 * 2 * 9 * 4) * 9 * 3 * 4 * 7) * 4 * 6 + 2
6 + 9 * 6 + (2 + 9 * 4) * 8
3 * (6 * 5 + (7 * 6 * 5 * 8) + 7 * 5 * 4) + (3 * (6 * 5 + 7 * 9) + (4 * 9 + 2 * 3) * 7 * 8 * 3) + 9 + 5 + 9
3 + (2 + (6 * 3 * 8 + 3) + 4) * (8 * 9) * 7 + 6
(2 * (7 * 9)) * 6 + 7 * 2 + 9 * 4
((5 + 4 * 2) + (3 * 2 * 6 + 3 + 5 * 5) * 4 + (2 + 5 * 9 * 8 + 3 * 5) * (7 * 2 * 8 * 2) + (5 * 4)) + 7 * 8 + 9
(8 + 4 + (5 * 4 * 6) * 2 * 8) * (6 + (9 * 4 * 5) * 7 * 5 * 4) + 3 + ((2 + 2) + 7 + 5 * 7 * 2 + 4) * 5 + (9 * 3 * (9 + 3 * 4 * 8 * 7 + 9))
6 * 7 * 3 * (7 * 2 + (6 * 4 * 5) * (5 * 3 * 8 + 8 + 3 + 9)) * (7 * 9 + 7 * 5)
4 + 2 + (3 * (9 + 7) * 6)
(8 + (8 * 4)) + (7 + 3 * 5) + 4 + 4 + 7
(7 + 7 * 4 * 4 * 4) * 4 + 3".lines();
        s.for_each(|l| println!("{}", evaluate(&mut l.chars(), simple_eval)));
    }
}
