use advent24::{
    bytegrid::{ByteGrid, Point, ORIGIN},
    parse_number_list, parse_number_list_delimited_by,
};
use clap;
use pathfinding::prelude::*;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day18/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

pub fn part1(input: &str, width: i32, height: i32, sim_before: i32) -> u32 {
    let mut world = ByteGrid::new_empty(b'.', width, height);
    for (i, line) in input.lines().enumerate() {
        let coords: Vec<i32> = parse_number_list_delimited_by(line, ",");
        if coords.len() != 2 {
            continue;
        }
        world[(coords[1], coords[0])] = b'#';
        if i == (sim_before - 1) as usize {
            break;
        }
    }
    let start = ORIGIN;
    let goal = Point {
        row: height - 1,
        col: width - 1,
    };
    let successors = |p: &Point| -> Vec<(Point, u32)> {
        p.orthogonals()
            .into_iter()
            .filter_map(|p| {
                if world.is_valid_point(p) && world[p] != b'#' {
                    Some((p, 1))
                } else {
                    None
                }
            })
            .collect()
    };
    let heuristic = |p: &Point| -> u32 { p.distance(&goal) };
    let success = |p: &Point| *p == goal;

    if let Some((_, cost)) = astar(&start, successors, heuristic, success) {
        cost
    } else {
        u32::MAX
    }
}

pub fn part2(input: &str) -> String {
    for (i, line) in input.lines().enumerate() {
        if part1(input, 71, 71, i as i32 + 1) == u32::MAX {
            return line.to_owned();
        }
    }
    "not found".to_string()
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    if args.part2 {
        let res = part2(&input);
        println!("{}", res);
    } else {
        let res = part1(&input, 71, 71, 1024);
        println!("{}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRID: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test() {
        assert_eq!(part1(TEST_GRID, 7, 7, 12), 22);
    }
}
