use std::time::{Duration, Instant};

fn find_loops(pk: usize) -> usize {
    let modulo = 20201227;

    let mut res = 0;
    let mut v = 1;
    while v != pk {
        v *= 7;
        v %= modulo;
        res += 1;
    }

    res
}

fn transform(subject: usize, loops: usize) -> usize {
    let mut r = 1;

    for _ in 0..loops {
        r *= subject;
        r %= 20201227;
    }

    r
}

fn part1(pk_a: usize, pk_b: usize) -> usize {
    let b_loops = find_loops(pk_b);
    transform(pk_a, b_loops)
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let card_pk = 12232269;
    let door_pk = 19452773;
    let p1 = part1(card_pk, door_pk);

    (
        p1.to_string(),
        "there is no part 2 lol".to_string(),
        start.elapsed(),
    )
}
