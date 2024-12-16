#![feature(test)]

use std::collections::HashMap;
use std::hash::Hash;

use clap::builder::Str;

pub mod bytegrid;

pub fn parse_number_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split_whitespace()
        .into_iter()
        .flat_map(|x| x.parse())
        .collect()
}

pub fn extract_numbers<T: std::str::FromStr>(s: &str) -> Vec<T> {
    let new_s: String = s
        .chars()
        .filter_map(|c| {
            if c.is_digit(10) || c == ' ' || c == '-' {
                Some(c)
            } else {
                Some(' ')
            }
        })
        .collect();
    parse_number_list(&new_s)
}

pub fn parse_number_list_delimited_by<T: std::str::FromStr>(s: &str, delim: &str) -> Vec<T> {
    s.split(delim).into_iter().flat_map(|x| x.parse()).collect()
}

pub fn string_to_2d_array(s: String) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

pub fn print_2d_array(world: &Vec<Vec<char>>) {
    for line in world.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Hash)]
enum NodeOp {
    Visit,
    Mark,
}

pub fn dfs<T: Hash + Eq + Copy>(
    graph: &HashMap<T, Vec<T>>,
    roots: &Vec<T>,
    postorder: bool,
) -> Vec<T> {
    let mut marks: HashMap<T, NodeOp> = HashMap::new();
    let mut result: Vec<T> = vec![];
    for root in roots.iter() {
        let search_root = (NodeOp::Visit, root);
        let mut stack = vec![search_root];
        while let Some((op, node)) = stack.pop() {
            match op {
                NodeOp::Visit => match marks.get(node) {
                    Some(NodeOp::Mark) => continue,
                    Some(NodeOp::Visit) => {
                        marks.insert(*node, NodeOp::Mark);
                    }
                    None => {
                        marks.insert(*node, NodeOp::Visit);
                        if !postorder {
                            result.push(*node);
                        }
                        stack.push((NodeOp::Mark, node));
                        match graph.get(node) {
                            Some(sucessors) => stack.extend(
                                sucessors.iter().map(|successor| (NodeOp::Visit, successor)),
                            ),
                            _ => (),
                        }
                    }
                },
                NodeOp::Mark => {
                    // add permanent mark
                    marks.insert(*node, NodeOp::Mark);
                    if postorder {
                        result.push(*node);
                    }
                }
            }
        }
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    w: usize,
    h: usize,
}

impl Tile {
    pub fn new(world_width: usize, world_height: usize, x: usize, y: usize) -> Self {
        Tile {
            x,
            y,
            w: world_width,
            h: world_height,
        }
    }

    pub fn from_world(world: &Vec<Vec<char>>, x: usize, y: usize) -> Self {
        Tile {
            x,
            y,
            w: world[0].len(),
            h: world.len(),
        }
    }

    pub fn char_at(&self, world: &Vec<Vec<char>>) -> Option<char> {
        if self.y >= world.len() || self.x >= world[self.y].len() {
            None
        } else {
            Some(world[self.y][self.x])
        }
    }

    pub fn digit_at(&self, world: &Vec<Vec<char>>) -> Option<u32> {
        if self.y >= world.len() || self.x >= world[self.y].len() {
            None
        } else {
            world[self.y][self.x].to_digit(10)
        }
    }

    pub fn topleft(&self) -> Option<Self> {
        if self.x > 0 && self.y > 0 {
            return Some(Self::new(self.w, self.h, self.x - 1, self.y - 1));
        }
        return None;
    }

    pub fn top(&self) -> Option<Self> {
        if self.y > 0 {
            return Some(Self::new(self.w, self.h, self.x, self.y - 1));
        }
        return None;
    }

    pub fn topright(&self) -> Option<Self> {
        if self.x + 1 < self.w && self.y > 0 {
            return Some(Self::new(self.w, self.h, self.x + 1, self.y - 1));
        }
        return None;
    }

    pub fn left(&self) -> Option<Self> {
        if self.x > 0 {
            return Some(Self::new(self.w, self.h, self.x - 1, self.y));
        }
        return None;
    }

    pub fn right(&self) -> Option<Self> {
        if self.x + 1 < self.w {
            return Some(Self::new(self.w, self.h, self.x + 1, self.y));
        }
        return None;
    }

    pub fn bottomleft(&self) -> Option<Self> {
        if self.x > 0 && self.y + 1 < self.h {
            return Some(Self::new(self.w, self.h, self.x - 1, self.y + 1));
        }
        return None;
    }

    pub fn bottom(&self) -> Option<Self> {
        if self.y + 1 < self.h {
            return Some(Self::new(self.w, self.h, self.x, self.y + 1));
        }
        return None;
    }

    pub fn bottomright(&self) -> Option<Self> {
        if self.x + 1 < self.w && self.y + 1 < self.h {
            return Some(Self::new(self.w, self.h, self.x + 1, self.y + 1));
        }
        return None;
    }

    pub fn orthogonal(&self) -> Vec<Self> {
        [self.top(), self.left(), self.right(), self.bottom()]
            .into_iter()
            .flatten()
            .collect()
    }
}
