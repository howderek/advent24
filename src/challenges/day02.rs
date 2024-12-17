use advent24::parse_number_list;
use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day2/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

#[derive(Debug, PartialEq)]
enum PlantState {
    Initial(i64),
    SafeIncreasing(i64),
    SafeDecreasing(i64),
    Unsafe,
}

impl PlantState {
    fn next_state(&self, next: i64) -> Self {
        match self {
            Self::Initial(num) | Self::SafeIncreasing(num) if (next > *num && next <= *num + 3) => {
                Self::SafeIncreasing(next)
            }
            Self::Initial(num) | Self::SafeDecreasing(num) if (next < *num && next >= *num - 3) => {
                Self::SafeDecreasing(next)
            }
            _ => Self::Unsafe,
        }
    }
}

fn part1(input: String) -> i64 {
    let mut safe = 0;
    for line in input.lines() {
        let digits: Vec<i64> = parse_number_list(line);
        let state = digits[1..]
            .iter()
            .fold(PlantState::Initial(digits[0]), |s, item| {
                s.next_state(*item)
            });
        safe += match state {
            PlantState::SafeIncreasing(_) | PlantState::SafeDecreasing(_) => 1,
            _ => 0,
        }
    }
    safe
}

fn part2(input: String) -> i64 {
    let mut safe = 0;
    for line in input.lines() {
        let digits: Vec<i64> = parse_number_list(line);
        for one_to_skip in 0..digits.len() {
            let mut new_digits = vec![];
            for (i, n) in digits.iter().enumerate() {
                if i != one_to_skip {
                    new_digits.push(*n);
                }
            }
            let state = new_digits[1..]
                .iter()
                .fold(PlantState::Initial(new_digits[0]), |s, item| {
                    s.next_state(*item)
                });
            if state != PlantState::Unsafe {
                safe += 1;
                break;
            }
        }
    }
    safe
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let result: i64 = match args.part2 {
        true => part2(input),
        false => part1(input),
    };
    println!("{}", result);
}
