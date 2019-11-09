use core::fmt;
use core::num::ParseIntError;
use core::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

struct ParseOpcodeError;

impl FromStr for Opcode {
    type Err = ParseOpcodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addr" => Ok(Opcode::AddRegister),
            "addi" => Ok(Opcode::AddImmediate),
            "mulr" => Ok(Opcode::MultiplyRegister),
            "muli" => Ok(Opcode::MultiplyImmediate),
            "banr" => Ok(Opcode::BitwiseANDRegister),
            "bani" => Ok(Opcode::BitwiseANDImmediate),
            "borr" => Ok(Opcode::BitwiseORRegister),
            "bori" => Ok(Opcode::BitwiseORImmediate),
            "setr" => Ok(Opcode::SetRegister),
            "seti" => Ok(Opcode::SetImmediate),
            "gtir" => Ok(Opcode::GreaterThanImmediateRegister),
            "gtri" => Ok(Opcode::GreaterThanRegisterImmediate),
            "gtrr" => Ok(Opcode::GreaterThanRegisterRegister),
            "eqir" => Ok(Opcode::EqualImmediateRegister),
            "eqri" => Ok(Opcode::EqualRegisterImmediate),
            "eqrr" => Ok(Opcode::EqualRegisterRegister),
            _ => Err(ParseOpcodeError),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Opcode::AddRegister => "addr",
                Opcode::AddImmediate => "addi",
                Opcode::MultiplyRegister => "mulr",
                Opcode::MultiplyImmediate => "muli",
                Opcode::BitwiseANDRegister => "banr",
                Opcode::BitwiseANDImmediate => "bani",
                Opcode::BitwiseORRegister => "borr",
                Opcode::BitwiseORImmediate => "bori",
                Opcode::SetRegister => "setr",
                Opcode::SetImmediate => "seti",
                Opcode::GreaterThanImmediateRegister => "gtir",
                Opcode::GreaterThanRegisterImmediate => "gtri",
                Opcode::GreaterThanRegisterRegister => "gtrr",
                Opcode::EqualImmediateRegister => "eqir",
                Opcode::EqualRegisterImmediate => "eqri",
                Opcode::EqualRegisterRegister => "eqrr",
            }
        )
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

enum ParseInstructionError {
    TooShort,
    Int(ParseIntError),
    Opcode(ParseOpcodeError),
}

impl From<ParseOpcodeError> for ParseInstructionError {
    fn from(error: ParseOpcodeError) -> ParseInstructionError {
        ParseInstructionError::Opcode(error)
    }
}

impl From<ParseIntError> for ParseInstructionError {
    fn from(error: ParseIntError) -> ParseInstructionError {
        ParseInstructionError::Int(error)
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.splitn(4, char::is_whitespace);
        Ok(Instruction {
            opcode: Opcode::from_str(it.next().ok_or(ParseInstructionError::TooShort)?)?,
            a: it.next().ok_or(ParseInstructionError::TooShort)?.parse()?,
            b: it.next().ok_or(ParseInstructionError::TooShort)?.parse()?,
            c: it.next().ok_or(ParseInstructionError::TooShort)?.parse()?,
        })
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.opcode, self.a, self.b, self.c)
    }
}

#[derive(Debug, Clone)]
struct CPU {
    registers: [usize; 6],
    executed_instructions: usize,
    instruction_pointer_index: usize,
    program: Vec<Instruction>,
}

impl CPU {
    fn from_input(input: &str) -> CPU {
        let mut lines = input.lines();
        let first_line = lines.next().unwrap();
        let instruction_pointer_index: usize = first_line[4..].parse().unwrap();
        let program: Vec<Instruction> = lines.filter_map(|s| s.parse().ok()).collect();

        CPU { instruction_pointer_index, program, executed_instructions: 0, registers: [0; 6] }
    }

    fn instruction_pointer(&self) -> usize {
        self.registers[self.instruction_pointer_index]
    }

    fn step(&mut self) {
        if let Some(instruction) = self.program.get(self.instruction_pointer()) {
            let Instruction { opcode, a, b, c } = *instruction;

            self.registers[c] = match opcode {
                Opcode::AddRegister => self.registers[a] + self.registers[b],
                Opcode::AddImmediate => self.registers[a] + b,
                Opcode::MultiplyRegister => self.registers[a] * self.registers[b],
                Opcode::MultiplyImmediate => self.registers[a] * b,
                Opcode::BitwiseANDRegister => self.registers[a] & self.registers[b],
                Opcode::BitwiseANDImmediate => self.registers[a] & b,
                Opcode::BitwiseORRegister => self.registers[a] | self.registers[b],
                Opcode::BitwiseORImmediate => self.registers[a] | b,
                Opcode::SetRegister => self.registers[a],
                Opcode::SetImmediate => a,
                Opcode::GreaterThanImmediateRegister => (a > self.registers[b]) as usize,
                Opcode::GreaterThanRegisterImmediate => (self.registers[a] > b) as usize,
                Opcode::GreaterThanRegisterRegister => {
                    (self.registers[a] > self.registers[b]) as usize
                }
                Opcode::EqualImmediateRegister => (a == self.registers[b]) as usize,
                Opcode::EqualRegisterImmediate => (self.registers[a] == b) as usize,
                Opcode::EqualRegisterRegister => (self.registers[a] == self.registers[b]) as usize,
            };

            self.executed_instructions += 1;
            self.registers[self.instruction_pointer_index] += 1;
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut cpu = CPU::from_input(input);

    while cpu.instruction_pointer() != 28 {
        cpu.step();
    }

    cpu.registers[3]
}

pub fn part2(input: &str) -> usize {
    let mut cpu = CPU::from_input(input);
    let mut seen = HashSet::new();
    let mut last_unique = None;

    loop {
        if cpu.instruction_pointer() == 28 {
            let value = cpu.registers[3];

            if seen.contains(&value) {
                break last_unique.unwrap();
            }

            last_unique = Some(value);
            seen.insert(value);
        }

        cpu.step();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 7_967_233);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 16_477_902);
    }
}
