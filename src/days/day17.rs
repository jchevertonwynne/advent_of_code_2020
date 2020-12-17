use std::collections::{HashSet, HashMap};
use std::time::{Duration, Instant};
use fnv::{FnvHashMap, FnvBuildHasher, FnvHashSet};

const INPUT: &str = include_str!("../../files/17.txt");

fn load_world(input: &str) -> HashSet<(i8, i8, i8)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .filter_map(|(i, j, c)| {
            if c == '#' {
                Some((i as i8, j as i8, 0))
            } else {
                None
            }
        })
        .collect()
}

fn part1(mut world: HashSet<(i8, i8, i8)>) -> usize {
    let mut neighbour_count: HashMap<(i8, i8, i8), usize, FnvBuildHasher> = FnvHashMap::with_hasher(FnvBuildHasher::default());

    for &tile in &world {
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue
                    }
                    let tile: (i8, i8, i8) = (tile.0 + dx, tile.1 + dy, tile.2 + dz);
                    *neighbour_count.entry(tile).or_insert(0) += 1;
                }
            }
        }
    }

    for _ in 0..6 {
        let mut next_world = HashSet::new();
        let mut next_neighbour_count: HashMap<(i8, i8, i8), usize, FnvBuildHasher> = FnvHashMap::with_hasher(FnvBuildHasher::default());
        for (&tile, &neighbours) in &neighbour_count {
            if neighbours == 3 || (neighbours == 2 && world.contains(&tile)) {
                next_world.insert(tile);
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            if dx == 0 && dy == 0 && dz == 0 {
                                continue
                            }
                            let neighbour = (tile.0 + dx, tile.1 + dy, tile.2 + dz);
                            *next_neighbour_count.entry(neighbour).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
        neighbour_count = next_neighbour_count;
        world = next_world;
    }

    world.len()
}

fn part2(world: HashSet<(i8, i8, i8)>) -> usize {
    let mut world = {
        let mut w = FnvHashSet::with_capacity_and_hasher(world.len(), FnvBuildHasher::default());
        for (i, j, k) in world {
            w.insert((i, j, k, 0));
        }
        w
    };

    // let mut world = world
    //     .into_iter()
    //     .map(|(i, j, k)| (i, j, k, 0))
    //     .collect::<HashSet<(i8, i8, i8, i8)>>();

    let mut neighbour_count: HashMap<(i8, i8, i8, i8), usize, FnvBuildHasher> = FnvHashMap::with_hasher(FnvBuildHasher::default());

    for &tile in &world {
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue
                        }
                        let tile: (i8, i8, i8, i8) = (tile.0 + dx, tile.1 + dy, tile.2 + dz, tile.3 + dw);
                        *neighbour_count.entry(tile).or_insert(0) += 1;
                    }

                }
            }
        }
    }

    for _ in 0..6 {
        let mut next_world = FnvHashSet::with_hasher(FnvBuildHasher::default());
        let mut next_neighbour_count: HashMap<(i8, i8, i8, i8), usize, FnvBuildHasher> = FnvHashMap::with_hasher(FnvBuildHasher::default());
        for (&tile, &neighbours) in &neighbour_count {
            if neighbours == 3 || (neighbours == 2 && world.contains(&tile)) {
                next_world.insert(tile);
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            for dw in -1..=1 {
                                if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                    continue
                                }
                                let neighbour = (tile.0 + dx, tile.1 + dy, tile.2 + dz, tile.3 + dw);
                                *next_neighbour_count.entry(neighbour).or_insert(0) += 1;
                            }

                        }
                    }
                }
            }
        }
        neighbour_count = next_neighbour_count;
        world = next_world;
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
