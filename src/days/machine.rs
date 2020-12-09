use std::str::FromStr;

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

    pub fn swap_ins(&mut self, swap: usize) -> bool {
        self.instructions[swap] = match self.instructions[swap] {
            Instruction::Jmp(a) => Instruction::Nop(a),
            Instruction::Nop(a) => Instruction::Jmp(a),
            Instruction::Acc(_) => return false,
        };
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
