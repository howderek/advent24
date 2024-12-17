use advent24::bytegrid::{ByteGrid, ORIGIN};
use clap;
use itertools::{self, Itertools};
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day8/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

pub fn solver(input: &str, skip: usize, take: usize) -> usize {
    let mut world = ByteGrid::new(input);
    let mut bytes_to_points = world.bytes_to_points();
    bytes_to_points.remove(&b'.');
    for point_vec in bytes_to_points.values() {
        for points in point_vec.iter().combinations(2) {
            let (mut smaller, mut larger) = (*points[0], *points[1]);
            if smaller > larger {
                (smaller, larger) = (larger, smaller);
            }
            let distance = larger - smaller;
            for antinode in world
                .iter_mut_towards(smaller, ORIGIN - distance)
                .skip(skip)
                .take(take)
            {
                *antinode = b'#';
            }
            for antinode in world
                .iter_mut_towards(larger, distance)
                .skip(skip)
                .take(take)
            {
                *antinode = b'#';
            }
        }
    }
    world.count(b'#')
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    println!("part1: {}", solver(&input, 1, 1));
    println!("part2: {}", solver(&input, 0, 1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRID: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_day8() {
        assert_eq!(solver(TEST_GRID, 1, 1), 14);
        assert_eq!(solver(TEST_GRID, 0, 100), 34);
    }
}
