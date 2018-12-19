use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Opcode {
    AddRegister,
    AddImmediate,
    MultiplyRegister,
    MultiplyImmediate,
    BitwiseANDRegister,
    BitwiseANDImmediate,
    BitwiseORRegister,
    BitwiseORImmediate,
    SetRegister,
    SetImmediate,
    GreaterThanImmediateRegister,
    GreaterThanRegisterImmediate,
    GreaterThanRegisterRegister,
    EqualImmediateRegister,
    EqualRegisterImmediate,
    EqualRegisterRegister,
}

const OPCODES: [Opcode; 16] = [
    Opcode::AddRegister,
    Opcode::AddImmediate,
    Opcode::MultiplyRegister,
    Opcode::MultiplyImmediate,
    Opcode::BitwiseANDRegister,
    Opcode::BitwiseANDImmediate,
    Opcode::BitwiseORRegister,
    Opcode::BitwiseORImmediate,
    Opcode::SetRegister,
    Opcode::SetImmediate,
    Opcode::GreaterThanImmediateRegister,
    Opcode::GreaterThanRegisterImmediate,
    Opcode::GreaterThanRegisterRegister,
    Opcode::EqualImmediateRegister,
    Opcode::EqualRegisterImmediate,
    Opcode::EqualRegisterRegister,
];

#[derive(Copy, Clone)]
struct Instruction {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    #[inline]
    fn run(self, registers: &mut [usize; 4]) {
        let Instruction { opcode, a, b, c } = self;
        match opcode {
            Opcode::AddRegister => {
                registers[c] = registers[a] + registers[b];
            }

            Opcode::AddImmediate => {
                registers[c] = registers[a] + b;
            }

            Opcode::MultiplyRegister => {
                registers[c] = registers[a] * registers[b];
            }

            Opcode::MultiplyImmediate => {
                registers[c] = registers[a] * b;
            }

            Opcode::BitwiseANDRegister => {
                registers[c] = registers[a] & registers[b];
            }

            Opcode::BitwiseANDImmediate => {
                registers[c] = registers[a] & b;
            }

            Opcode::BitwiseORRegister => {
                registers[c] = registers[a] | registers[b];
            }

            Opcode::BitwiseORImmediate => {
                registers[c] = registers[a] | b;
            }

            Opcode::SetRegister => {
                registers[c] = registers[a];
            }

            Opcode::SetImmediate => {
                registers[c] = a;
            }

            Opcode::GreaterThanImmediateRegister => {
                registers[c] = if a > registers[b] { 1 } else { 0 };
            }

            Opcode::GreaterThanRegisterImmediate => {
                registers[c] = if registers[a] > b { 1 } else { 0 };
            }

            Opcode::GreaterThanRegisterRegister => {
                registers[c] = if registers[a] > registers[b] { 1 } else { 0 };
            }

            Opcode::EqualImmediateRegister => {
                registers[c] = if a == registers[b] { 1 } else { 0 };
            }

            Opcode::EqualRegisterImmediate => {
                registers[c] = if registers[a] == b { 1 } else { 0 };
            }

            Opcode::EqualRegisterRegister => {
                registers[c] = if registers[a] == registers[b] { 1 } else { 0 };
            }
        }
    }
}

#[derive(Debug)]
struct Sample {
    instruction: [usize; 4],
    registers_before: [usize; 4],
    registers_after: [usize; 4],
}

impl Sample {
    fn possible_opcodes(&self) -> u16 {
        let mut registers = [0; 4];
        let mut result = 0u16;

        let [_, a, b, c] = self.instruction;

        for (i, opcode) in OPCODES.iter().enumerate() {
            let instruction = Instruction { opcode: *opcode, a, b, c };
            registers.copy_from_slice(&self.registers_before);
            instruction.run(&mut registers);

            if registers == self.registers_after {
                result |= 1 << i;
            }
        }

        result
    }
}

pub fn parse_samples(input: &str) -> impl Iterator<Item = Sample> + '_ {
    lazy_static! {
      static ref PATTERN: Regex = Regex::new(r"Before:\s+\[(\d+),\s+(\d+),\s+(\d+),\s+(\d+)\]\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)+\s+After:\s+\[(\d+),\s+(\d+),\s+(\d+),\s+(\d+)\]").unwrap();
    }

    PATTERN.captures_iter(input).filter_map(|caps| {
        Some(Sample {
            registers_before: [
                caps.get(1)?.as_str().parse().ok()?,
                caps.get(2)?.as_str().parse().ok()?,
                caps.get(3)?.as_str().parse().ok()?,
                caps.get(4)?.as_str().parse().ok()?,
            ],
            instruction: [
                caps.get(5)?.as_str().parse().ok()?,
                caps.get(6)?.as_str().parse().ok()?,
                caps.get(7)?.as_str().parse().ok()?,
                caps.get(8)?.as_str().parse().ok()?,
            ],
            registers_after: [
                caps.get(9)?.as_str().parse().ok()?,
                caps.get(10)?.as_str().parse().ok()?,
                caps.get(11)?.as_str().parse().ok()?,
                caps.get(12)?.as_str().parse().ok()?,
            ],
        })
    })
}

pub fn parse_instructions(input: &str) -> impl Iterator<Item = [usize; 4]> + '_ {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
    }

    let input = &input[input.rfind("After").unwrap()..];

    PATTERN.captures_iter(input).filter_map(|caps| {
        Some([
            caps.get(1)?.as_str().parse().ok()?,
            caps.get(2)?.as_str().parse().ok()?,
            caps.get(3)?.as_str().parse().ok()?,
            caps.get(4)?.as_str().parse().ok()?,
        ])
    })
}

#[allow(dead_code)]
fn create_opcode_set(opcodes: u16) -> HashSet<Opcode> {
    (0..16).filter(|i| (opcodes >> i) & 1 == 1).map(|i| OPCODES[i]).collect()
}

fn compute_opcode_map(input: &str) -> [Opcode; 16] {
    let mut possible_opcodes = [u16::max_value(); 16];

    for sample in parse_samples(input) {
        let opcode_number = sample.instruction[0];
        possible_opcodes[opcode_number] &= sample.possible_opcodes();
    }

    let mut ready = 0u16;

    // This will only work if there are enough samples, otherwise it'll loop forever
    while ready != u16::max_value() {
        for i in 0..16 {
            if possible_opcodes[i].count_ones() == 1 {
                ready |= 1 << i;
            }

            if ready >> i & 1 == 1 {
                for j in 0..16 {
                    if i != j {
                        possible_opcodes[j] &= !possible_opcodes[i];
                    }
                }
            }
        }
    }

    let mut opcode_map = [Opcode::AddImmediate; 16];

    for i in 0..16 {
        opcode_map[i] = OPCODES[possible_opcodes[i].trailing_zeros() as usize];
    }

    opcode_map
}

pub fn part1(input: &str) -> usize {
    parse_samples(input).filter(|sample| sample.possible_opcodes().count_ones() >= 3).count()
}

pub fn part2(input: &str) -> usize {
    let opcode_map = compute_opcode_map(input);
    let instructions = parse_instructions(input).map(|[opcode_number, a, b, c]| Instruction {
        opcode: opcode_map[opcode_number],
        a,
        b,
        c,
    });

    let mut registers = [0; 4];

    for instruction in instructions {
        instruction.run(&mut registers);
    }

    registers[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn possible_opcodes_work() {
        let sample = Sample {
            instruction: [9, 2, 1, 2],
            registers_before: [3, 2, 1, 1],
            registers_after: [3, 2, 2, 1],
        };

        let possible_opcodes = sample.possible_opcodes();

        let possible_opcodes = create_opcode_set(possible_opcodes);

        assert!(possible_opcodes.contains(&Opcode::MultiplyRegister));
        assert!(possible_opcodes.contains(&Opcode::AddImmediate));
        assert!(possible_opcodes.contains(&Opcode::SetImmediate));
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 542);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 575);
    }
}
