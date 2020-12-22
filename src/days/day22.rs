use std::collections::{HashSet, VecDeque};
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/22.txt");

fn load_players(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut players = input.split("\n\n");
    let p1 = players.next().expect("should have a player");
    let p2 = players.next().expect("should have a player");

    let p1 = p1
        .split('\n')
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect::<VecDeque<_>>();
    let p2 = p2
        .split('\n')
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect::<VecDeque<_>>();

    (p1, p2)
}

fn part1(mut player_1: VecDeque<usize>, mut player_2: VecDeque<usize>) -> usize {
    while !player_1.is_empty() && !player_2.is_empty() {
        let a = player_1.pop_front().unwrap();
        let b = player_2.pop_front().unwrap();

        if a > b {
            player_1.push_back(a);
            player_1.push_back(b);
        } else {
            player_2.push_back(b);
            player_2.push_back(a);
        }
    }

    let winner = if player_1.is_empty() {
        player_2
    } else {
        player_1
    };
    (1..).zip(winner.iter().rev()).map(|(i, v)| i * v).sum()
}

enum Player {
    P1,
    P2,
}

fn recursive_combat_loop(player_1: &mut VecDeque<usize>, player_2: &mut VecDeque<usize>) -> Player {
    let mut seen_stacks: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();

    while !player_1.is_empty() && !player_2.is_empty() {
        if seen_stacks.contains(&(player_1.clone(), player_2.clone())) {
            return Player::P1;
        }

        seen_stacks.insert((player_1.clone(), player_2.clone()));

        let a = player_1.pop_front().unwrap();
        let b = player_2.pop_front().unwrap();

        if a > player_1.len() || b > player_2.len() {
            if a > b {
                player_1.push_back(a);
                player_1.push_back(b);
            } else {
                player_2.push_back(b);
                player_2.push_back(a);
            }
            continue;
        }

        let mut sub_p1 = player_1
            .iter()
            .take(a)
            .copied()
            .collect::<VecDeque<usize>>();
        let mut sub_p2 = player_2
            .iter()
            .take(b)
            .copied()
            .collect::<VecDeque<usize>>();

        match recursive_combat_loop(&mut sub_p1, &mut sub_p2) {
            Player::P1 => {
                player_1.push_back(a);
                player_1.push_back(b);
            }
            Player::P2 => {
                player_2.push_back(b);
                player_2.push_back(a);
            }
        }
    }

    if player_1.is_empty() {
        Player::P2
    } else {
        Player::P1
    }
}

fn part2(mut player_1: VecDeque<usize>, mut player_2: VecDeque<usize>) -> usize {
    let winner = match recursive_combat_loop(&mut player_1, &mut player_2) {
        Player::P1 => player_1,
        Player::P2 => player_2,
    };

    (1..).zip(winner.iter().rev()).map(|(i, v)| i * v).sum()
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let (a, b) = load_players(INPUT);
    let p1 = part1(a.clone(), b.clone());
    let p2 = part2(a, b);

    (p1, p2, start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day22::{load_players, part2};

    #[test]
    fn load_data() {
        let s = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let (p1, p2) = load_players(s);
        println!("{:?}", p1);
        println!("{:?}", p2);
    }

    #[test]
    fn recursive_game() {
        let s = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let (p1, p2) = load_players(s);
        assert_eq!(part2(p1, p2), 291);
    }
}
