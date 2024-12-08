use advent24::parse_number_list_delimited_by;
use clap;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day5/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

pub fn parse_graph(s: &str) -> HashMap<u64, Vec<u64>> {
    let mut res: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in s.lines() {
        let split: Vec<&str> = line.split('|').collect();
        if split.len() == 2 {
            res.entry(split[1].parse().unwrap())
                .or_insert_with(|| vec![])
                .push(split[0].parse().unwrap());
        }
    }
    res
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let graph = parse_graph(&input);
    let updates: Vec<Vec<u64>> = input
        .lines()
        .filter(|l| l.contains(","))
        .map(|l| parse_number_list_delimited_by(l, ","))
        .collect();
    let mut part1 = 0;
    let mut part2 = 0;
    println!("{:?}", graph);
    for update in updates.iter() {
        let mut is_valid = true;
        'outer: for (i, n) in update.iter().enumerate() {
            match graph.get(n) {
                Some(req_predecessors) => {
                    let actual_predecessors = &update[..i + 1];
                    for p in req_predecessors.iter() {
                        if update.contains(p) && !actual_predecessors.contains(p) {
                            is_valid = false;
                            break 'outer;
                        }
                    }
                }
                _ => (),
            }
        }
        if is_valid {
            println!("valid: {:?}", update);
            part1 += update[update.len() / 2];
        } else {
            println!("invalid: {:?}", update);
            let mut correct: Vec<u64> = update.clone();
            correct.sort_by(|a, b| {
                match graph.get(a) {
                    Some(preds) => {
                        if preds.contains(b) {
                            return Ordering::Greater;
                        }
                    }
                    None => (),
                }
                match graph.get(b) {
                    Some(preds) if preds.contains(a) => {
                        return Ordering::Less;
                    }
                    _ => return Ordering::Equal,
                }
            });
            println!("corrected: {:?}", correct);
            part2 += correct[correct.len() / 2];
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
