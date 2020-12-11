use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/11.txt");

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Floor,
    Occupied,
    Empty,
}

enum SightMode {
    Immediate,
    LineOfSight,
}

#[derive(Clone)]
struct World {
    a: Vec<Vec<Tile>>,
    b: Vec<Vec<Tile>>,
    first: bool
}

impl World {
    fn iterate(&mut self, trigger: usize, sight_mode: SightMode) -> bool {
        let mut change = false;

        match self.first {
            true => {
                for (i, row) in self.a.iter().enumerate() {
                    for (j, tile) in row.iter().enumerate() {
                        let occ = match sight_mode {
                            SightMode::Immediate => self.surrounding(i, j),
                            SightMode::LineOfSight => self.line_of_sight(i, j),
                        };
                        let next = match tile {
                            Tile::Occupied if occ >= trigger => {
                                change = true;
                                Tile::Empty
                            }
                            Tile::Empty if occ == 0 => {
                                change = true;
                                Tile::Occupied
                            }
                            _ => *tile,
                        };
                        self.b[i][j] = next;
                    }
                }
            }
            false => {
                for (i, row) in self.b.iter().enumerate() {
                    for (j, tile) in row.iter().enumerate() {
                        let occ = match sight_mode {
                            SightMode::Immediate => self.surrounding(i, j),
                            SightMode::LineOfSight => self.line_of_sight(i, j),
                        };
                        let next = match tile {
                            Tile::Occupied if occ >= trigger => {
                                change = true;
                                Tile::Empty
                            }
                            Tile::Empty if occ == 0 => {
                                change = true;
                                Tile::Occupied
                            }
                            _ => *tile,
                        };
                        self.a[i][j] = next;
                    }
                }
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

    fn surrounding(&self, i: usize, j: usize) -> usize {
        let mut occupied = 0;

        let directions = vec![
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        for (dx, dy) in directions {
            let nx = i as i64 + dx;
            let ny = j as i64 + dy;
            if nx < 0 || ny < 0 || nx >= self.curr().len() as i64 || ny >= self.curr()[0].len() as i64 {
                continue;
            }
            if self.curr()[nx as usize][ny as usize] == Tile::Occupied {
                occupied += 1;
            }
        }

        occupied
    }

    fn line_of_sight(&self, i: usize, j: usize) -> usize {
        let mut occupied = 0;
        let directions = vec![
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        for (dx, dy) in directions {
            let mut nx = i as i64 + dx;
            let mut ny = j as i64 + dy;
            while nx >= 0 && ny >= 0 && nx < self.curr().len() as i64 && ny < self.curr()[0].len() as i64 {
                match self.curr()[nx as usize][ny as usize] {
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
    let second = vec![vec![Tile::Empty; contents[0].len()]; contents.len()];
    World{
        a: contents,
        b: second,
        first: true
    }
}

fn part1(mut world: World) -> usize {
    while world.iterate(4, SightMode::Immediate) {}

    world
        .curr()
        .iter()
        .map(|row| row.iter().filter(|&t| *t == Tile::Occupied).count())
        .sum()
}

fn part2(mut world: World) -> usize {
    while world.iterate(5, SightMode::LineOfSight) {}

    world
        .curr()
        .iter()
        .map(|row| row.iter().filter(|&t| *t == Tile::Occupied).count())
        .sum()
}

pub fn run() -> (usize, usize, Duration) {
    let start = Instant::now();
    let world = load_world(INPUT);
    let dl = Instant::now();
    let p1 = part1(world.clone());
    let dp1 = Instant::now();
    let p2 = part2(world);
    let done = Instant::now();

    println!("{:?} {:?}", dl - start, dp1 - dl);

    (p1, p2, done - start)
}

#[cfg(test)]
mod test {
    use crate::days::day11::{load_world, part1};

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
