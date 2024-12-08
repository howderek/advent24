use advent24::{
    chargrid::{ByteGrid, Point, ORIGIN},
    parse_number_list, print_2d_array, string_to_2d_array, Tile,
};
use clap;
use itertools::{self, Itertools};
use std::{
    collections::{HashMap, HashSet},
    fs, iter, str,
};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day8/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

pub fn solver(input: &str, skip: usize, take: usize) -> usize {
    let mut world = ByteGrid::new(&input);
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
    return world.count(b'#');
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    println!("part1: {}", solver(&input, 1, 1));
    println!("part2: {}", solver(&input, 0, 1000));
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;

//     #[test]
//     fn test_u8_grid() {
//         let mut grid = ByteGrid::new(
//             "190: 10 19\n\
//              3267: 81 40 27\n\
//              83: 17 5\n\
//              156: 15 6\n\
//              7290: 6 8 6 15\n\
//              161011: 16 10 13\n\
//              192: 17 8 14\n\
//              21037: 9 7 18 13\n\
//              292: 11 6 16 20",
//         );
