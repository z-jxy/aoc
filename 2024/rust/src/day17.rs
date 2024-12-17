use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[repr(u8)]
enum Instr {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instr {
    fn from(value: u8) -> Self {
        match value {
            0 => Instr::Adv,
            1 => Instr::Bxl,
            2 => Instr::Bst,
            3 => Instr::Jnz,
            4 => Instr::Bxc,
            5 => Instr::Out,
            6 => Instr::Bdv,
            7 => Instr::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[inline(always)]
fn adv(a: usize, b: u32) -> usize {
    a / 2usize.pow(b) as usize
}

impl VM {
    #[inline(always)]
    fn a(&self) -> usize {
        self.registers[0]
    }

    #[inline(always)]
    fn b(&self) -> usize {
        self.registers[1]
    }

    #[inline(always)]
    fn c(&self) -> usize {
        self.registers[2]
    }

    #[inline(always)]
    fn b_mut(&mut self) -> &mut usize {
        &mut self.registers[1]
    }

    #[inline(always)]
    fn c_mut(&mut self) -> &mut usize {
        &mut self.registers[2]
    }

    #[inline(always)]
    fn combo_operand(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.a(),
            5 => self.b(),
            6 => self.c(),
            7 => panic!("Reserved operand"),
            _ => panic!("Invalid operand"),
        }
    }

    #[inline(always)]
    fn exec_instr(&mut self, instr: Instr, operand: u8) {
        match instr {
            Instr::Adv => {
                self.registers[0] = adv(self.a(), self.combo_operand(operand) as u32);
            }
            Instr::Bxl => {
                self.registers[1] = self.registers[1] ^ operand as usize;
            }
            Instr::Bst => {
                *self.b_mut() = self.combo_operand(operand) % 8;
            }
            Instr::Jnz => {
                if self.registers[0] != 0 {
                    self.ip = operand as usize;
                    return;
                }
            }
            Instr::Bxc => {
                *self.b_mut() = self.b() ^ self.c();
            }
            Instr::Out => {
                self.output.push((self.combo_operand(operand) % 8) as u8);
            }
            Instr::Bdv => {
                *self.b_mut() = adv(self.a(), self.combo_operand(operand) as u32);
            }
            Instr::Cdv => {
                *self.c_mut() = adv(self.a(), self.combo_operand(operand) as u32);
            }
        }

        self.ip += 2;
    }

    fn best_corrupted_a(&self) -> usize {
        let mut quines = HashSet::new();
        quines.insert(0);

        for &num in self.program.iter().rev() {
            let mut new_quines = HashSet::new();
            for curr in quines {
                for i in 0..8 {
                    let new = (curr << 3) + i;
                    let output = Self::hack(new);
                    if output == num as usize {
                        new_quines.insert(new);
                    }
                }
            }

            if new_quines.is_empty() {
                panic!("No valid corrupted A found");
            }

            quines = new_quines;
        }

        *quines.iter().min().unwrap_or(&0)
    }

    /*

        [
          1: Frame { registers: [777, 0, 0], ip: 0, instr: Bst, operand: 4, combo_operand: 777 },
          2: Frame { registers: [777, 1, 0], ip: 2, instr: Bxl, operand: 5, combo_operand: 1 },
          3: Frame { registers: [777, 4, 0], ip: 4, instr: Cdv, operand: 5, combo_operand: 4 },
          4: Frame { registers: [777, 4, 48], ip: 6, instr: Adv, operand: 3, combo_operand: 3 },
          5: Frame { registers: [97, 4, 48], ip: 8, instr: Bxc, operand: 1, combo_operand: 1 },
          6: Frame { registers: [97, 52, 48], ip: 10, instr: Bxl, operand: 6, combo_operand: 48 },
          7: Frame { registers: [97, 50, 48], ip: 12, instr: Out, operand: 5, combo_operand: 50 }
        ]
        ===========================
        1: B = A % 8
        2: B = B ^ 5
        3: C = A >> B
        4: A = A >> 3
        5: B = B ^ C
        6: B = B ^ 6
        ---------------------------
        // let b = a % 8;
        // let b = b ^ 5;
        // let c = a >> b;
        // let b = b ^ c;
        // let b = b ^ 6;
    */
    #[inline(always)]
    fn hack(a: usize) -> usize {
        let mut b = a % 8 ^ 5;
        b ^= a >> b;
        b ^= 6;

        b % 8
    }
}

#[derive(Clone, Default)]
struct VM {
    registers: [usize; 3],
    program: Vec<u8>,
    ip: usize,
    output: Vec<u8>,
}

/// Used to initially reverse operations for [`VM::hack`]
#[allow(dead_code)]
struct Frame {
    registers: [usize; 3],
    ip: usize,
    instr: Instr,
    operand: u8,
    combo_operand: usize,
}

#[aoc_generator(day17)]
fn parse(input: &str) -> VM {
    let mut lines = input.lines();

    let mut registers = [0; 3];

    for i in 0..3 {
        let line = lines.next().unwrap();
        let value = line.split_whitespace().last().unwrap().parse().unwrap();
        registers[i] = value;
    }

    _ = lines.next().unwrap();

    let program = lines.next().unwrap().split_once(": ").unwrap().1;
    let program = program.split(',').map(|x| x.parse().unwrap()).collect_vec();

    VM {
        registers,
        program,
        ..Default::default()
    }
}

#[aoc(day17, part1)]
fn part1(input: &VM) -> String {
    let mut vm = input.clone();

    while vm.ip < input.program.len() {
        let [opcode, operand] = input.program[vm.ip..vm.ip + 2] else {
            panic!("Invalid program");
        };
        vm.exec_instr(Instr::from(opcode), operand);
    }

    let output = vm
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    output
}

#[aoc(day17, part2)]
fn part2(vm: &VM) -> usize {
    let a = vm.best_corrupted_a();

    assert_eq!(a, 109020013201563, "This solution is input dependent");

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(&EXAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }
}
