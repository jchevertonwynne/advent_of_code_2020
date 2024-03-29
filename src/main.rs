use std::time::Duration;

mod days;

enum Runnable {
    Single(usize),
    Range(usize, usize),
    Repeat(usize, usize),
    AllRep(usize),
}

type RunFunc = fn() -> (String, String, Duration);

fn main() {
    let opts: Vec<RunFunc> = vec![
        days::day01::run,
        days::day02::run,
        days::day03::run,
        days::day04::run,
        days::day05::run,
        days::day06::run,
        days::day07::run,
        days::day08::run,
        days::day09::run,
        days::day10::run,
        days::day11::run,
        days::day12::run,
        days::day13::run,
        days::day14::run,
        days::day15::run,
        days::day16::run,
        days::day17::run,
        days::day18::run,
        days::day19::run,
        days::day20::run,
        days::day21::run,
        days::day22::run,
        days::day23::run,
        days::day24::run,
        days::day25::run,
    ];

    let args = std::env::args().skip(1);

    let mut actions = Vec::new();

    for arg in args {
        if arg == "!" {
            actions.push(Runnable::Single(opts.len()))
        } else if let Some(repeats) = arg.strip_prefix('@') {
            match repeats.parse::<usize>() {
                Ok(i) => actions.push(Runnable::AllRep(i)),
                _ => println!("illegal value for repeats: {}", repeats),
            }
        } else if arg == "." {
            actions.push(Runnable::Range(1, opts.len()))
        } else if arg.contains(':') {
            let parts = arg.split(':').collect::<Vec<_>>();
            if parts.len() != 2 {
                println!("too many parts: {}", arg);
                continue;
            }
            match parts[0].parse::<usize>() {
                Ok(i) if i > 0 && i <= opts.len() => match parts[1].parse::<usize>() {
                    Ok(repeats) => actions.push(Runnable::Repeat(i, repeats)),
                    _ => println!("illegal value for repeats: {}", arg),
                },
                _ => println!("illegal repeats arg: {}", arg),
            }
        } else if arg.contains('-') {
            let parts = arg.split('-').collect::<Vec<_>>();
            match (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                (Ok(a), Ok(b)) if a > 0 && a <= opts.len() && b > 0 && b <= opts.len() && a < b => {
                    actions.push(Runnable::Range(a, b))
                }
                _ => println!("invalid range: {}", arg),
            }
        } else {
            match arg.parse::<usize>() {
                Ok(i) if i > 0 && i <= opts.len() => {
                    actions.push(Runnable::Single(i));
                }
                _ => println!("invalid range: {}", arg),
            }
        }
    }

    let mut cum_duration = Duration::default();

    for action in actions {
        match action {
            Runnable::Single(i) => {
                println!();
                println!("day {}", i);
                let (p1, p2, duration) = opts[i - 1]();
                println!("    part 1: {}", p1);
                println!("    part 2: {}", p2);
                println!("    time:   {:?}", duration);
                cum_duration += duration;
            }
            Runnable::Range(first, last) => {
                for i in first..=last {
                    println!();
                    println!("day {}", i);
                    let (p1, p2, duration) = opts[i - 1]();
                    println!("    part 1: {}", p1);
                    println!("    part 2: {}", p2);
                    println!("    time:   {:?}", duration);
                    cum_duration += duration;
                }
            }
            Runnable::Repeat(i, repeats) => {
                let mut min = Duration::from_secs(100_000);
                let mut max = Duration::default();
                let mut running = Duration::default();
                println!();
                println!("day {} - {} runs", i, repeats);
                for rep in 0..repeats {
                    let (p1, p2, duration) = opts[i - 1]();
                    if rep == 0 {
                        println!("    part 1: {}", p1);
                        println!("    part 2: {}", p2);
                    }
                    running += duration;
                    min = Duration::min(min, duration);
                    max = Duration::max(max, duration);
                    cum_duration += duration;
                }
                println!("times:");
                println!("    minimum: {:?}", min);
                println!("    average: {:?}", running / repeats as u32);
                println!("    maximum: {:?}", max);
            }
            Runnable::AllRep(repeats) => {
                let mut total = Duration::default();
                for (i, t) in opts.iter().enumerate() {
                    println!();
                    println!("day {} - {} runs", i, repeats);
                    let mut min = Duration::from_secs(100_000);
                    for rep in 0..repeats {
                        let (p1, p2, duration) = t();
                        if rep == 0 {
                            println!("    part 1: {}", p1);
                            println!("    part 2: {}", p2);
                        }
                        min = Ord::min(min, duration);
                        total += duration;
                    }
                    println!("    fastest: {:?}", min);
                    cum_duration += min;
                }
            }
        }
    }

    println!();
    println!("total runtime: {:?}", cum_duration);
}
