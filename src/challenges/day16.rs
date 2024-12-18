use advent24::bytegrid::{ByteGrid, Point, EAST, NORTH, SOUTH, WEST};
use clap;
use pathfinding::prelude::*;
use std::{collections::HashSet, fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day16/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Reindeer {
    pub pos: Point,
    pub heading: Point,
}

impl Reindeer {
    pub fn new(pos: Point) -> Self {
        Reindeer { pos, heading: EAST }
    }

    pub fn successors(&self) -> Vec<(Reindeer, u32)> {
        match self.heading {
            NORTH => vec![
                (
                    Reindeer {
                        pos: self.pos + NORTH,
                        heading: NORTH,
                    },
                    1,
                ),
                (
                    Reindeer {
                        pos: self.pos,
                        heading: WEST,
                    },
                    1000,
                ),
                (
                    Reindeer {
                        pos: self.pos,
                        heading: EAST,
                    },
                    1000,
                ),
            ],
            SOUTH => vec![
                (
                    Reindeer {
                        pos: self.pos + SOUTH,
                        heading: SOUTH,
                    },
                    1,
                ),
                (
                    Reindeer {
                        pos: self.pos,
                        heading: WEST,
                    },
                    1000,
                ),
                (
                    Reindeer {
                        pos: self.pos,
                        heading: EAST,
                    },
                    1000,
                ),
            ],
            EAST => vec![
                (
                    Reindeer {
                        pos: self.pos + EAST,
                        heading: EAST,
                    },
                    1,
                ),
                (
                    Reindeer {
                        pos: self.pos,
                        heading: NORTH,
                    },
                    1000,
                ),
                (
                    Reindeer {
                        pos: self.pos,
                        heading: SOUTH,
                    },
                    1000,
                ),
            ],
            WEST => vec![
                (
                    Reindeer {
                        pos: self.pos + WEST,
                        heading: WEST,
                    },
                    1,
                ),
                (
                    Reindeer {
                        pos: self.pos,
                        heading: NORTH,
                    },
                    1000,
                ),
                (
                    Reindeer {
                        pos: self.pos,
                        heading: SOUTH,
                    },
                    1000,
                ),
            ],
            _ => unreachable!(),
        }
    }

    pub fn dist(&self, other: &Reindeer) -> u32 {
        self.pos.row.abs_diff(other.pos.row) + self.pos.col.abs_diff(other.pos.col)
    }
}

pub fn part1(input: &str) -> u32 {
    let mut world = ByteGrid::new(input);
    let bytes_to_points = world.bytes_to_points();

    let start = Reindeer::new(bytes_to_points[&b'S'][0]);
    let goal = Reindeer::new(bytes_to_points[&b'E'][0]);
    let successors = |p: &Reindeer| -> Vec<(Reindeer, u32)> {
        p.successors()
            .into_iter()
            .filter(|r| world.is_valid_point(r.0.pos) && world[r.0.pos] != b'#')
            .collect()
    };
    let heuristic = |p: &Reindeer| -> u32 { p.dist(&goal) };
    let success = |p: &Reindeer| p.pos == goal.pos;

    let (path, total_cost) = astar(&start, successors, heuristic, success).expect("no path found");
    for p in path {
        world[p.pos] = match p.heading {
            NORTH => b'^',
            EAST => b'>',
            SOUTH => b'V',
            WEST => b'<',
            _ => b'?',
        };
    }
    world[start.pos] = b'S';
    world[goal.pos] = b'E';
    println!("{}", &world);
    total_cost
}

pub fn part2(input: &str) -> u32 {
    let mut world = ByteGrid::new(input);
    let bytes_to_points = world.bytes_to_points();

    let start = Reindeer::new(bytes_to_points[&b'S'][0]);
    let goal = Reindeer::new(bytes_to_points[&b'E'][0]);
    let successors = |p: &Reindeer| -> Vec<(Reindeer, u32)> {
        p.successors()
            .into_iter()
            .filter(|r| world.is_valid_point(r.0.pos) && world[r.0.pos] != b'#')
            .collect()
    };
    let heuristic = |p: &Reindeer| -> u32 { p.dist(&goal) };
    let success = |p: &Reindeer| p.pos == goal.pos;

    let (paths, _) =
        astar_bag_collect(&start, successors, heuristic, success).expect("no path found");

    let mut tiles: HashSet<Point> = HashSet::new();
    for path in paths {
        for reindeer in path {
            tiles.insert(reindeer.pos);
        }
    }

    for tile in tiles.iter() {
        world[*tile] = b'*';
    }

    let b_to_p = world.bytes_to_points();
    println!("{}", &world);

    b_to_p[&b'*'].len() as u32
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    if args.part2 {
        let res = part2(&input);
        println!("{}", res);
    } else {
        let res = part1(&input);
        println!("{}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRID: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST_GRID_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_day9() {
        assert_eq!(part1(TEST_GRID), 7036);
        assert_eq!(part1(TEST_GRID_2), 11048);
        assert_eq!(part2(TEST_GRID), 45);
        assert_eq!(part2(TEST_GRID_2), 64);
    }
}
