use advent24::parse_number_list;
use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day7/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

pub fn part1(args: &Args) -> i64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let mut grand_total = 0;
    for line in input.lines() {
        let mut part_iter = line.split(":");
        let correct_total: i64 = part_iter.next().unwrap().parse().unwrap();
        let parts: Vec<i64> = parse_number_list(part_iter.next().unwrap());
        for i in 0..i64::pow(2, (parts.len() - 1) as u32) {
            let mut total = parts[0];
            for (j, part) in parts[1..].iter().enumerate() {
                if i & 1 << j > 0 {
                    total += part;
                } else {
                    total *= part;
                }
            }
            if total == correct_total {
                grand_total += total;
                break;
            }
        }
    }
    grand_total
}

pub fn part2(args: &Args) -> i64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let mut grand_total = 0;
    for line in input.lines() {
        let mut part_iter = line.split(":");
        let correct_total: i64 = part_iter.next().unwrap().parse().unwrap();
        let parts: Vec<i64> = parse_number_list(part_iter.next().unwrap());
        for i in 0..i64::pow(3, (parts.len() - 1) as u32) {
            let mut remaining = i;
            let mut total = parts[0];
            for (j, part) in parts[1..].iter().enumerate() {
                let place = i64::pow(3, (parts.len() - j - 2) as u32);
                let digit = (remaining / place) as i64;
                remaining = (remaining % place) as i64;
                total = match digit {
                    0 => total + part,
                    1 => total * part,
                    2 => total * (i64::pow(10, (*part as f64).log10() as u32 + 1)) + part,
                    _ => unreachable!(),
                }
            }
            if total == correct_total {
                grand_total += total;
                break;
            }
        }
    }
    grand_total
}

pub fn entrypoint(args: &Args) {
    if args.part2 {
        let res = part2(args);
        println!("{}", res);
    } else {
        let res = part1(args);
        println!("{}", res);
    }
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
