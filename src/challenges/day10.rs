use advent24::chargrid::{ByteGrid, Point, ORIGIN};
use atoi::atoi;
use clap;
use itertools::{Itertools, PeekingNext};
use pathfinding::prelude::{astar, astar_bag_collect};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    fs, str, usize, vec,
};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day10/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut grid = ByteGrid::new(input);
    let bytes_to_points = grid.bytes_to_points();

    let successors = |p: &Point, offset: i8| -> Vec<(Point, u32)> {
        let mut res: Vec<Point> = vec![];
        let val: i8 = atoi(&[grid[*p]]).unwrap();
        for adj in p.orthogonals() {
            if grid.is_valid_point(adj) {
                let next_val: i8 = atoi(&[grid[adj]]).unwrap();
                if next_val == val + offset {
                    res.push(adj);
                }
            }
        }
        res.into_iter().map(|p| (p, 1)).collect()
    };

    fn heuristic(a: &Point, b: &Point) -> u32 {
        (a.col.abs_diff(b.col) + a.row.abs_diff(b.row)) as u32
    }

    let mut p1_score = 0;
    let mut p2_score = 0;

    for root in bytes_to_points[&b'0'].iter() {
        for dest in bytes_to_points[&b'9'].iter() {
            if let Some(result) = astar_bag_collect(
                root,
                |p| successors(p, 1),
                |p| heuristic(p, dest),
                |p| p == dest,
            ) {
                p1_score += 1;
                for path in result.0 {
                    // let mut new_grid = ByteGrid::new(input);
                    // for v in new_grid.data.iter_mut() {
                    //     *v = b'.';
                    // }
                    // for point in result.0.iter() {
                    //     new_grid[*point] = grid[*point];
                    // }
                    // println!("{}\n", new_grid);
                    p2_score += 1;
                }
            }
        }
    }
    (p1_score, p2_score)
}

pub fn part2(input: &str) -> usize {
    0
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let (part1, part2) = solve(&input);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRID: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_day9_part1() {
        assert_eq!(solve(&TEST_GRID), (36, 81));
    }

    #[test]
    fn test_day9_part2() {

        // assert_eq!(part2(&"171010402"), 88);
    }
}
