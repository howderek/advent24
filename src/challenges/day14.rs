use advent24::{bytegrid::ByteGrid, extract_numbers, parse_number_list};
use clap;
use itertools::Itertools;
use std::{
    fs,
    fs::File,
    i128,
    io::Write,
    str::{self, FromStr},
};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day14/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

#[derive(Debug, Default)]
pub struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    pub fn pos_after(&self, t: i32, w: i32, h: i32) -> (i32, i32) {
        let mut x = (self.x + (self.dx * t)) % w;
        if x < 0 {
            x = w + x;
        }
        let mut y = (self.y + (self.dy * t)) % h;
        if y < 0 {
            y = h + y
        }
        (x, y)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRobotError;

impl FromStr for Robot {
    type Err = ParseRobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<i32> = extract_numbers(s);
        if nums.len() != 4 {
            return Err(ParseRobotError);
        }
        Ok(Robot {
            x: nums[0],
            y: nums[1],
            dx: nums[2],
            dy: nums[3],
        })
    }
}

pub fn part1(input: &str, w: i32, h: i32) -> i32 {
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        robots.push(line.parse().unwrap());
    }
    let final_positions: Vec<(i32, i32)> = robots.iter().map(|r| r.pos_after(100, w, h)).collect();
    let mut q = [0; 4];
    for pos in final_positions.iter() {
        let mut quad_idx = 0;
        if pos.0 == w / 2 || pos.1 == h / 2 {
            continue;
        }
        if pos.0 > w / 2 {
            quad_idx += 1;
        }
        if pos.1 > h / 2 {
            quad_idx += 2;
        }
        q[quad_idx] += 1;
    }
    dbg!(&robots);
    dbg!(&final_positions);
    dbg!(&q);
    q[0] * q[1] * q[2] * q[3]
}

pub fn part2(input: &str, w: i32, h: i32) -> i32 {
    let mut file = fs::File::create("output.txt").unwrap();
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        robots.push(line.parse().unwrap());
    }
    let mut board = ByteGrid::new_empty(b' ', w, h);
    for t in 0..10000 {
        let positions: Vec<(i32, i32)> = robots.iter().map(|r| r.pos_after(t, w, h)).collect();
        for pos in positions {
            board[(pos.1, pos.0)] = b'#';
        }
        writeln!(file, "{}:\n{}\n\n", t, &board).unwrap();
        board.data.fill(b' ');
    }
    0
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    if args.part2 {
        let res = part2(&input, 101, 103);
        println!("{}", res);
    } else {
        let res = part1(&input, 101, 103);
        println!("{}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRID: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_day9() {
        assert_eq!(part1(&TEST_GRID, 11, 7), 12);
    }
}
