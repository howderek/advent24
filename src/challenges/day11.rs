use advent24::parse_number_list;
use clap;
use std::{collections::HashMap, fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day11/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

pub fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut res: HashMap<u64, u64> = HashMap::new();
    for stone in stones.keys() {
        let count = *stones.get(stone).unwrap();
        let nstr = format!("{}", stone);
        match stone {
            0 => {
                res.entry(1).and_modify(|v| *v += count).or_insert(count);
            }
            _ if nstr.len() % 2 == 0 && nstr.len() >= 2 => {
                let first_half: u64 = (&nstr)[0..nstr.len() / 2].parse().unwrap();
                res.entry(first_half)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
                let second_half: u64 = (&nstr)[nstr.len() / 2..nstr.len()].parse().unwrap();
                res.entry(second_half)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            }
            n => {
                res.entry(n * 2024)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            }
        }
    }
    res
}

pub fn part1(input: &str) -> u64 {
    let stones: Vec<u64> = parse_number_list(input);
    let mut stone_map: HashMap<u64, u64> = HashMap::new();
    for s in stones.iter() {
        stone_map.entry(*s).and_modify(|v| *v += 1).or_insert(1);
    }
    for _ in 0..25 {
        stone_map = blink(stone_map);
    }
    stone_map.values().sum::<u64>() as u64
}

pub fn part2(input: &str) -> u64 {
    let stones: Vec<u64> = parse_number_list(input);
    let mut stone_map: HashMap<u64, u64> = HashMap::new();
    for s in stones.iter() {
        stone_map.entry(*s).and_modify(|v| *v += 1).or_insert(1);
    }
    for _ in 0..75 {
        stone_map = blink(stone_map);
    }
    stone_map.values().sum::<u64>() as u64
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

    #[test]
    fn test_day11() {
        assert_eq!(part1(&"125 17"), 55312);
    }
}
