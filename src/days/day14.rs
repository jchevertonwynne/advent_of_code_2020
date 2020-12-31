use fxhash::FxBuildHasher;
use std::collections::HashMap;
use std::convert::TryInto;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/14.txt");

#[derive(Debug, Copy, Clone)]
enum Mask {
    Unset,
    One,
    Zero,
}

#[derive(Debug)]
enum InputLine {
    Mask([Mask; 36]),
    Setting(usize, usize),
}

fn p2_applier(
    mem: &mut HashMap<usize, usize, FxBuildHasher>,
    orig_addr: usize,
    addr: usize,
    ind: usize,
    mask: &[Mask; 36],
    val: usize,
) {
    if ind == mask.len() {
        mem.insert(addr, val);
        return;
    }

    match mask[ind] {
        Mask::Unset => {
            p2_applier(mem, orig_addr, addr + (1 << ind), ind + 1, mask, val);
            p2_applier(mem, orig_addr, addr, ind + 1, mask, val);
        }
        Mask::One => p2_applier(mem, orig_addr, addr + (1 << ind), ind + 1, mask, val),
        Mask::Zero => p2_applier(
            mem,
            orig_addr,
            addr | (orig_addr & (1 << ind)),
            ind + 1,
            mask,
            val,
        ),
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut mask = [Mask::Unset; 36];
    let mut mem_p1: HashMap<usize, usize, FxBuildHasher> =
        HashMap::with_hasher(FxBuildHasher::default());
    let mut mem_p2: HashMap<usize, usize, FxBuildHasher> =
        HashMap::with_hasher(FxBuildHasher::default());
    input
        .lines()
        .map(|line| {
            if line.starts_with("mask") {
                let mask: [Mask; 36] = line[7..]
                    .chars()
                    .rev()
                    .map(|c| match c {
                        'X' => Mask::Unset,
                        '1' => Mask::One,
                        '0' => Mask::Zero,
                        _ => panic!("yoooooo: {}", c),
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("you can do this");
                InputLine::Mask(mask)
            } else {
                let mut parts = line.split(' ');
                let mem = parts.next().expect("go on");
                let mem = mem[4..mem.len() - 1].parse().expect("and parse");
                parts.next();
                let val = parts
                    .next()
                    .expect("have a val please")
                    .parse()
                    .expect("and parse");
                InputLine::Setting(mem, val)
            }
        })
        .for_each(|instruction| match instruction {
            InputLine::Mask(m) => mask = m,
            InputLine::Setting(addr, val) => {
                let apply = mask.iter().enumerate().fold(0, |acc, (i, m)| match m {
                    Mask::Unset => acc + (val & (1 << i)),
                    Mask::One => acc + (1 << i),
                    Mask::Zero => acc,
                });
                mem_p1.insert(addr, apply);
                p2_applier(&mut mem_p2, addr, 0, 0, &mask, val)
            }
        });

    (mem_p1.values().sum(), mem_p2.values().sum())
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let (p1, p2) = solve(INPUT);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day14::{solve, INPUT};

    #[test]
    fn test_actual() {
        assert_eq!(solve(INPUT), (15_018_100_062_885, 5_724_245_857_696));
    }
}
