use itertools::Itertools;
use std::fmt::{Display, Formatter};

pub fn solve1(lines: &[String]) -> i64 {
    let mut program = Program::from(lines);
    let mut output = Vec::new();
    while program.index < program.program.len() {
        if let Some(val) = program.run() {
            output.push(val);
        }
    }
    println!("Solution: {}", output.iter().join(","));
    // todo: support non integer value returns for solves
    output.into_iter().join("").parse::<i64>().unwrap()
}

pub fn solve2(lines: &[String]) -> i64 {
    let mut program = Program::from(lines);
    /*
    My input program essentially boils down to the following:
        A ... -> B
        OUT B % 8
        A / 3 -> A
        LOOP if A > 0
        END
     */
    program.self_replication(0, 0).unwrap() as i64
}

#[derive(Debug)]
struct Program {
    index: usize,
    reg_a: u128,
    reg_b: u128,
    reg_c: u128,
    program: Vec<u128>,
}

impl Program {
    fn run(&mut self) -> Option<u128> {
        let mut result = None;
        while self.index < self.program.len() {
            let opcode_value = self.program[self.index];
            let operand = self.program[self.index + 1];
            let combo = match operand {
                0..4 => operand,
                4 => self.reg_a,
                5 => self.reg_b,
                6 => self.reg_c,
                7.. => 0,
            };
            match Opcode::from(opcode_value) {
                Opcode::Adv => self.reg_a /= 2u128.pow(combo as u32),
                Opcode::Bxl => self.reg_b ^= operand,
                Opcode::Bst => self.reg_b = combo % 8,
                Opcode::Bxc => self.reg_b ^= self.reg_c,
                Opcode::Bdv => self.reg_b = self.reg_a / 2u128.pow(combo as u32),
                Opcode::Cdv => self.reg_c = self.reg_a / 2u128.pow(combo as u32),
                Opcode::Jnz => {
                    if self.reg_a != 0 {
                        self.index = operand as usize;
                        break
                    }
                }
                Opcode::Out => result = Some(combo % 8),
            }
            self.index += 2;
        }
        result
    }

    fn self_replication(&mut self, index: u128, next: u128) -> Option<u128> {
        let prev_a = next * 8;
        for i in 0..8 {
            let possible_a = prev_a + i;
            self.index = 0;
            self.reg_a = possible_a;
            self.reg_b = 0;
            self.reg_c = 0;
            if let Some(output) = self.run() {
                let prog_index = self.program.len() - 1 - index as usize;
                if output == self.program[prog_index] {
                    if prog_index == 0 {
                        return Some(possible_a)
                    }
                    let result= self.self_replication(index + 1, possible_a);
                    if result.is_some() {
                        return result
                    }
                }
            }
        }
        None
    }
}


impl From<&[String]> for Program {
    // todo: make this not shit.  I'm just abusing the puzzle input layout lmao
    fn from(lines: &[String]) -> Self {
        Program {
            index: 0,
            reg_a: lines[0][12..].parse::<u128>().unwrap(),
            reg_b: lines[1][12..].parse::<u128>().unwrap(),
            reg_c: lines[2][12..].parse::<u128>().unwrap(),
            program: lines[4][9..]
                .split(",")
                .map(|s| s.parse::<u128>().unwrap())
                .collect_vec(),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program [")
            .and_then(|_| writeln!(f, "  index: {}", self.index))
            .and_then(|_| writeln!(f, "  reg_a: {}", self.reg_a))
            .and_then(|_| writeln!(f, "  reg_b: {}", self.reg_b))
            .and_then(|_| writeln!(f, "  reg_c: {}", self.reg_c))
            .and_then(|_| writeln!(f, "  program: {:?}", self.program))
            .and_then(|_| writeln!(f, "]"))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u128> for Opcode {
    fn from(value: u128) -> Self {
        match value {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => unreachable!(),
        }
    }
}