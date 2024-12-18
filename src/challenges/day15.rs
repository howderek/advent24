use advent24::bytegrid::{ByteGrid, Point, EAST, NORTH, SOUTH, WEST};
use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day15/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

fn take_step(world: &mut ByteGrid, from: Point, towards: u8) -> Option<Point> {
    let my_char = world[from];
    if let Some(heading) = match towards {
        b'^' => Some(NORTH),
        b'>' => Some(EAST),
        b'<' => Some(WEST),
        b'v' => Some(SOUTH),
        _ => None,
    } {
        let next_pos = from + heading;
        let behind_next_pos = from + heading + heading;
        if !world.is_valid_point(behind_next_pos) {
            return Some(from);
        }
        match (world[next_pos], world[behind_next_pos]) {
            (b'O', b'.') => {
                world[from] = b'.';
                world[next_pos] = my_char;
                world[behind_next_pos] = b'O';
                Some(next_pos)
            }
            (b'O', b'O') => {
                if let Some(point) = take_step(world, next_pos, towards) {
                    if point != next_pos {
                        world[from] = b'.';
                        world[next_pos] = my_char;
                        world[behind_next_pos] = b'O';
                        Some(next_pos)
                    } else {
                        Some(from)
                    }
                } else {
                    Some(from)
                }
            }
            (b'.', b'.') | (b'.', b'#') | (b'.', b'O') => {
                world[from] = b'.';
                world[next_pos] = my_char;
                Some(next_pos)
            }
            _ => Some(from),
        }
    } else {
        None
    }
}

pub fn part1(input: &str) -> i32 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut world = ByteGrid::new(parts[0]);
    let bytes_to_points = world.bytes_to_points();
    let mut character = bytes_to_points[&b'@'][0];
    println!("{}\n", &world);
    for c in parts[1].chars() {
        if let Some(p) = take_step(&mut world, character, c as u8) {
            println!("{}\n", &world);
            character = p;
        }
    }
    let updated_bytes_to_points = world.bytes_to_points();
    let mut score = 0;
    println!("{}\n", &world);
    for p in updated_bytes_to_points[&b'O'].iter() {
        score += 100 * p.row + p.col;
    }
    score
}

pub fn push_vert(world: &mut ByteGrid, from: Point, direction: Point) -> bool {
    let backup = world.data.clone();
    let from_byte = world[from];
    let points_to_check = match from_byte {
        b'[' => [from + direction, from + direction + EAST],
        b']' => [from + direction + WEST, from + direction],
        _ => unreachable!(),
    };
    for p in points_to_check {
        if world.is_valid_point(p) {
            let next_byte = world[p];
            match next_byte {
                b'[' | b']' => {
                    if !push_vert(world, p, direction) {
                        world.data = backup;
                        return false;
                    }
                }
                b'.' => continue,
                _ => {
                    world.data = backup;
                    return false;
                }
            }
        }
    }
    world[points_to_check[0]] = b'[';
    world[points_to_check[1]] = b']';
    world[from] = b'.';
    match from_byte {
        b'[' => world[from + EAST] = b'.',
        b']' => world[from + WEST] = b'.',
        _ => unreachable!(),
    }
    true
}

pub fn push_east(world: &mut ByteGrid, from: Point) -> bool {
    let p = from + EAST * 2;
    if world.is_valid_point(p) {
        let next_byte = world[p];
        match next_byte {
            b'[' => {
                if !push_east(world, p) {
                    return false;
                }
            }
            b'.' => (),
            _ => return false,
        }
    }
    world[p] = b']';
    world[p - EAST] = b'[';
    world[from] = b'.';
    true
}

pub fn push_west(world: &mut ByteGrid, from: Point) -> bool {
    let p = from + WEST * 2;
    if world.is_valid_point(p) {
        let next_byte = world[p];
        match next_byte {
            b']' => {
                if !push_west(world, p) {
                    return false;
                }
            }
            b'.' => (),
            _ => return false,
        }
    }
    world[p] = b'[';
    world[p - WEST] = b']';
    world[from] = b'.';
    true
}

fn take_step_part2(world: &mut ByteGrid, from: Point, towards: u8) -> Option<Point> {
    let my_char = world[from];
    if let Some(heading) = match towards {
        b'^' => Some(NORTH),
        b'>' => Some(EAST),
        b'<' => Some(WEST),
        b'v' => Some(SOUTH),
        _ => None,
    } {
        let next_pos = from + heading;
        match world[next_pos] {
            b'[' | b']' => match heading {
                NORTH | SOUTH => {
                    if push_vert(world, next_pos, heading) {
                        world[from] = b'.';
                        world[next_pos] = my_char;
                        Some(next_pos)
                    } else {
                        Some(from)
                    }
                }
                EAST => {
                    if push_east(world, next_pos) {
                        world[from] = b'.';
                        world[next_pos] = my_char;
                        Some(next_pos)
                    } else {
                        Some(from)
                    }
                }
                WEST => {
                    if push_west(world, next_pos) {
                        world[from] = b'.';
                        world[next_pos] = my_char;
                        Some(next_pos)
                    } else {
                        Some(from)
                    }
                }
                _ => unreachable!(),
            },
            b'.' => {
                world[from] = b'.';
                world[next_pos] = my_char;
                Some(next_pos)
            }
            _ => Some(from),
        }
    } else {
        None
    }
}

pub fn part2(input: &str) -> i32 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut new_world = parts[0].to_owned();
    new_world = new_world.replace("#", "##");
    new_world = new_world.replace(".", "..");
    new_world = new_world.replace("O", "[]");
    new_world = new_world.replace("@", "@.");
    let mut world = ByteGrid::new(&new_world);

    let bytes_to_points = world.bytes_to_points();
    let mut character = bytes_to_points[&b'@'][0];
    println!("{}\n", &world);
    for c in parts[1].chars() {
        if let Some(p) = take_step_part2(&mut world, character, c as u8) {
            character = p;
            println!("{}\n", &world);
        }
    }
    let updated_bytes_to_points = world.bytes_to_points();
    let mut score = 0;
    println!("{}\n", &world);
    for p in updated_bytes_to_points[&b'['].iter() {
        score += 100 * p.row + p.col;
    }
    score
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

    const TEST_GRID: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const TEST_GRID_2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const TEST_GRID_3: &str = "\
#######
#..#..#
#...O.#
#..OO.#
#.@O..#
#.O...#
#.O...#
#.....#
#######

v^>>v>^<^>>>v>^^<<<<<<<v<^^v<<<^^";

    const TEST_GRID_4: &str = "\
######
#....#
#..#.#
#....#
#.O..#
#.OO@#
#.O..#
#....#
######

<vv<<^^^";

    #[test]
    fn test_day9() {
        assert_eq!(part1(TEST_GRID), 2028);
        assert_eq!(part1(TEST_GRID_2), 10092);
        assert_eq!(part2(TEST_GRID_2), 9021);
        assert_eq!(part2(TEST_GRID_3), 1732);
        assert_eq!(part2(TEST_GRID_4), 1216);
    }
}
