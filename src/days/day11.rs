use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/11.txt");

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Floor,
    Occupied,
    Empty,
}

impl Tile {
    fn next(&self, occupied: usize, trigger: usize) -> Option<Tile> {
        match self {
            Tile::Occupied if occupied >= trigger => Some(Tile::Empty),
            Tile::Empty if occupied == 0 => Some(Tile::Occupied),
            _ => None,
        }
    }
}

enum SightMode {
    Surrounding,
    LineOfSight,
}

#[derive(Clone, Debug)]
struct World {
    floor: Vec<Vec<Tile>>,
    first: bool,
    line_of_sight: Vec<Vec<Vec<(usize, usize)>>>,
}

impl World {
    fn iterate(&mut self, trigger: usize, sight_mode: SightMode) -> bool {
        let mut change = false;
        let mut copy_into = self.floor.clone();

        for (i, row) in self.floor.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                let occupied = match sight_mode {
                    SightMode::Surrounding => self.surrounding(i, j),
                    SightMode::LineOfSight => self.line_of_sight[i][j]
                        .iter()
                        .filter(|(x, y)| self.floor[*x][*y] == Tile::Occupied)
                        .count(),
                };
                if let Some(next) = tile.next(occupied, trigger) {
                    copy_into[i][j] = next;
                    change = true;
                }
            }
        }

        if change {
            self.first = !self.first;
            self.floor = copy_into;
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
                let mut ind = Vec::new();

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
    let mut world = World {
        floor: contents,
        first: true,
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

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let world = load_world(INPUT);
    let p1 = part1(world.clone());
    let p2 = part2(world.clone());
    let done = Instant::now();

    (p1, p2, done - start)
}

#[cfg(test)]
mod test {
    use crate::days::day11::{load_world, part1, part2, INPUT};

    #[test]
    fn test_actual() {
        let world = load_world(INPUT);
        assert_eq!(part1(world.clone()), 2204);
        assert_eq!(part2(world.clone()), 1986);
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

    #[test]
    fn test_sight() {
        let s = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";
        let world = load_world(s);
        for c in world.line_of_sight.chunks(7) {
            for v in c {
                print!("{} ", v.len());
            }
            println!();
        }
        assert_eq!(world.line_of_sight[3 * world.w + 2].len(), 0);
    }
}
