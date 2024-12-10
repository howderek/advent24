use advent24::bytegrid::{ByteGrid, Point};
use clap;
use std::{collections::VecDeque, fs};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day10/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

pub fn solve(input: &str) -> (i32, i32) {
    let grid = ByteGrid::new(input);
    let bytes_to_points = grid.bytes_to_points();

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut visited: Vec<i32> = vec![0; grid.data.len()];

    for root in bytes_to_points[&b'0'].iter() {
        queue.push_back(*root);
        visited.fill(0);
        visited[grid.point_to_idx(*root)] = 1;

        grid.bfs_all(*root, |p| {
            let val = grid[p];
            let mut adj = vec![];
            let times_visited = visited[grid.point_to_idx(p)];
            if val == b'9' {
                p1_score += 1;
                p2_score += times_visited;
            } else {
                for ortho in p
                    .orthogonals()
                    .into_iter()
                    .filter(|p| grid.is_valid_point(*p) && grid[*p] == val + 1)
                {
                    if visited[grid.point_to_idx(ortho)] == 0 {
                        adj.push(ortho);
                    }
                    visited[grid.point_to_idx(ortho)] += times_visited;
                }
            }
            Some(adj)
        });
    }

    (p1_score, p2_score)
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
    fn test_day9() {
        assert_eq!(solve(&TEST_GRID), (36, 81));
    }
}
