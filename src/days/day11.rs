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

#[derive(Clone)]
struct World {
    a: Vec<Vec<Tile>>,
    b: Vec<Vec<Tile>>,
    first: bool,
}

impl World {
    fn iterate(&mut self, trigger: usize, sight_mode: SightMode) -> bool {
        let mut change = false;

        let mut changes = Vec::new();

        if self.first {
            for (i, row) in self.a.iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    let occupied = match sight_mode {
                        SightMode::Surrounding => self.surrounding(i, j),
                        SightMode::LineOfSight => self.line_of_sight(i, j),
                    };
                    if let Some(next) = tile.next(occupied, trigger) {
                        self.b[i][j] = next;
                        changes.push((i, j));
                        change = true;
                    }
                }
            }

            for (i, j) in changes.into_iter() {
                self.a[i][j] = self.b[i][j];
            }
        } else {
            for (i, row) in self.b.iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    let occupied = match sight_mode {
                        SightMode::Surrounding => self.surrounding(i, j),
                        SightMode::LineOfSight => self.line_of_sight(i, j),
                    };
                    if let Some(next) = tile.next(occupied, trigger) {
                        self.a[i][j] = next;
                        changes.push((i, j));
                        change = true;
                    }
                }
            }

            for (i, j) in changes.into_iter() {
                self.b[i][j] = self.a[i][j];
            }
        }

        if change {
            self.first = !self.first;
        }

        change
    }

    fn curr(&self) -> &Vec<Vec<Tile>> {
        if self.first {
            &self.a
        } else {
            &self.b
        }
    }

    fn occupied(&self) -> usize {
        self.curr()
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&t| *t == Tile::Occupied)
            .count()
    }

    fn surrounding(&self, i: usize, j: usize) -> usize {
        let mut occupied = 0;
        let curr = self.curr();

        for &(dx, dy) in ORDINALS.iter() {
            let nx = i as i64 + dx;
            let ny = j as i64 + dy;
            if nx < 0 || ny < 0 || nx >= curr.len() as i64 || ny >= curr[0].len() as i64 {
                continue;
            }
            if curr[nx as usize][ny as usize] == Tile::Occupied {
                occupied += 1;
            }
        }

        occupied
    }

    fn line_of_sight(&self, i: usize, j: usize) -> usize {
        let mut occupied = 0;
        let curr = self.curr();

        for &(dx, dy) in ORDINALS.iter() {
            let mut nx = i as i64 + dx;
            let mut ny = j as i64 + dy;
            while nx >= 0 && ny >= 0 && nx < curr.len() as i64 && ny < curr[0].len() as i64 {
                match curr[nx as usize][ny as usize] {
                    Tile::Occupied => {
                        occupied += 1;
                        break;
                    }
                    Tile::Empty => break,
                    _ => (),
                }
                nx += dx;
                ny += dy;
            }
        }

        occupied
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
                    _ => panic!("bad input: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let second = contents.clone();
    World {
        a: contents,
        b: second,
        first: true,
    }
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
    fn test_p1() {
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
        assert_eq!(part1(world), 37);
    }
}
