use advent24::parse_number_list;
use clap;
use itertools::Itertools;
use std::{fs, i128, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day13/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

#[derive(Debug, Default)]
pub struct ClawMachine {
    pub button_a: (i128, i128),
    pub button_b: (i128, i128),
    pub prize: (i128, i128),
}

// impl PartialOrd for ClawMachine {
//     fn partial_cmp(&self, other: &Self) -> Ordering {
//         self.prize.partial_cmp(&other.prize)
//     }
// }

impl ClawMachine {
    pub fn token_cost(&self, max: i128) -> i128 {
        let a1 = self.button_a.0;
        let b1 = self.button_b.0;
        let c1 = self.prize.0;
        let a2 = self.button_a.1;
        let b2 = self.button_b.1;
        let c2 = self.prize.1;

        let a_buttons_numer = (c1 * b2) - (b1 * c2);
        let a_buttons_denom = (a1 * b2) - (b1 * a2);
        let mut a_buttons = 0;

        if a_buttons_denom != 0 && a_buttons_numer % a_buttons_denom == 0 {
            a_buttons = a_buttons_numer / a_buttons_denom;
            //     println!(
            //         "
            // ({:<6} * {:<6}) - ({:<6} * {:<6})        {:<6}
            // --------------------------------------   =   --------  = {}
            // ({:<6} * {:<6}) - ({:<6} * {:<6})        {:<6}
            // ",
            //         c1, b2, b1, c2, a_buttons_numer, a_buttons, a1, b2, b1, a2, a_buttons_denom
            //     );
            if a_buttons > max || a_buttons < 0 {
                return 0;
            }
        }

        let b_buttons_numer = (a1 * c2) - (c1 * a2);
        let b_buttons_denom = (a1 * b2) - (b1 * a2);
        let mut b_buttons = 0;

        if b_buttons_denom != 0 && b_buttons_numer % b_buttons_denom == 0 {
            b_buttons = b_buttons_numer / b_buttons_denom;
            //     println!(
            //         "
            // ({:<6} * {:<6}) - ({:<6} * {:<6})        {:<6}
            // --------------------------------------   =   --------  = {}
            // ({:<6} * {:<6}) - ({:<6} * {:<6})        {:<6}
            //     ",
            //         a1, c2, c1, a2, b_buttons_numer, b_buttons, a1, b2, b1, a2, b_buttons_denom
            //     );
            if b_buttons > max || b_buttons < 0 {
                return 0;
            }
        }

        if (
            self.button_a.0 * a_buttons + self.button_b.0 * b_buttons,
            self.button_a.1 * a_buttons + self.button_b.1 * b_buttons,
        ) == (self.prize.0, self.prize.1)
        {
            return a_buttons * 3 + b_buttons;
        } else {
            return 0;
        }
    }
}

pub fn parse(input: &str) -> Vec<ClawMachine> {
    let mut res: Vec<ClawMachine> = vec![];
    for mut game_desc in &input.lines().chunks(4) {
        let button_a_str: String = game_desc
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10) || *c == ' ')
            .collect();
        let button_b_str: String = game_desc
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10) || *c == ' ')
            .collect();
        let prize_str: String = game_desc
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10) || *c == ' ')
            .collect();
        let a_params = parse_number_list(&button_a_str);
        let b_params = parse_number_list(&button_b_str);
        let prize_params = parse_number_list(&prize_str);
        res.push(ClawMachine {
            button_a: (a_params[0], a_params[1]),
            button_b: (b_params[0], b_params[1]),
            prize: (prize_params[0], prize_params[1]),
        });
    }
    res
}

pub fn part1(input: &str) -> i128 {
    let claw_machines = parse(input);
    let mut total_tokens = 0;
    for claw_machine in claw_machines {
        total_tokens += claw_machine.token_cost(100);
    }
    total_tokens
}

pub fn part2(input: &str) -> i128 {
    let claw_machines = parse(input);
    let mut total_tokens = 0;
    for mut claw_machine in claw_machines {
        claw_machine.prize.0 += 10000000000000;
        claw_machine.prize.1 += 10000000000000;
        total_tokens += claw_machine.token_cost(i128::MAX);
    }
    total_tokens
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
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_day9() {
        assert_eq!(part1(&TEST_GRID), 480);
    }
}
