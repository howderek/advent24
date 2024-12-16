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
        return match (world[next_pos], world[behind_next_pos]) {
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
                        return Some(next_pos);
                    } else {
                        return Some(from);
                    }
                } else {
                    return Some(from);
                }
            }
            (b'.', b'.') | (b'.', b'#') | (b'.', b'O') => {
                world[from] = b'.';
                world[next_pos] = my_char;
                Some(next_pos)
            }
            _ => return Some(from),
        };
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

fn take_step_part2(world: &mut ByteGrid, from: Point, towards: u8) -> Option<Point> {
    let my_char = world[from];
    if let Some(heading) = match towards {
        b'^' => Some(NORTH),
        b'>' => Some(EAST),
        b'<' => Some(WEST),
        b'v' => Some(SOUTH),
        _ => None,
    } {
        let next1_pos = from + heading;
        let next2_pos = next1_pos + heading;
        let next3_pos = next2_pos + heading;
        if !world.is_valid_point(next2_pos) || !world.is_valid_point(next3_pos) {
            return Some(from);
        }
        return match (world[next1_pos], world[next2_pos], world[next3_pos]) {
            (b'[', b']', b'.') => {
                world[from] = b'.';
                world[next1_pos] = my_char;
                world[next2_pos] = b'[';
                world[next3_pos] = b']';
                Some(next1_pos)
            }
            (b'[', b']', b'[') => {
                if let Some(point) = take_step(world, next1_pos, towards) {
                    if point != next1_pos {
                        world[from] = b'.';
                        world[next1_pos] = my_char;
                        world[next2_pos] = b'[';
                        world[next3_pos] = b']';
                        return Some(next1_pos);
                    } else {
                        return Some(from);
                    }
                } else {
                    return Some(from);
                }
            }
            (b']', b'[', b']') => {
                if let Some(point) = take_step(world, next1_pos, towards) {
                    if point != next1_pos {
                        world[from] = b'.';
                        world[next1_pos] = my_char;
                        world[next2_pos] = b']';
                        world[next3_pos] = b'[';
                        return Some(next1_pos);
                    } else {
                        return Some(from);
                    }
                } else {
                    return Some(from);
                }
            }
            (b']', b'.', _) => {
                world[from] = b'.';
                world[next1_pos] = my_char;
                world[next1_pos + WEST] = b'.';
                world[next2_pos] = b']';
                world[next2_pos + WEST] = b'[';
                Some(next1_pos)
            }
            (b'[', b'.', _) => {
                world[from] = b'.';
                world[next1_pos] = my_char;
                world[next1_pos + EAST] = b'.';
                world[next2_pos] = b'[';
                world[next2_pos + EAST] = b']';
                Some(next1_pos)
            }
            (b'.', _, _) => {
                world[from] = b'.';
                world[next1_pos] = my_char;
                Some(next1_pos)
            }
            _ => return Some(from),
        };
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
            println!("{}\n", &world);
            character = p;
        }
    }
    let updated_bytes_to_points = world.bytes_to_points();
    let mut score = 0;
    println!("{}\n", &world);
    for p in updated_bytes_to_points[&b'['].iter() {
        if p.col < world.width / 2 {
            score += 100 * p.row + p.col;
        }
    }
    for p in updated_bytes_to_points[&b']'].iter() {
        if p.col > world.width / 2 {
            score += 100 * p.row + p.col;
        }
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
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_day9() {
        assert_eq!(part1(&TEST_GRID), 2028);
        assert_eq!(part1(&TEST_GRID_2), 10092);
        assert_eq!(part2(&TEST_GRID_2), 9021);
    }
}
