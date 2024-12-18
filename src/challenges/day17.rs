use advent24::extract_numbers;
use clap;
use itertools::Itertools;
use std::{fs, iter::zip, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day17/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Literal(u64),
    Register(u64),
    Reserved,
    Invalid,
}

impl Operand {
    pub fn combo(input: u64) -> Self {
        match input {
            0..=3 => Self::Literal(input),
            4..=6 => Self::Register(input - 4),
            7 => Self::Reserved,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub program: Vec<u64>,
    pub output: Vec<u64>,
    pub i: usize,
    pub registers: [u64; 3],
}

static REG_A: usize = 0;
static REG_B: usize = 1;
static REG_C: usize = 2;

impl Computer {
    pub fn new(program: Vec<u64>, registers: [u64; 3]) -> Self {
        Computer {
            program,
            output: vec![],
            i: 0,
            registers,
        }
    }

    pub fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let program: Vec<u64> = lines[4]
            .replace("Program: ", "")
            .split(",")
            .map(|c| c.chars().next().unwrap().to_digit(10).unwrap() as u64)
            .collect();
        Computer::new(
            program,
            [
                extract_numbers(lines[0])[0],
                extract_numbers(lines[1])[0],
                extract_numbers(lines[2])[0],
            ],
        )
    }

    pub fn operand_to_value(&mut self, op: Operand) -> u64 {
        match op {
            Operand::Literal(v) => v,
            Operand::Register(r) if r < 3 => self.registers[r as usize],
            _ => {
                dbg!(op);
                dbg!(self);
                panic!("invalid state reached");
            }
        }
    }

    pub fn execute(&mut self) -> bool {
        if self.i >= self.program.len() {
            return false;
        }
        match self.program[self.i] {
            0 => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
                // The denominator is found by raising 2 to the power of the instruction's combo operand.
                let op = Operand::combo(self.program[self.i + 1]);
                self.registers[REG_A] /= u64::pow(2, self.operand_to_value(op) as u32);
                self.i += 2;
            }
            1 => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's
                // literal operand, then stores the result in register B.
                self.registers[REG_B] ^= self.program[self.i + 1];
                self.i += 2;
            }
            2 => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
                // then writes that value to the B register.
                let op = Operand::combo(self.program[self.i + 1]);
                self.registers[REG_B] = self.operand_to_value(op) % 8;
                self.i += 2;
            }
            3 => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. f the A
                // register is not zero, it jumps by setting the instruction pointer to the value of
                // its literal operand if this instruction jumps, the instruction pointer is not
                // increased by 2 after this instruction.
                if self.registers[REG_A] != 0 {
                    self.i = self.program[self.i + 1] as usize;
                } else {
                    self.i += 2;
                }
            }
            4 => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
                // register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                self.registers[REG_B] ^= self.registers[REG_C];
                self.i += 2;
            }
            5 => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
                // then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                let op = Operand::combo(self.program[self.i + 1]);
                let v = self.operand_to_value(op);
                self.output.push(v % 8);
                self.i += 2;
            }
            6 => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that
                // the result is stored in the B register. (The numerator is still read from the A register.)
                let op = Operand::combo(self.program[self.i + 1]);
                self.registers[REG_B] =
                    self.registers[REG_A] / u64::pow(2, self.operand_to_value(op) as u32);
                self.i += 2;
            }
            7 => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that
                // the result is stored in the C register. (The numerator is still read from the A register.)
                let op = Operand::combo(self.program[self.i + 1]);
                self.registers[REG_C] =
                    self.registers[REG_A] / u64::pow(2, self.operand_to_value(op) as u32);
                self.i += 2;
            }
            _ => unreachable!(),
        }
        true
    }

    pub fn execute_verbose(&mut self) -> bool {
        if self.i >= self.program.len() {
            return false;
        }
        print!("program [ ");
        for (i, p) in self.program.iter().enumerate() {
            if i == self.i {
                print!("\x1b[0;32m{p} ");
            } else if i == self.i + 1 {
                print!("{p} \x1b[0m");
            } else {
                print!("{p} ");
            }
        }
        println!(
            "]     registers [A {:8}]  [B {:8}]  [C {:8}]",
            self.registers[REG_A], self.registers[REG_B], self.registers[REG_C]
        );
        match self.program[self.i] {
            0 => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
                // The denominator is found by raising 2 to the power of the instruction's combo operand.
                let op = Operand::combo(self.program[self.i + 1]);
                let denom = u64::pow(2, self.operand_to_value(op) as u32);
                println!(
                    "adv {:?} ({}): storing {} == {} / {} in A",
                    op,
                    self.operand_to_value(op),
                    self.registers[REG_A] / denom,
                    self.registers[REG_A],
                    denom
                );
                self.registers[REG_A] /= denom;
                self.i += 2;
            }
            1 => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's
                // literal operand, then stores the result in register B.
                println!(
                    "bxl {}: storing {} == {} ^ {} in B",
                    self.program[self.i + 1],
                    self.registers[REG_B] ^ self.program[self.i + 1],
                    self.registers[REG_B],
                    self.program[self.i + 1]
                );
                self.registers[REG_B] ^= self.program[self.i + 1];
                self.i += 2;
            }
            2 => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
                // then writes that value to the B register.
                let op = Operand::combo(self.program[self.i + 1]);
                let val = self.operand_to_value(op);
                println!(
                    "bst {:?} ({}): storing {} == {} % 8 in B",
                    op.clone(),
                    val,
                    val % 8,
                    val
                );
                self.registers[REG_B] = val % 8;
                self.i += 2;
            }
            3 => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. f the A
                // register is not zero, it jumps by setting the instruction pointer to the value of
                // its literal operand. if this instruction jumps, the instruction pointer is not
                // increased by 2 after this instruction.
                if self.registers[REG_A] != 0 {
                    println!("jnz: jumping to {}", self.program[self.i + 1]);
                    self.i = self.program[self.i + 1] as usize;
                } else {
                    println!("jnz: skipping jump");
                    self.i += 2;
                }
            }
            4 => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
                // register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                println!(
                    "bxc: storing {} == {} ^ {} in B",
                    self.registers[REG_B] ^ self.registers[REG_C],
                    self.registers[REG_B],
                    self.registers[REG_C]
                );
                self.registers[REG_B] ^= self.registers[REG_C];
                self.i += 2;
            }
            5 => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
                // then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                let op = Operand::combo(self.program[self.i + 1]);
                let v = self.operand_to_value(op);
                println!(
                    "out: storing {} == {} % 8 in output [ {} \x1b[0;32m{}\x1b[0;0m ]",
                    v % 8,
                    v,
                    self.output.iter().join(" "),
                    v % 8
                );
                self.output.push(v % 8);
                self.i += 2;
            }
            6 => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that
                // the result is stored in the B register. (The numerator is still read from the A register.)
                let op = Operand::combo(self.program[self.i + 1]);
                let denom = u64::pow(2, self.operand_to_value(op) as u32);
                println!(
                    "bdv {:?} ({}): storing {} == {} / {} in B",
                    op,
                    self.operand_to_value(op),
                    self.registers[REG_A] / denom,
                    self.registers[REG_A],
                    denom
                );
                self.registers[REG_B] =
                    self.registers[REG_A] / u64::pow(2, self.operand_to_value(op) as u32);
                self.i += 2;
            }
            7 => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that
                // the result is stored in the C register. (The numerator is still read from the A register.)
                let op = Operand::combo(self.program[self.i + 1]);
                let denom = u64::pow(2, self.operand_to_value(op) as u32);
                println!(
                    "cdv {:?} ({}): storing {} == {} / {} in C",
                    op,
                    self.operand_to_value(op),
                    self.registers[REG_A] / denom,
                    self.registers[REG_A],
                    denom
                );
                self.registers[REG_C] =
                    self.registers[REG_A] / u64::pow(2, self.operand_to_value(op) as u32);
                self.i += 2;
            }
            _ => unreachable!(),
        }
        println!();
        true
    }

    pub fn execute_to_end(&mut self) {
        while self.execute() {}
    }

    pub fn reset(&mut self, registers: [u64; 3]) {
        self.registers = registers;
        self.i = 0;
        self.output.clear();
    }
}

pub fn part1(input: &str) -> String {
    let mut computer = Computer::parse(input);
    while computer.execute_verbose() {}
    computer.output.iter().join(",")
}

pub fn part2(input: &str) -> String {
    let orig_computer = Computer::parse(input);
    let mut computer = orig_computer.clone();
    let desired: [u64; 16] = [2, 4, 1, 1, 7, 5, 0, 3, 1, 4, 4, 4, 5, 5, 3, 0];
    // i started out with all zeros and as I got better sim scores I added to the
    // the right side of this
    let input: [u64; 16] = [5, 0, 0, 0, 0, 3, 2, 7, 5, 6, 0, 2, 5, 0, 5, 2];
    let mut base = 0;
    let mut most_similar = 0;
    for j in &input {
        base <<= 3;
        base |= j;
    }
    for i in 0..u64::pow(8, 8) {
        let reg_a = base | (i << 33);
        computer.reset([reg_a, 0, 0]);
        computer.execute_to_end();
        let similarity_score: u64 = zip(computer.output.iter(), desired.iter())
            .map(|(a, b)| if a == b { 1 } else { 0 })
            .sum();
        if similarity_score > most_similar {
            most_similar = similarity_score;
            println!(
                "{reg_a:016b} {:?} sim_score={}",
                computer.output, similarity_score
            )
        }
        if computer.output == desired {
            return format!("{reg_a}");
        }
    }
    "not found".to_string()
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
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const TEST_GRID_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn test_day17() {
        assert_eq!(part1(TEST_GRID), "4635635210");
        assert_eq!(part1(TEST_GRID_2), "42567777310");

        let mut computer = Computer::new(vec![2, 6], [0, 0, 9]);
        computer.execute_to_end();
        assert_eq!(computer.registers[REG_B], 1);

        let mut computer = Computer::new(vec![5, 0, 5, 1, 5, 4], [10, 0, 0]);
        computer.execute_to_end();
        assert_eq!(computer.output, vec![0, 1, 2]);

        let mut computer = Computer::new(vec![1, 7], [0, 29, 0]);
        computer.execute_to_end();
        assert_eq!(computer.registers[REG_B], 26);

        let mut computer = Computer::new(vec![4, 0], [0, 2024, 43690]);
        computer.execute_to_end();
        assert_eq!(computer.registers[REG_B], 44354);
    }
}
