use crate::common::machine::Machine;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/08.txt");

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
    use crate::common::machine::Machine;
    use crate::days::day08::{part1, part2, INPUT};

    #[test]
    fn test_parts() {
        let mut machine = INPUT.parse::<Machine>().expect("please be a machine");
        assert_eq!(part1(&mut machine), 1_671);
        machine.reset();
        assert_eq!(part2(&mut machine), 892);
    }
}
