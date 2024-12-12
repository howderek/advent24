use advent24::bytegrid::{ByteGrid, Point};
use clap;
use std::{collections::HashSet, fs, i32, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day12/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

#[derive(Debug)]
pub struct Region {
    byte: u8,
    points: Vec<Point>,
    min_row: i32,
    max_row: i32,
    min_col: i32,
    max_col: i32,
    area: i32,
    sides: i32,
    perimeter: i32,
}

impl Region {
    pub fn new(byte: u8) -> Self {
        Region {
            byte: byte,
            points: vec![],
            min_row: i32::MAX,
            max_row: 0,
            min_col: i32::MAX,
            max_col: 0,
            area: 0,
            sides: 0,
            perimeter: 0,
        }
    }

    pub fn add_point(&mut self, grid: &ByteGrid, point: Point) {
        if point.row < self.min_row {
            self.min_row = point.row;
        }
        if point.row > self.max_row {
            self.max_row = point.row;
        }
        if point.col < self.min_col {
            self.min_col = point.col;
        }
        if point.col > self.max_col {
            self.max_col = point.col;
        }
        for adj in point.orthogonals() {
            if let Some(p) = grid.get_point(adj) {
                if *p != self.byte {
                    self.perimeter += 1;
                }
            } else {
                self.perimeter += 1;
            }
        }
        for corner in point.corners() {
            if grid.get_point(corner[0]) != Some(&self.byte)
                && grid.get_point(corner[2]) != Some(&self.byte)
            {
                self.sides += 1;
            } else if grid.get_point(corner[0]) == Some(&self.byte)
                && grid.get_point(corner[2]) == Some(&self.byte)
                && grid.get_point(corner[1]) != Some(&self.byte)
            {
                self.sides += 1;
            }
        }

        self.area += 1;
        self.points.push(point);
    }

    pub fn area(&self) -> i32 {
        self.area
    }

    pub fn sides(&self) -> i32 {
        self.sides
    }

    pub fn p1_score(&self) -> i32 {
        self.area() * self.perimeter
    }

    pub fn p2_score(&self) -> i32 {
        self.area() * self.sides()
    }
}

pub fn solve(input: &str) -> (i32, i32) {
    let grid = ByteGrid::new(input);
    let mut regions: Vec<Region> = vec![];
    let mut seen: HashSet<Point> = HashSet::new();
    for row in 0..grid.height {
        for col in 0..grid.width {
            let point = Point::new(row, col);
            if !seen.contains(&point) {
                let byte = grid[point];
                let mut region = Region::new(byte);
                grid.flood_orthogonals(point, |_, p| {
                    seen.insert(p);
                    region.add_point(&grid, p);
                });
                regions.push(region);
            }
        }
    }
    let p1 = regions.iter().map(|r| r.p1_score()).sum();
    let p2 = regions.iter().map(|r| r.p2_score()).sum();
    (p1, p2)
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let (p1, p2) = solve(&input);
    println!("part1: {}", p1);
    println!("part2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMOL_EXAMPLE: &str = "\
AAAA
BBCD
BBCC
EEEC";

    const LARGER_EXAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const ESHAPED_EXAMPLE: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const THIRD_EXAMPLE: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    const CROSS: &str = "\
.A.
AAA
.A.";

    const DIAG: &str = "\
.BB
A.B
AA.";

    #[test]
    fn test_day12() {
        assert_eq!(solve(&SMOL_EXAMPLE), (140, 80));
        assert_eq!(solve(&LARGER_EXAMPLE), (1930, 1206));
        assert_eq!(solve(&ESHAPED_EXAMPLE).1, 236);
        assert_eq!(solve(&THIRD_EXAMPLE).1, 368);
        assert_eq!(solve(&"AA\nAB").1, 22);
        assert_eq!(solve(&CROSS).1, 76);
        assert_eq!(solve(&DIAG).1, 48);
    }
}
