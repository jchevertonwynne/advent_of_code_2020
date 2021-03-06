use std::str::FromStr;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/08.txt");

#[derive(Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

#[derive(Clone)]
pub struct Machine {
    i: i64,
    instructions: Vec<Instruction>,
    acc: i64,
    swapped: Option<(Instruction, usize)>,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 5 {
            return Err("input too short".to_string());
        }
        let num = &s[3..].trim();
        let i = num[1..].parse::<i64>().map_err(|err| err.to_string())?;
        let i = match num.starts_with('+') {
            true => i,
            false => -i,
        };
        match &s[..3] {
            "nop" => Ok(Instruction::Nop(i)),
            "acc" => Ok(Instruction::Acc(i)),
            "jmp" => Ok(Instruction::Jmp(i)),
            _ => Err("invalid instruction".to_string()),
        }
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        for instruction in s.lines() {
            instructions.push(instruction.parse()?);
        }
        Ok(Machine {
            i: 0,
            instructions,
            acc: 0,
            swapped: None,
        })
    }
}

impl Machine {
    pub fn acc(&self) -> i64 {
        self.acc
    }

    pub fn ins_count(&self) -> usize {
        self.instructions.len()
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.i = 0;
        let mut replace: Option<(Instruction, usize)> = None;
        std::mem::swap(&mut replace, &mut self.swapped);
        if let Some((ins, i)) = replace {
            self.instructions[i] = ins;
        }
    }

    pub fn swap_ins(&mut self, ind: usize) -> bool {
        let mut new_ins = match self.instructions[ind] {
            Instruction::Jmp(a) => Instruction::Nop(a),
            Instruction::Nop(a) => Instruction::Jmp(a),
            Instruction::Acc(_) => return false,
        };
        std::mem::swap(&mut new_ins, &mut self.instructions[ind]);
        self.swapped = Some((new_ins, ind));
        true
    }

    pub fn run_to_cycle(&mut self) -> bool {
        let mut seen = vec![0; self.instructions.len()];
        loop {
            self.iterate();
            if self.i >= self.instructions.len() as i64 {
                return false;
            }

            seen[self.i as usize] += 1;
            if seen[self.i as usize] == 2 {
                return true;
            }
        }
    }

    fn iterate(&mut self) {
        match self.instructions[self.i as usize] {
            Instruction::Acc(i) => {
                self.acc += i;
                self.i += 1;
            }
            Instruction::Jmp(i) => self.i += i,
            Instruction::Nop(_) => self.i += 1,
        }
    }
}

fn part1(machine: &mut Machine) -> usize {
    machine.run_to_cycle();
    machine.acc() as usize
}

fn part2(machine: &mut Machine) -> usize {
    (0..machine.ins_count())
        .find_map(|r| {
            machine.reset();
            match machine.swap_ins(r) && !machine.run_to_cycle() {
                true => Some(machine.acc()),
                false => None,
            }
        })
        .expect("one solution") as usize
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let mut machine = INPUT.parse::<Machine>().expect("please be a machine");
    let p1 = part1(&mut machine);
    let p2 = part2(&mut machine);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod test {
    use crate::days::day08::{part1, part2, Machine, INPUT};

    #[test]
    fn test_parts() {
        let mut machine = INPUT.parse::<Machine>().expect("please be a machine");
        assert_eq!(part1(&mut machine), 1_671);
        machine.reset();
        assert_eq!(part2(&mut machine), 892);
    }
}
