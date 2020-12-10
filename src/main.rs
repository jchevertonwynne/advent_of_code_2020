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
        match arg.parse::<usize>() {
            Ok(0) => (),
            Ok(i) => match opts.get(i - 1) {
                Some(opt) => {
                    println!("-------------------------");
                    println!("{}", format!("day {}", i));
                    opt();
                }
                None => println!("invalid option {}", arg),
            },
            _ => println!("illegal arg: {}", arg),
        }
    }
    let end = Instant::now();
    println!("{:?}", end - start);
}
