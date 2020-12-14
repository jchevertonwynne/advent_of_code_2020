use std::collections::HashMap;
use std::convert::TryInto;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/14.txt");

lazy_static! {
    static ref STATIC_INPUT: Vec<InputLine> = load_program(INPUT);
}

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

fn load_program(input: &str) -> Vec<InputLine> {
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
        .collect()
}

fn part1(instructions: &[InputLine]) -> usize {
    let mut mask = [Mask::Unset; 36];
    let mut mem = HashMap::new();
    for instruction in instructions {
        match *instruction {
            InputLine::Mask(m) => mask = m,
            InputLine::Setting(addr, val) => {
                let mut apply = 0;
                for (i, m) in mask.iter().enumerate() {
                    match m {
                        Mask::Unset => apply += val & (1 << i),
                        Mask::One => apply += 1 << i,
                        Mask::Zero => (),
                    }
                }
                mem.insert(addr, apply);
            }
        }
    }

    mem.values().sum()
}

fn applier(
    mem: &mut HashMap<usize, usize>,
    orig_addr: usize,
    addr: usize,
    ind: usize,
    mask: &[Mask; 36],
    val: usize,
) {
    if ind == mask.len() {
        mem.insert(addr, val);
        return
    }

    match mask[ind] {
        Mask::Unset => {
            applier(mem, orig_addr, addr + (1 << ind), ind + 1, mask, val);
            applier(mem, orig_addr, addr, ind + 1, mask, val);
        }
        Mask::One => applier(mem, orig_addr, addr + (1 << ind), ind + 1, mask, val),
        Mask::Zero => applier(
            mem,
            orig_addr,
            addr | (orig_addr & (1 << ind)),
            ind + 1,
            mask,
            val,
        ),
    }
}

fn part2(instructions: &[InputLine]) -> usize {
    let mut mask = [Mask::Unset; 36];
    let mut mem: HashMap<usize, usize> = HashMap::new();
    for instruction in instructions {
        match *instruction {
            InputLine::Mask(m) => mask = m,
            InputLine::Setting(addr, val) => applier(&mut mem, addr, 0, 0, &mask, val),
        }
    }

    mem.values().sum()
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let p1 = part1(&STATIC_INPUT);
    let p2 = part2(&STATIC_INPUT);

    (p1, p2, start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day14::{applier, load_program, part1, InputLine, INPUT, part2};
    use std::collections::HashMap;

    #[test]
    fn test_actual() {
        let ins = load_program(INPUT);
        assert_eq!(part1(&ins), 15018100062885);
        assert_eq!(part2(&ins), 5724245857696);
    }

    #[test]
    fn test_example() {
        let s = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let ins = load_program(s);
        assert_eq!(part1(&ins), 165);
    }

    #[test]
    fn test_applier() {
        let s = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let ins = load_program(s);
        let mask = match ins[0] {
            InputLine::Mask(m) => m,
            InputLine::Setting(_, _) => panic!("pls dont be this"),
        };

        let (addr, val) = match ins[1] {
            InputLine::Mask(_) => panic!("pls dnt be this"),
            InputLine::Setting(addr, val) => (addr, val),
        };

        let mut mem: HashMap<usize, usize> = HashMap::new();
        applier(&mut mem, addr, 0, 0, &mask, val);

        println!("{:?}", mem);
    }
}
