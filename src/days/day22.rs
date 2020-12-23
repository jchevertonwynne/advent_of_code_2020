use std::collections::{HashSet, VecDeque};
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/22.txt");

fn load_players(input: &str) -> (VecDeque<i8>, VecDeque<i8>) {
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

fn part1(mut player_1: VecDeque<i8>, mut player_2: VecDeque<i8>) -> usize {
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
    (1usize..)
        .zip(winner.iter().rev())
        .map(|(i, v)| (i * *v as usize))
        .sum()
}

enum Player {
    Player1,
    Player2,
}

fn recursive_combat_loop(player_1: &mut VecDeque<i8>, player_2: &mut VecDeque<i8>) -> Player {
    let mut seen_stacks: HashSet<(VecDeque<i8>, VecDeque<i8>)> = HashSet::new();

    while !player_1.is_empty() && !player_2.is_empty() {
        if seen_stacks.contains(&(player_1.clone(), player_2.clone())) {
            return Player::Player1;
        }

        seen_stacks.insert((player_1.clone(), player_2.clone()));

        let a = player_1.pop_front().unwrap();
        let b = player_2.pop_front().unwrap();

        if a > player_1.len() as i8 || b > player_2.len() as i8 {
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
            .take(a as usize)
            .copied()
            .collect::<VecDeque<i8>>();
        let mut sub_p2 = player_2
            .iter()
            .take(b as usize)
            .copied()
            .collect::<VecDeque<i8>>();

        match recursive_combat_loop(&mut sub_p1, &mut sub_p2) {
            Player::Player1 => {
                player_1.push_back(a);
                player_1.push_back(b);
            }
            Player::Player2 => {
                player_2.push_back(b);
                player_2.push_back(a);
            }
        }
    }

    if player_1.is_empty() {
        Player::Player2
    } else {
        Player::Player1
    }
}

fn part2(mut player_1: VecDeque<i8>, mut player_2: VecDeque<i8>) -> usize {
    let winner = match recursive_combat_loop(&mut player_1, &mut player_2) {
        Player::Player1 => player_1,
        Player::Player2 => player_2,
    };

    (1usize..)
        .zip(winner.iter().rev())
        .map(|(i, v)| (i * *v as usize))
        .sum()
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let (a, b) = load_players(INPUT);
    let p1 = part1(a.clone(), b.clone());
    let p2 = part2(a, b);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day22::{load_players, part1, part2};

    #[test]
    fn play_games() {
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
        assert_eq!(part1(p1.clone(), p2.clone()), 306);
        assert_eq!(part2(p1, p2), 291);
    }
}
