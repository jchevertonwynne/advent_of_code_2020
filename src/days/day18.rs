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

fn load_input(input: &str) -> Vec<&str> {
    input.lines().collect()
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

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let lines = load_input(INPUT);
    let p1 = part1(&lines);
    let p2 = part2(&lines);

    (p1, p2, start.elapsed())
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
}
