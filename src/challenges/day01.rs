use clap;
use regex::Regex;
use std::{collections::HashMap, fs, iter::zip, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day2/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

fn extract_lists(input: String) -> (Vec<i64>, Vec<i64>) {
    let re = Regex::new(r"(\d+)   (\d+)").unwrap();

    let mut left_list: Vec<i64> = vec![];
    let mut right_list: Vec<i64> = vec![];

    for (_, [left, right]) in re.captures_iter(&input).map(|c| c.extract()) {
        left_list.push(left.parse().unwrap());
        right_list.push(right.parse().unwrap());
    }

    (left_list, right_list)
}

fn vec_to_counts(v: &Vec<i64>) -> HashMap<i64, i64> {
    let mut res = HashMap::new();

    for k in v.iter() {
        *res.entry(*k).or_insert(0) += 1;
    }

    res
}

fn part1(input: String) -> Option<i64> {
    let (mut left_list, mut right_list) = extract_lists(input);
    left_list.sort();
    right_list.sort();

    let mut differences = 0;
    for (left, right) in zip(left_list, right_list) {
        differences += i64::abs(left - right)
    }

    Some(differences)
}

fn part2(input: String) -> Option<i64> {
    let (left_list, right_list) = extract_lists(input);
    let counts = vec_to_counts(&right_list);

    let mut similarity_score = 0;
    for number in left_list.iter() {
        similarity_score += *number * counts.get(number).unwrap_or(&0);
    }

    Some(similarity_score)
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let result: Option<i64>;
    if args.part2 {
        result = part2(input);
    } else {
        result = part1(input);
    }
    match result {
        Some(count) => println!("{}", count),
        None => println!("No digits found"),
    }
}
