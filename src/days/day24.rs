use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/24.txt");
const HEX_OFFSETS: [(i16, i16); 6] = [(0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1)];

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn offset(&self) -> (i16, i16) {
        match self {
            Direction::East => (1, 0),
            Direction::SouthEast => (0, -1),
            Direction::SouthWest => (-1, -1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (0, 1),
            Direction::NorthEast => (1, 1),
        }
    }
}

type Path = Vec<Direction>;

#[derive(PartialEq)]
enum Tile {
    Black,
    White,
}

impl Tile {
    fn swap(&mut self) {
        *self = match self {
            Tile::Black => Tile::White,
            Tile::White => Tile::Black,
        }
    }
}

fn load_paths(input: &str) -> Vec<Path> {
    input
        .lines()
        .map(|mut line| {
            let mut res = Vec::new();
            while !line.is_empty() {
                if line.starts_with('e') {
                    res.push(Direction::East);
                    line = &line[1..];
                } else if line.starts_with('w') {
                    res.push(Direction::West);
                    line = &line[1..];
                } else if line.starts_with('s') {
                    if line.starts_with("sw") {
                        res.push(Direction::SouthWest)
                    } else if line.starts_with("se") {
                        res.push(Direction::SouthEast)
                    } else {
                        panic!("lol")
                    }
                    line = &line[2..];
                } else if line.starts_with('n') {
                    if line.starts_with("nw") {
                        res.push(Direction::NorthWest)
                    } else if line.starts_with("ne") {
                        res.push(Direction::NorthEast)
                    } else {
                        panic!("lol")
                    }
                    line = &line[2..];
                } else {
                    panic!("lol")
                }
            }
            res
        })
        .collect()
}

fn part1(paths: &[Path]) -> (usize, HashMap<(i16, i16), Tile>) {
    let mut seen = HashMap::new();

    for path in paths {
        let mut i = 0i16;
        let mut j = 0i16;

        for dir in path {
            let (a, b) = dir.offset();
            i += a;
            j += b;
        }

        seen.entry((i, j)).or_insert(Tile::White).swap();
    }

    (seen.values().filter(|t| **t == Tile::Black).count(), seen)
}

fn part2(seen: HashMap<(i16, i16), Tile>) -> usize {
    let mut black_coords = seen
        .iter()
        .filter(|(_, colour)| **colour == Tile::Black)
        .map(|kv| *kv.0)
        .collect::<HashSet<_>>();

    let mut neighbours = HashMap::new();
    for &(i, j) in &black_coords {
        HEX_OFFSETS
            .iter()
            .map(|&(a, b)| (a + i, b + j))
            .for_each(|neighbour_coord| {
                *neighbours.entry(neighbour_coord).or_insert(0i8) += 1;
            });
    }

    for _ in 0..100 {
        let mut new_black = HashSet::with_capacity(black_coords.len());
        let mut new_neighbours = HashMap::with_capacity(neighbours.len());
        for (coord, black_neighbour_count) in neighbours {
            if black_neighbour_count == 2
                || (black_neighbour_count == 1 && black_coords.contains(&coord))
            {
                new_black.insert(coord);
                HEX_OFFSETS
                    .iter()
                    .map(|&(a, b)| (a + coord.0, b + coord.1))
                    .for_each(|neighbour_coord| {
                        *new_neighbours.entry(neighbour_coord).or_insert(0i8) += 1;
                    });
            }
        }

        black_coords = new_black;
        neighbours = new_neighbours;
    }

    black_coords.len()
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let paths = load_paths(INPUT);
    let (p1, generated) = part1(&paths);
    let p2 = part2(generated);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day24::{load_paths, part1, part2};

    #[test]
    fn test_example() {
        let s = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let paths = load_paths(s);
        let (p1, gen) = part1(&paths);
        assert_eq!(p1, 10);
        assert_eq!(part2(gen), 2208);
    }
}
