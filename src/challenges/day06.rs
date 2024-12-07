use advent24::{print_2d_array, string_to_2d_array, Tile};
use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day6/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

struct Character {
    pub current_pos: Tile,
    pub next_step: fn(&Tile) -> Option<Tile>,
}

impl Character {
    fn char_to_next_step(c: char) -> Option<fn(&Tile) -> Option<Tile>> {
        match c {
            '^' => Some(Tile::top),
            '>' => Some(Tile::right),
            '<' => Some(Tile::left),
            'v' => Some(Tile::bottom),
            _ => None,
        }
    }

    fn rotate_right(&mut self) -> &mut Self {
        if self.next_step == Tile::top {
            self.next_step = Tile::right;
        } else if self.next_step == Tile::right {
            self.next_step = Tile::bottom;
        } else if self.next_step == Tile::bottom {
            self.next_step = Tile::left;
        } else if self.next_step == Tile::left {
            self.next_step = Tile::top;
        }
        self
    }

    fn take_step(&mut self, world: &mut Vec<Vec<char>>) -> Option<u64> {
        if let Some(tile) = (self.next_step)(&self.current_pos) {
            match world[tile.y][tile.x] {
                '#' | 'O' => {
                    self.rotate_right();
                    return Some(0);
                }
                '.' | '1' | '2' | '3' | '4' => {
                    let mut tile_visited = 0;
                    match world[self.current_pos.y][self.current_pos.x] {
                        '.' | '^' | '>' | 'v' | '<' => {
                            world[self.current_pos.y][self.current_pos.x] = '1';
                            tile_visited = 1;
                        }
                        '1' => {
                            world[self.current_pos.y][self.current_pos.x] = '2';
                        }
                        '2' => {
                            world[self.current_pos.y][self.current_pos.x] = '3';
                        }
                        '3' => {
                            world[self.current_pos.y][self.current_pos.x] = '4';
                        }
                        '4' => {
                            world[self.current_pos.y][self.current_pos.x] = '5';
                            return None;
                        }
                        _ => return None,
                    }
                    self.current_pos = tile;
                    return Some(tile_visited);
                }
                _ => return None,
            }
        } else {
            world[self.current_pos.y][self.current_pos.x] = 'X';
            return None;
        }
    }
}

pub fn run_simulation(world: &mut Vec<Vec<char>>) -> u64 {
    let mut character = Character {
        current_pos: Tile::from_world(&world, 0, 0),
        next_step: Tile::top,
    };
    'outer: for (y, line) in world.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if let Some(valid_next_step) = Character::char_to_next_step(*c) {
                character.next_step = valid_next_step;
                character.current_pos.x = x;
                character.current_pos.y = y;
                break 'outer;
            }
        }
    }
    let mut total_positions_visited = 1;
    while let Some(position_visited) = character.take_step(world) {
        total_positions_visited += position_visited;
    }
    total_positions_visited
}

pub fn part1(args: &Args) -> u64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let mut world = string_to_2d_array(input);
    let res = run_simulation(&mut world);
    print_2d_array(&world);
    res
}

pub fn part2(args: &Args) -> u64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let world = string_to_2d_array(input);
    let mut total_causes_loop = 0;
    for y in 0..world.len() {
        for x in 0..world[0].len() {
            if world[y][x] == '.' {
                let mut new_world = world.clone();
                new_world[y][x] = 'O';
                run_simulation(&mut new_world);
                if new_world.iter().any(|l| l.iter().any(|c| *c == '5')) {
                    // print_2d_array(&new_world);
                    // println!();
                    total_causes_loop += 1;
                }
            }
        }
    }
    total_causes_loop
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
