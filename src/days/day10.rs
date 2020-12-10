const INPUT: &str = include_str!("../../files/10.txt");

fn load_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().expect("int val pls")).collect()
}

fn part1(jolts: &[usize]) -> usize {
    let mut ones = 0;
    let mut threes = 0;
    let mut curr = 0;

    for &jolt in jolts {
        let diff = jolt - curr;
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            _ => ()
        }
        curr = jolt
    }

    threes += 1;

    ones * threes
}


/*
0 1 2 3 -> 6
     1
   1
 2
3
*/

fn part2(jolts: &[usize]) -> usize {
    let mut ways_to_next = Vec::with_capacity(jolts.len() + 1);
    ways_to_next.push(jolts.iter().take_while(|&j| *j <= 3).count());

    for i in 0..jolts.len() - 1 {
        ways_to_next.push(jolts[i + 1..].iter().take_while(|j| *j - jolts[i] <= 3).count());
    }

    let mut res = Vec::with_capacity(ways_to_next.len());
    res.push(1);

    println!("{:?}", ways_to_next);

    let mut i = ways_to_next.len() - 3;
    loop {
        let opts = &ways_to_next[i + 1..i + 1 + ways_to_next[i]];
        println!("{}, {:?}", ways_to_next[i], opts);
        res.push(2usize.pow(*opts.iter().max().expect("must have next") as u32));

        if i == 0 {
            break
        }

        i -= 1
    }


    println!("{:?}", res);

    *res.last().expect("pls")

}

pub fn run() {
    let mut jolts = load_input(INPUT);
    jolts.sort_unstable();
    println!("0 {:?}", jolts);
    let p1 = part1(&jolts);
    let p2 = part2(&jolts);
    println!("    part 1: {}", p1);
    println!("    part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use crate::days::day10::{load_input, part2};

    #[test]
    fn test_part2() {
        let mut nums = load_input("16
10
15
5
1
11
7
19
6
12
4");
        nums.sort_unstable();
        println!("0 {:?}", nums);
        assert_eq!(part2(&nums), 8);

        println!("------------");

        let mut nums = load_input("28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3");
        nums.sort_unstable();
        println!("{:?}", nums);
        assert_eq!(part2(&nums), 19208);
    }
}