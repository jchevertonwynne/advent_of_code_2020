use crate::days::machine::Machine;
use std::time::Instant;

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

pub fn run() {
    let start = Instant::now();
    let mut machine = INPUT.parse::<Machine>().expect("please be a machine");
    let data_loaded = Instant::now();
    let p1 = part1(&mut machine);
    let done_part1 = Instant::now();
    let p2 = part2(&mut machine);
    let done_part2 = Instant::now();

    println!("    part 1: {}", p1);
    println!("    part 2: {}", p2);
    println!("time taken:");
    println!("    total: {:?}", done_part2.duration_since(start));
    println!("    data load: {:?}", data_loaded.duration_since(start));
    println!("    part 1: {:?}", done_part1.duration_since(data_loaded));
    println!("    part 2: {:?}", done_part2.duration_since(done_part1));
}

#[cfg(test)]
mod test {
    use crate::days::day8::{part1, part2, INPUT};
    use crate::days::machine::Machine;

    #[test]
    fn test_parts() {
        let mut machine = INPUT.parse::<Machine>().expect("please be a machine");
        assert_eq!(part1(&mut machine), 1671);
        machine.reset();
        assert_eq!(part2(&mut machine), 892);
    }
}
