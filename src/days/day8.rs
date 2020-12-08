use crate::days::machine::Machine;
use rayon::prelude::*;
use std::time::Instant;

const INPUT: &str = include_str!("../../files/08.txt");

fn part1(mut machine: Machine) -> i64 {
    machine.run_to_cycle();
    machine.acc()
}

fn part2(machine: Machine) -> i64 {
    (0..machine.ins_count())
        .into_par_iter()
        .find_map_any(|r| {
            let mut m = machine.clone();
            match m.swap_ins(r) && m.run_to_cycle() {
                true => None,
                false => Some(m.acc()),
            }
        })
        .expect("one solution")
}

pub fn run() {
    let start = Instant::now();
    let machine = INPUT.parse::<Machine>().expect("please be a machine");
    let data_loaded = Instant::now();
    let p1 = part1(machine.clone());
    let done_part1 = Instant::now();
    let p2 = part2(machine.clone());
    let done_part2 = Instant::now();

    println!("    part 1: {}", p1);
    println!("    part 2: {}", p2);
    println!("time taken:");
    println!("    total: {:?}", done_part2.duration_since(start));
    println!("    data load: {:?}", data_loaded.duration_since(start));
    println!("    part 1: {:?}", done_part1.duration_since(data_loaded));
    println!("    part 2: {:?}", done_part2.duration_since(done_part1));
}
