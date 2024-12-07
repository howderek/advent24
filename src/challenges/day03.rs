use advent24::parse_number_list;
use clap;
use regex::Regex;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day3/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

fn challenge(input: String) -> (i64, i64) {
    let multiply = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_instr = Regex::new(r"do\(\)").unwrap();
    let dont_instr = Regex::new(r"don't\(\)").unwrap();

    let cleaned = multiply.replace_all(&input, "\nVALID: mul $1 $2\n");
    let cleaned2 = do_instr.replace_all(&cleaned, "\nVALID: enable\n");
    let cleaned3 = dont_instr.replace_all(&cleaned2, "\nVALID: disable\n");

    let mut enabled = true;
    let mut part1 = 0;
    let mut part2 = 0;

    for line in cleaned3.lines() {
        if line.len() < 8 {
            continue;
        }
        match &line[0..8] {
            "VALID: m" => {
                let parts: Vec<i64> = parse_number_list(line);
                if enabled {
                    part2 += parts[0] * parts[1];
                }
                part1 += parts[0] * parts[1];
            }
            "VALID: d" => {
                enabled = false;
            }
            "VALID: e" => {
                enabled = true;
            }
            _ => {}
        }
    }

    (part1, part2)
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let (part1, part2) = challenge(input);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
