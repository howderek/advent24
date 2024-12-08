use advent24::{string_to_2d_array, Tile};
use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day4/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

pub fn part1(args: &Args) -> u64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let mut world = string_to_2d_array(input);
    let count = find_word_any_direction(&mut world, "XMAS");
    count
}

fn check_direction(
    tile: Tile,
    world: &Vec<Vec<char>>,
    direction_func: fn(&Tile) -> Option<Tile>,
    word: &str,
) -> u64 {
    let mut word_iter = word.chars();
    let first_char = tile.char_at(world).unwrap();
    if first_char != word_iter.next().unwrap() {
        return 0;
    }
    let mut t = tile;
    let mut len_checked = 1;
    while let Some(next_tile) = direction_func(&t) {
        let wordc_opt = word_iter.next();
        if wordc_opt.is_none() {
            break;
        }
        let wordc = wordc_opt.unwrap();
        let worldc = next_tile.char_at(world).unwrap();
        if worldc != wordc {
            return 0;
        }
        len_checked += 1;
        t = next_tile;
    }
    if len_checked == word.len() {
        return 1;
    } else {
        return 0;
    }
}

fn find_word_any_direction(world: &mut Vec<Vec<char>>, word: &str) -> u64 {
    let mut count = 0;
    for (y, line) in world.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let tile = Tile::from_world(world, x, y);
            count += check_direction(tile, world, Tile::right, word);
            count += check_direction(tile, world, Tile::bottomright, word);
            count += check_direction(tile, world, Tile::bottom, word);
            count += check_direction(tile, world, Tile::bottomleft, word);
            count += check_direction(tile, world, Tile::left, word);
            count += check_direction(tile, world, Tile::topleft, word);
            count += check_direction(tile, world, Tile::top, word);
            count += check_direction(tile, world, Tile::topright, word);
        }
    }
    count
}

pub fn part2(args: &Args) -> u64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let world = string_to_2d_array(input);
    let mut xmas_count = 0;
    for (y, line) in world[1..world.len() - 1].iter().enumerate() {
        for (x, c) in line[1..line.len() - 1].iter().enumerate() {
            if *c != 'A' {
                continue;
            }
            let topleft = world[y][x];
            let topright = world[y][x + 2];
            let bottomleft = world[y + 2][x];
            let bottomright = world[y + 2][x + 2];
            xmas_count += match (topleft, topright, bottomleft, bottomright) {
                ('M', 'M', 'S', 'S') => 1,
                ('M', 'S', 'M', 'S') => 1,
                ('S', 'M', 'S', 'M') => 1,
                ('S', 'S', 'M', 'M') => 1,
                _ => 0,
            }
        }
    }
    xmas_count
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
