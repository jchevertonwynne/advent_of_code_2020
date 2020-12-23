use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/12.txt");

enum Instruction {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Forward(i64),
    Left(i64),
    Right(i64),
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

fn load_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let dist = line[1..].parse().expect("you have a value");
            match line.chars().next().expect("non zero size") {
                'N' => Instruction::North(dist),
                'S' => Instruction::South(dist),
                'E' => Instruction::East(dist),
                'W' => Instruction::West(dist),
                'F' => Instruction::Forward(dist),
                'L' => Instruction::Left(dist / 90),
                'R' => Instruction::Right(dist / 90),
                _ => panic!("lol"),
            }
        })
        .collect()
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut direction = Direction::East;
    let mut x = 0i64;
    let mut y = 0i64;

    for instruction in instructions {
        match *instruction {
            Instruction::North(i) => y += i,
            Instruction::South(i) => y -= i,
            Instruction::East(i) => x += i,
            Instruction::West(i) => x -= i,
            Instruction::Forward(i) => match direction {
                Direction::North => y += i,
                Direction::South => y -= i,
                Direction::East => x += i,
                Direction::West => x -= i,
            },
            Instruction::Left(l) => {
                for _ in 0..l {
                    direction = direction.left();
                }
            }
            Instruction::Right(r) => {
                for _ in 0..r {
                    direction = direction.right();
                }
            }
        }
    }

    (x.abs() + y.abs()) as usize
}

fn part2(instructions: &[Instruction]) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut wx = 10i64;
    let mut wy = 1i64;

    for instruction in instructions {
        match *instruction {
            Instruction::North(i) => wy += i,
            Instruction::South(i) => wy -= i,
            Instruction::East(i) => wx += i,
            Instruction::West(i) => wx -= i,
            Instruction::Forward(i) => {
                x += i * wx;
                y += i * wy;
            }
            Instruction::Left(l) => {
                for _ in 0..l {
                    std::mem::swap(&mut wx, &mut wy);
                    wx = -wx;
                }
            }
            Instruction::Right(r) => {
                for _ in 0..r {
                    std::mem::swap(&mut wx, &mut wy);
                    wy = -wy;
                }
            }
        }
    }
    (x.abs() + y.abs()) as usize
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let instructions = load_instructions(INPUT);
    let p1 = part1(&instructions);
    let p2 = part2(&instructions);
    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day12::{load_instructions, part1, part2};

    #[test]
    fn test_parts() {
        let s = "F10
N3
F7
R90
F11";
        let ins = load_instructions(s);
        assert_eq!(part1(&ins), 25);
        assert_eq!(part2(&ins), 286);
    }
}
