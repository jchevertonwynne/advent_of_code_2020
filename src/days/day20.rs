use fnv::FnvBuildHasher;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};
use std::time::{Duration, Instant};

const INPUT: &str = include_str!("../../files/20.txt");

#[derive(Default, Clone)]
struct Tile([[char; 10]; 10]);

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, line) in self.0.iter().enumerate() {
            for char in line {
                write!(f, "{}", char).expect("be ok");
            }
            if i < self.0.len() - 1 {
                writeln!(f).expect("be ok");
            }
        }

        std::fmt::Result::Ok(())
    }
}

impl Tile {
    fn left(&self) -> [char; 10] {
        self.0
            .iter()
            .map(|r| r[0])
            .collect::<Vec<_>>()
            .try_into()
            .expect("should be correct length")
    }

    fn right(&self) -> [char; 10] {
        self.0
            .iter()
            .map(|r| r[9])
            .collect::<Vec<_>>()
            .try_into()
            .expect("should be correct length")
    }

    fn top(&self) -> [char; 10] {
        self.0[0]
    }

    fn bottom(&self) -> [char; 10] {
        self.0[9]
    }

    fn flip_hori(&self) -> Tile {
        let mut res = self.clone();
        for row in res.0.iter_mut() {
            row.reverse();
        }
        res
    }

    fn flip_vert(&self) -> Tile {
        let mut res = self.clone();
        res.0.reverse();
        res
    }

    fn rotate(&self) -> Tile {
        let mut res = Tile::default();

        for (i, row) in self.0.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                res.0[j][9 - i] = *tile;
            }
        }

        res
    }

    fn rotations(&self) -> Vec<Tile> {
        let mut res = Vec::new();

        res.push(self.clone());
        for _ in 0..3 {
            res.push(res.last().expect("should have element").rotate())
        }

        let tile_h = self.flip_hori();
        res.push(tile_h);
        for _ in 0..3 {
            res.push(res.last().expect("should have element").rotate())
        }

        let tile_v = self.flip_vert();
        res.push(tile_v);
        for _ in 0..3 {
            res.push(res.last().expect("should have element").rotate())
        }

        let tile_v = self.flip_vert().flip_hori();
        res.push(tile_v);
        for _ in 0..3 {
            res.push(res.last().expect("should have element").rotate())
        }

        res
    }
}

fn load_tiles(input: &str) -> HashMap<usize, Tile, FnvBuildHasher> {
    let mut res = HashMap::with_hasher(FnvBuildHasher::default());

    for tileset in input.split("\n\n") {
        let mut lines = tileset.lines();
        let title = lines.next().expect("should have title row");

        let id = title[5..9]
            .parse::<usize>()
            .expect("please be a valid 4 digit int");

        let mut parts = Tile::default();
        for (i, line) in lines.enumerate() {
            for (j, tile) in line.chars().enumerate() {
                parts.0[i][j] = tile;
            }
        }

        res.insert(id, parts);
    }

    res
}

fn part1(tiles: &HashMap<usize, Tile, FnvBuildHasher>) -> usize {
    let mut seen_counts = HashMap::with_hasher(FnvBuildHasher::default());

    for (id, tile) in tiles {
        for rot in tile.rotations() {
            seen_counts
                .entry(rot.top())
                .or_insert_with(|| HashSet::with_hasher(FnvBuildHasher::default()))
                .insert(*id);
        }
    }

    let res = seen_counts
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .flat_map(|kv| kv.1.iter().copied())
        .collect::<Vec<_>>();

    let mut options = HashMap::with_capacity_and_hasher(res.len(), FnvBuildHasher::default());
    for id in res {
        *options.entry(id).or_insert(0) += 1;
    }

    options
        .iter()
        .filter(|(_, v)| **v == 4)
        .map(|kv| kv.0)
        .product()
}

fn rotate(coords: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let x = coords.iter().fold(0, |acc, &(x, _)| Ord::max(acc, x));
    coords.iter().map(|&(a, b)| (b, x - a)).collect()
}

fn mirror_flip(coords: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let x = coords.iter().fold(0, |acc, &(x, _)| Ord::max(acc, x));
    coords.iter().map(|&(a, b)| (x - a, b)).collect()
}

fn part2(tiles: &HashMap<usize, Tile, FnvBuildHasher>) -> usize {
    let mut tiles_that_have_this_edge: HashMap<
        [char; 10],
        HashSet<usize, FnvBuildHasher>,
        FnvBuildHasher,
    > = HashMap::with_capacity_and_hasher(tiles.len(), FnvBuildHasher::default());

    for (id, tile) in tiles {
        for tile in tile.rotations() {
            tiles_that_have_this_edge
                .entry(tile.top())
                .or_insert_with(|| HashSet::with_hasher(FnvBuildHasher::default()))
                .insert(*id);
        }
    }

    let singular_solutions = tiles_that_have_this_edge
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .flat_map(|kv| kv.1.iter().copied())
        .collect::<Vec<_>>();

    let mut options =
        HashMap::with_capacity_and_hasher(singular_solutions.len(), FnvBuildHasher::default());
    for id in singular_solutions {
        *options.entry(id).or_insert(0) += 1;
    }

    let corner = options
        .iter()
        .filter(|(_, v)| **v == 4)
        .map(|kv| *kv.0)
        .map(|id| (id, tiles.get(&id).expect("exist")))
        .next()
        .expect("should be a corner unit");

    let mut left_corner = corner.1.clone();
    while !(tiles_that_have_this_edge
        .get(&left_corner.left())
        .expect("pls")
        .len()
        == 1
        && tiles_that_have_this_edge
            .get(&left_corner.top())
            .expect("pls")
            .len()
            == 1)
    {
        left_corner = left_corner.rotate();
    }

    let mut grid: [[Tile; 12]; 12] = Default::default();
    grid[0][0] = left_corner;

    let mut placed = HashSet::with_hasher(FnvBuildHasher::default());
    placed.insert(corner.0);

    for i in 1..12 {
        let above = grid[i - 1][0].bottom();

        let connecting_id = tiles_that_have_this_edge
            .get(&above)
            .expect("find this above")
            .iter()
            .find(|i| !placed.contains(*i))
            .expect("should have next option");

        placed.insert(*connecting_id);

        for rotation in tiles.get(connecting_id).expect("should exist").rotations() {
            if rotation.top() == above
                && tiles_that_have_this_edge
                    .get(&rotation.left())
                    .expect("exists")
                    .len()
                    == 1
            {
                grid[i][0] = rotation;
                break;
            }
        }
    }

    for j in 1..12 {
        let to_left = grid[0][j - 1].right();

        let connecting_id = tiles_that_have_this_edge
            .get(&to_left)
            .expect("find this above")
            .iter()
            .find(|i| !placed.contains(*i))
            .expect("should have next option");

        placed.insert(*connecting_id);

        for rotation in tiles.get(connecting_id).expect("should exist").rotations() {
            if rotation.left() == to_left
                && tiles_that_have_this_edge
                    .get(&rotation.top())
                    .expect("exists")
                    .len()
                    == 1
            {
                grid[0][j] = rotation;
                break;
            }
        }
    }

    for i in 1..12 {
        for j in 1..12 {
            let top_edge = grid[i - 1][j].bottom();
            let left_edge = grid[i][j - 1].right();

            let match_top = tiles_that_have_this_edge
                .get(&top_edge)
                .expect(&*format!("{:?}", top_edge));
            let match_left = tiles_that_have_this_edge
                .get(&left_edge)
                .expect(&*format!("{:?}", left_edge));
            let connecting_id = HashSet::intersection(&match_top, &match_left)
                .next()
                .expect("is one option");

            for rotation in tiles.get(connecting_id).expect("should exist").rotations() {
                if rotation.top() == top_edge && rotation.left() == left_edge {
                    grid[i][j] = rotation;
                    break;
                }
            }
        }
    }

    let mut fixed_grid = [[0 as char; 96]; 96];
    for (x, row) in grid.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            for (w, tr) in tile.0[1..9].iter().enumerate() {
                for (q, tt) in tr[1..9].iter().enumerate() {
                    fixed_grid[x * 8 + w][y * 8 + q] = *tt;
                }
            }
        }
    }

    let monster = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";
    let mut monster_coords = monster
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .filter(|(_, _, c)| *c == '#')
        .map(|(i, j, _)| (i, j))
        .collect::<Vec<_>>();

    let mut monster_count = 0;

    for _ in 0..4 {
        monster_coords = rotate(&monster_coords);

        let max_coords = monster_coords
            .iter()
            .fold((0usize, 0usize), |(ax, ay), &(x, y)| {
                (Ord::max(ax, x), Ord::max(ay, y))
            });

        for i in 0..96 - max_coords.0 {
            for j in 0..96 - max_coords.1 {
                if monster_coords
                    .iter()
                    .map(|(x, y)| (x + i, y + j))
                    .all(|(i, j)| fixed_grid[i][j] == '#')
                {
                    monster_count += 1;
                }
            }
        }

        let monster_coords = mirror_flip(&monster_coords);

        let max_coords = monster_coords
            .iter()
            .fold((0usize, 0usize), |(ax, ay), &(x, y)| {
                (Ord::max(ax, x), Ord::max(ay, y))
            });

        for i in 0..96 - max_coords.0 {
            for j in 0..96 - max_coords.1 {
                if monster_coords
                    .iter()
                    .map(|(x, y)| (x + i, y + j))
                    .all(|(i, j)| fixed_grid[i][j] == '#')
                {
                    monster_count += 1;
                }
            }
        }
    }

    fixed_grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|c| **c == '#')
        .count()
        - (monster_coords.len() * monster_count)
}

pub fn run() -> (String, String, Duration) {
    let start = Instant::now();
    let tiles = load_tiles(INPUT);
    let p1 = part1(&tiles);
    let p2 = part2(&tiles);

    (p1.to_string(), p2.to_string(), start.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::days::day20::{load_tiles, mirror_flip, rotate, Tile};
    use std::convert::TryInto;

    #[test]
    fn can_rotate() {
        let s = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";
        let tiles = load_tiles(s);
        let tile = tiles.get(&2311).expect("is a thing");
        println!("{:?}", tile);
        // println!("{:?}", tile.get_edges());
    }

    #[test]
    fn find_monsters() {
        let world = ".####...#####..#...###..
#####..#..#.#.####..#.#.
.#.#...#.###...#.##.##..
#.#.##.###.#.##.##.#####
..##.###.####..#.####.##
...#.#..##.##...#..#..##
#.##.#..#.#..#..##.#.#..
.###.##.....#...###.#...
#.####.#.#....##.#..#.#.
##...#..#....#..#...####
..#.##...###..#.#####..#
....#.##.#.#####....#...
..##.##.###.....#.##..#.
#...#...###..####....##.
.#.##...#.##.#.#.###...#
#.###.#..####...##..#...
#.###...#.##...#.######.
.###.###.#######..#####.
..##.#..#..#.#######.###
#.#..##.########..#..##.
#.#####..#.#...##..#....
#....##..#.#########..##
#...#.....#..##...###.##
#..###....##.#...##.##.#"
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let monster = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";
        let monster_coords = monster
            .lines()
            .enumerate()
            .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| (i, j, c)))
            .filter(|(_, _, c)| *c == '#')
            .map(|(i, j, _)| (i, j))
            .collect::<Vec<_>>();

        let max_coords = monster_coords
            .iter()
            .fold((0usize, 0usize), |(ax, ay), &(x, y)| {
                (Ord::max(ax, x), Ord::max(ay, y))
            });

        let mut monster_count = 0;
        for i in 0..=24 - max_coords.0 {
            for j in 0..=24 - max_coords.1 {
                if monster_coords
                    .iter()
                    .map(|(x, y)| (x + i, y + j))
                    .all(|(i, j)| world[i][j] == '#')
                {
                    monster_count += 1;
                }
            }
        }
        assert_eq!(monster_count, 2);

        assert_eq!(
            world
                .iter()
                .flat_map(|line| line.iter())
                .filter(|c| **c == '#')
                .count()
                - (monster_count * monster_coords.len()),
            273
        );
    }

    #[test]
    fn test_rotations() {
        let s: Vec<Vec<char>> = "abcdefghij
abcdefghij
abcdefghij
abcdefghij
abcdefghij
abcdefghij
abcdefghij
abcdefghij
abcdefghij
abcdefghij"
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut b: [[char; 10]; 10] = Default::default();
        b.iter_mut()
            .zip(s)
            .for_each(|(q, w)| *q = w.try_into().expect("yes"));

        let t = Tile(b);
        println!("{:?}", t);
        println!("{:?}", t.rotate());
        println!("{:?}", t.rotate().rotate());
        println!("{:?}", t.rotate().rotate().rotate());
        println!("{:?}", t.flip_hori());
        println!("{:?}", t.flip_vert());
        println!("{:?}", t.flip_vert().flip_hori());
        println!("{:?}", t.left());
        println!("{:?}", t.right());
        println!("{:?}", t.top());
        println!("{:?}", t.bottom());
    }

    #[test]
    fn test_rotate() {
        let coords = vec![(0, 1), (1, 0), (2, 2)];
        println!("{:?}", rotate(&coords));
        println!("{:?}", mirror_flip(&coords));
    }

    #[test]
    fn test_monster_rotations() {
        let world = ".#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###"
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let monster = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";
        let mut monster_coords = monster
            .lines()
            .enumerate()
            .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| (i, j, c)))
            .filter(|(_, _, c)| *c == '#')
            .map(|(i, j, _)| (i, j))
            .collect::<Vec<_>>();

        let mut monster_count = 0;

        for _ in 0..4 {
            monster_coords = rotate(&monster_coords);

            let max_coords = monster_coords
                .iter()
                .fold((0usize, 0usize), |(ax, ay), &(x, y)| {
                    (Ord::max(ax, x), Ord::max(ay, y))
                });

            for i in 0..24 - max_coords.0 {
                for j in 0..24 - max_coords.1 {
                    if monster_coords
                        .iter()
                        .map(|(x, y)| (x + i, y + j))
                        .all(|(i, j)| world[i][j] == '#')
                    {
                        monster_count += 1;
                    }
                }
            }

            let monster_coords = mirror_flip(&monster_coords);

            let max_coords = monster_coords
                .iter()
                .fold((0usize, 0usize), |(ax, ay), &(x, y)| {
                    (Ord::max(ax, x), Ord::max(ay, y))
                });

            for i in 0..24 - max_coords.0 {
                for j in 0..24 - max_coords.1 {
                    if monster_coords
                        .iter()
                        .map(|(x, y)| (x + i, y + j))
                        .all(|(i, j)| world[i][j] == '#')
                    {
                        monster_count += 1;
                    }
                }
            }
        }
        println!("{}", monster_count);
    }
}
