use advent24::bytegrid::ByteGrid;
use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day12/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

pub fn solve(input: &str) -> (i32, i32) {
    let grid = ByteGrid::new(input);
    let regions = grid.to_regions();
    let p1 = regions.iter().map(|r| r.area() * r.perimeter()).sum();
    let p2 = regions.iter().map(|r| r.area() * r.sides()).sum();
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
        assert_eq!(solve(SMOL_EXAMPLE), (140, 80));
        assert_eq!(solve(LARGER_EXAMPLE), (1930, 1206));
        assert_eq!(solve(ESHAPED_EXAMPLE).1, 236);
        assert_eq!(solve(THIRD_EXAMPLE).1, 368);
        assert_eq!(solve("AA\nAB").1, 22);
        assert_eq!(solve(CROSS).1, 76);
        assert_eq!(solve(DIAG).1, 48);
    }
}
