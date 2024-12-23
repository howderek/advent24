use clap::{self, builder::Str};
use min_max_heap::MinMaxHeap;
use std::{
    collections::{HashMap, VecDeque},
    fs, str,
};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day19/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

pub fn ways_to_make(word: String, alphabet: &[&str]) -> i64 {
    let mut candidates = MinMaxHeap::new();
    let mut idx_to_counts = HashMap::new();
    let mut ways = 0;
    candidates.push(0);
    idx_to_counts.insert(0, 1);
    while let Some(idx) = candidates.pop_min() {
        for chunk in alphabet.iter() {
            if idx + chunk.len() <= word.len() && word[idx..idx + chunk.len()] == **chunk {
                let cur_idx_count: i64 = idx_to_counts[&idx];
                if idx + chunk.len() >= word.len() {
                    ways += cur_idx_count;
                } else if !chunk.is_empty() {
                    let new_idx = idx + chunk.len();
                    if let Some(k) = idx_to_counts.get_mut(&new_idx) {
                        *k += cur_idx_count;
                    } else {
                        candidates.push(new_idx);
                        idx_to_counts.insert(new_idx, cur_idx_count);
                    }
                }
            }
        }
    }
    ways
}

pub fn solver(input: &str) -> (i64, i64) {
    let mut parts = input.split("\n\n");
    let alphabet: Vec<&str> = parts.next().unwrap().split(", ").collect();
    let candidates: Vec<&str> = parts.next().unwrap().lines().collect();
    let mut is_made_with_alphabet_count: i64 = 0;
    let mut ways = 0;
    for word in candidates {
        let new_ways = ways_to_make(word.to_owned(), &alphabet);
        if new_ways > 0 {
            println!("✅ - {word} is made with alphabet");
            is_made_with_alphabet_count += 1;
            ways += new_ways;
        } else {
            println!("🚫 - {word} is not made with alphabet");
        }
    }
    (is_made_with_alphabet_count, ways)
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let (p1, p2) = solver(&input);
    println!("part1: {}", p1);
    println!("part1: {}", p2);
    println!("this day has not yet been implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRID: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test() {
        assert_eq!(
            ways_to_make("derek".to_string(), &["de", "d", "e", "r", "k", "ek"]),
            4
        );
        assert_eq!(ways_to_make("derek".to_string(), &["d", "e", "k"]), 0);
        assert_eq!(solver(TEST_GRID), (6, 16));
    }
}
