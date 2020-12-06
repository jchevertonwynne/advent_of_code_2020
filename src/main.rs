#![feature(try_trait)]

mod days;

fn main() {
    let opts = [
        || days::day1::run(),
        || days::day2::run(),
        || days::day3::run(),
        || days::day4::run(),
        || days::day5::run(),
        || days::day6::run(),
    ];

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    for arg in args {
        match arg.parse::<usize>() {
            Ok(i) => {
                if i == 0 {
                    continue;
                }
                match opts.get(i - 1) {
                    Some(opt) => {
                        println!("--------------------");
                        println!("{}", format!("day {}", i));
                        opt();
                    }
                    None => println!("invalid option {}", arg),
                }
            }
            _ => println!("illegal arg: {}", arg),
        }
    }
}
