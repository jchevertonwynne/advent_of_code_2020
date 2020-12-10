use std::time::Instant;

mod days;

fn main() {
    let opts = [
        || days::day1::run(),
        || days::day2::run(),
        || days::day3::run(),
        || days::day4::run(),
        || days::day5::run(),
        || days::day6::run(),
        || days::day7::run(),
        || days::day8::run(),
        || days::day9::run(),
        || days::day10::run(),
    ];

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let start = Instant::now();
    for arg in args {
        if arg.contains(':') {
            let parts = arg.split(':').collect::<Vec<_>>();
            match parts[0].parse::<usize>() {
                Ok(i) => match parts.len() {
                    1 => {
                        println!("-------------------------");
                        println!("{}", format!("day {}", i));
                        opts[i - 1]();
                    }
                    2 => match parts[1].parse::<usize>() {
                        Ok(j) => {
                            for _ in 0..j {
                                println!("-------------------------");
                                println!("{}", format!("day {} run {}", i, j));
                                opts[i - 1]();
                            }
                        }
                        _ => println!("illegal quantifier: {}", arg),
                    },
                    _ => panic!("what"),
                },
                _ => println!("illegal arg: {}", arg),
            }
        } else if arg.contains('-') {
            let parts = arg.split('-').collect::<Vec<_>>();
            match (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                (Ok(a), Ok(b)) => {
                    for i in a..=b {
                        println!("-------------------------");
                        println!("{}", format!("day {}", i));
                        opts[i - 1]();
                    }
                }
                _ => println!("invalid range: {}", arg),
            }
        } else {
            match arg.parse::<usize>() {
                Ok(i) => {
                    println!("-------------------------");
                    println!("{}", format!("day {}", i));
                    opts[i - 1]();
                }
                _ => println!("invalid range: {}", arg),
            }
        }
    }
    let end = Instant::now();
    println!("{:?}", end - start);
}
