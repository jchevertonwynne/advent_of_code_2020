use arrayvec::ArrayVec;
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/11.txt");

const ORDINALS: [(i64, i64); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Floor,
    Occupied,
    Empty,
}

enum SightMode {
    Surrounding,
    LineOfSight,
}

type LineOfSightOptions = ArrayVec<[(usize, usize); 8]>;

#[derive(Clone, Debug)]
struct World {
    floor: Vec<Vec<Tile>>,
    first: bool,
    to_apply: Vec<(usize, usize)>,
    line_of_sight: Vec<Vec<LineOfSightOptions>>,
}

impl World {
    fn iterate(&mut self, trigger: usize, sight_mode: SightMode) -> bool {
        let mut change = false;

        for (i, row) in self.floor.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Floor => (),
                    Tile::Occupied => match sight_mode {
                        SightMode::Surrounding => {
                            if self.surrounding(i, j) >= trigger {
                                self.to_apply.push((i, j));
                                change = true;
                            }
                        }
                        SightMode::LineOfSight => {
                            if self.line_of_sight[i][j].len() >= trigger
                                && self.line_of_sight[i][j]
                                    .iter()
                                    .filter(|(x, y)| self.floor[*x][*y] == Tile::Occupied)
                                    .count()
                                    >= trigger
                            {
                                self.to_apply.push((i, j));
                                change = true;
                            }
                        }
                    },
                    Tile::Empty => match sight_mode {
                        SightMode::Surrounding => {
                            if self.surrounding(i, j) == 0 {
                                self.to_apply.push((i, j));
                                change = true;
                            }
                        }
                        SightMode::LineOfSight => {
                            if self.line_of_sight[i][j]
                                .iter()
                                .all(|(x, y)| self.floor[*x][*y] != Tile::Occupied)
                            {
                                self.to_apply.push((i, j));
                                change = true;
                            }
                        }
                    },
                }
            }
        }

        while let Some((i, j)) = self.to_apply.pop() {
            self.floor[i][j] = match self.floor[i][j] {
                Tile::Floor => panic!("omg"),
                Tile::Occupied => Tile::Empty,
                Tile::Empty => Tile::Occupied,
            }
        }

        change
    }

    fn occupied(&self) -> usize {
        self.floor
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&t| *t == Tile::Occupied)
            .count()
    }

    fn surrounding(&self, i: usize, j: usize) -> usize {
        let mut occupied = 0;

        if i > 0 {
            if j > 0 && self.floor[i - 1][j - 1] == Tile::Occupied {
                occupied += 1;
            }

            if self.floor[i - 1][j] == Tile::Occupied {
                occupied += 1;
            }

            if j < self.floor[0].len() - 1 && self.floor[i - 1][j + 1] == Tile::Occupied {
                occupied += 1;
            }
        }

        if i < self.floor.len() - 1 {
            if j > 0 && self.floor[i + 1][j - 1] == Tile::Occupied {
                occupied += 1;
            }

            if self.floor[i + 1][j] == Tile::Occupied {
                occupied += 1;
            }

            if j < self.floor[0].len() - 1 && self.floor[i + 1][j + 1] == Tile::Occupied {
                occupied += 1;
            }
        }

        if j > 0 && self.floor[i][j - 1] == Tile::Occupied {
            occupied += 1;
        }

        if j < self.floor[0].len() - 1 && self.floor[i][j + 1] == Tile::Occupied {
            occupied += 1;
        }

        occupied
    }

    fn gen_line_of_sight_options(&mut self) {
        for i in 0..self.floor.len() {
            let mut row = Vec::new();
            for j in 0..self.floor[0].len() {
                let mut ind = ArrayVec::new();

                for &(dx, dy) in ORDINALS.iter() {
                    let mut nx = i as i64 + dx;
                    let mut ny = j as i64 + dy;
                    while nx >= 0
                        && ny >= 0
                        && nx < self.floor.len() as i64
                        && ny < self.floor[0].len() as i64
                    {
                        match self.floor[nx as usize][ny as usize] {
                            Tile::Occupied | Tile::Empty => {
                                ind.push((nx as usize, ny as usize));
                                break;
                            }
                            _ => (),
                        }
                        nx += dx;
                        ny += dy;
                    }
                }
                row.push(ind);
            }
            self.line_of_sight.push(row);
        }
    }
}

fn load_world(input: &str) -> World {
    let contents = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Tile::Empty,
                    '.' => Tile::Floor,
                    '#' => Tile::Occupied,
                    _ => panic!("bad input: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let size = contents.len() * contents[0].len();
    let mut world = World {
        floor: contents,
        first: true,
        to_apply: Vec::with_capacity(size),
        line_of_sight: Vec::new(),
    };
    world.gen_line_of_sight_options();
    world
}

fn part1(mut world: World) -> usize {
    while world.iterate(4, SightMode::Surrounding) {}
    world.occupied()
}

fn part2(mut world: World) -> usize {
    while world.iterate(5, SightMode::LineOfSight) {}
    world.occupied()
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let world = load_world(INPUT);
    let p1 = part1(world.clone());
    let p2 = part2(world.clone());

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod test {
    use crate::days::day11::{load_world, part1, part2, INPUT};

    #[test]
    fn test_actual() {
        let world = load_world(INPUT);
        assert_eq!(part1(world.clone()), 2_204);
        assert_eq!(part2(world.clone()), 1_986);
    }

    #[test]
    fn test_example() {
        let s = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let world = load_world(s);
        assert_eq!(part1(world.clone()), 37);
        assert_eq!(part2(world.clone()), 26);
    }
}
