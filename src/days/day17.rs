use std::collections::HashSet;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/17.txt");

fn load_world(input: &str) -> HashSet<(i64, i64, i64)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .filter_map(|(i, j, c)| {
            if c == '#' {
                Some((i as i64, j as i64, 0))
            } else {
                None
            }
        })
        .collect()
}

fn part1(mut world: HashSet<(i64, i64, i64)>) -> usize {
    for _ in 0..6 {
        let mut next = HashSet::new();
        for &(i, j, k) in &world {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        let tile = (i + dx, j + dy, k + dz);
                        let living_neighbours = (-1..=1)
                            .flat_map(move |di| {
                                (-1..=1).flat_map(move |dj| (-1..=1).map(move |dk| (di, dj, dk)))
                            })
                            .map(move |(di, dj, dk)| (tile.0 + di, tile.1 + dj, tile.2 + dk))
                            .filter(|t| t != &tile && world.contains(t))
                            .count();

                        match world.contains(&tile) {
                            true => {
                                if living_neighbours == 2 || living_neighbours == 3 {
                                    next.insert(tile);
                                }
                            }
                            false => {
                                if living_neighbours == 3 {
                                    next.insert(tile);
                                }
                            }
                        }
                    }
                }
            }
        }
        world = next;
    }
    world.len()
}

fn part2(world: HashSet<(i64, i64, i64)>) -> usize {
    let mut world = world
        .into_iter()
        .map(|(i, j, k)| (i, j, k, 0))
        .collect::<HashSet<(i64, i64, i64, i64)>>();
    for _ in 0..6 {
        let mut next = HashSet::new();
        for &(i, j, k, l) in &world {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        for dw in -1..=1 {
                            let tile = (i + dx, j + dy, k + dz, l + dw);
                            let living_neighbours = (-1..=1)
                                .flat_map(move |di| {
                                    (-1..=1).flat_map(move |dj| {
                                        (-1..=1).flat_map(move |dk| {
                                            (-1..=1).map(move |dl| (di, dj, dk, dl))
                                        })
                                    })
                                })
                                .map(move |(di, dj, dk, dl)| {
                                    (tile.0 + di, tile.1 + dj, tile.2 + dk, tile.3 + dl)
                                })
                                .filter(|t| t != &tile && world.contains(t))
                                .count();

                            match world.contains(&tile) {
                                true => {
                                    if living_neighbours == 2 || living_neighbours == 3 {
                                        next.insert(tile);
                                    }
                                }
                                false => {
                                    if living_neighbours == 3 {
                                        next.insert(tile);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        world = next;
    }
    world.len()
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();

    let world = load_world(INPUT);
    let p1 = part1(world.clone());
    let p2 = part2(world.clone());

    (p1, p2, start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day17::{load_world, part1, part2};
    use std::collections::HashSet;

    #[test]
    fn test_example() {
        let s = ".#.
..#
###";
        let world = load_world(s);
        let expected = vec![(0, 1, 0), (1, 2, 0), (2, 0, 0), (2, 1, 0), (2, 2, 0)]
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(world, expected);

        assert_eq!(part1(world.clone()), 112);
        assert_eq!(part2(world.clone()), 848);
    }
}
