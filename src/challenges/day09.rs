use clap;
use itertools::PeekingNext;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day9/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Block {
    File { id: usize },
    Free,
}

pub fn parse_puzzle(input: &str) -> Vec<Block> {
    let mut output: Vec<Block> = vec![];
    let mut id = 0;
    for (i, c) in input.chars().enumerate() {
        if !c.is_ascii_digit() {
            println!("warn: bad char: {}", c);
            continue;
        }
        if c == '0' && i % 2 == 0 {
            println!("warn: 0 len file @ input[{}]", i);
        }
        for _k in 0..c.to_digit(10).unwrap() {
            if i % 2 == 0 {
                output.push(Block::File { id });
            } else {
                output.push(Block::Free)
            }
        }
        if i % 2 == 0 {
            id += 1;
        }
    }
    output
}

pub fn score_fs(fs: &[Block]) -> usize {
    let mut sum = 0;
    for (i, block) in fs.iter().enumerate() {
        if let Block::File { id } = block {
            sum += id * i;
        }
    }
    sum
}

pub fn part1(input: &str) -> usize {
    let fs = parse_puzzle(input);
    let mut new_fs: Vec<Block> = vec![];
    let fwd_iter = fs.iter();
    let mut rev_iter = fs.iter().rev();
    let mut seen_ids = HashSet::<usize>::new();
    let mut total_i = 0;
    for block in fwd_iter {
        match *block {
            Block::File { id } => {
                new_fs.push(*block);
                seen_ids.insert(id);
                total_i += 1;
            }
            Block::Free => {
                while let Some(Block::Free) = rev_iter.peeking_next(|x| **x == Block::Free) {
                    total_i += 1;
                }
                if total_i >= fs.len() {
                    break;
                }
                if let Block::File { id } = *rev_iter.next().unwrap() {
                    new_fs.push(Block::File { id });
                    total_i += 2;
                }
            }
        }
        if total_i >= fs.len() {
            break;
        }
    }
    score_fs(&new_fs)
}

#[derive(Debug, Default, Copy, Clone)]
pub struct File {
    id: usize,
    pos: usize,
    size: usize,
}

#[derive(Debug, Default)]
struct Defragmenter {
    pub gap_to_pos: [BinaryHeap<Reverse<usize>>; 9],
    pub files: Vec<File>,
}

impl Defragmenter {
    pub fn new(input: &str) -> Self {
        let mut s = Defragmenter::default();
        let mut pos = 0;
        let mut id = 0;
        for (i, c) in input.chars().enumerate() {
            if !c.is_ascii_digit() {
                println!("warn: bad char: {}", c);
                continue;
            }
            if c == '0' && i % 2 == 0 {
                println!("warn: 0 len file @ input[{}]", i);
            }
            let size = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                s.files.push(File { id, pos, size });
                id += 1;
            } else if size > 0 {
                s.gap_to_pos[size - 1].push(Reverse(pos));
            }
            pos += size;
        }
        s
    }

    pub fn defragment(&mut self) {
        for file in self.files.iter_mut().rev() {
            let mut size_for_smallest_pos = usize::MAX;
            let mut smallest_pos = usize::MAX;
            for size in file.size..10 {
                match self.gap_to_pos[size - 1].peek() {
                    Some(Reverse(pos)) if *pos < smallest_pos => {
                        smallest_pos = *pos;
                        size_for_smallest_pos = size;
                    }
                    _ => (),
                }
            }
            if smallest_pos < file.pos {
                // better position found
                if let Some(Reverse(new_pos)) = self.gap_to_pos[size_for_smallest_pos - 1].pop() {
                    if file.size < size_for_smallest_pos {
                        let remaining_size = size_for_smallest_pos - file.size;
                        let remaining_pos = new_pos + file.size;
                        self.gap_to_pos[remaining_size - 1].push(Reverse(remaining_pos));
                    }
                    file.pos = new_pos;
                }
            }
        }
    }
}

pub fn score_files(files: &Vec<File>) -> usize {
    let mut score = 0;
    for file in files {
        for i in 0..file.size {
            score += file.id * (file.pos + i)
        }
    }
    score
}

pub fn part2(input: &str) -> usize {
    let mut defragmenter = Defragmenter::new(input);
    defragmenter.defragment();
    score_files(&defragmenter.files)
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

    const TEST_GRID: &str = "2333133121414131402";

    #[test]
    fn test_day9_part1() {
        let p1 = part1(TEST_GRID);
        assert_eq!(p1, 1928);
    }

    #[test]
    fn test_day9_part2() {
        assert_eq!(part2(TEST_GRID), 2858);

        // correct scoring for ids > 9
        assert_eq!(part2("1010101010101010101010"), 385);

        // 0.1 -> 01. (simplest move)
        assert_eq!(part2("111"), 1);

        // 0.........1 -> 01......... (simplest move, max space)
        assert_eq!(part2("191"), 1);

        // 0..11 -> 011..
        assert_eq!(part2("122"), 3);

        // 0.12 -> 021.
        assert_eq!(part2("11101"), 4);

        // 0...12 -> 021...
        assert_eq!(part2("13101"), 4);

        // 00..11..22 -> 002211....
        assert_eq!(part2("22222"), 19);

        // 00..1.2.3 -> 00321....
        assert_eq!(part2("2211111"), 16);

        // 00..1.2.33.4 -> 00421...33..
        assert_eq!(part2("221111211"), 69);

        // 0..1.22...333 -> 0221...333...
        assert_eq!(part2("1211233"), 81);

        // 00.....11 -> 0011.....
        assert_eq!(part2("252"), 5);

        // 0.1.2.3 -> 0312..
        assert_eq!(part2("1111111"), 11);

        // 0...1223 -> 03221...
        assert_eq!(part2("1310201"), 17);

        assert_eq!(part2("354631466260"), 1325);
        assert_eq!(part2("171010402"), 88);
    }
}
