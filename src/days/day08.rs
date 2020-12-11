use crate::common::machine::Machine;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/08.txt");

fn part1(machine: &mut Machine) -> i64 {
    machine.run_to_cycle();
    machine.acc()
}

fn part2(machine: &mut Machine) -> i64 {
    (0..machine.ins_count())
        .find_map(|r| {
            machine.reset();
            match machine.swap_ins(r) && !machine.run_to_cycle() {
                true => Some(machine.acc()),
                false => None,
            }
        })
        .expect("one solution")
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let mut machine = INPUT.parse::<Machine>().expect("please be a machine");
    let p1 = part1(&mut machine);
    let p2 = part2(&mut machine);
    let done = Instant::now();

    (p1 as usize, p2 as usize, done - start)
}

#[cfg(test)]
mod test {
    use crate::common::machine::Machine;
    use crate::days::day08::{part1, part2, INPUT};

    #[test]
    fn test_parts() {
        let mut machine = INPUT.parse::<Machine>().expect("please be a machine");
        assert_eq!(part1(&mut machine), 1671);
        machine.reset();
        assert_eq!(part2(&mut machine), 892);
    }
}
