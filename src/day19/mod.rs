use core::num::ParseIntError;
use core::str::FromStr;

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
    // Our custom instruction
    SumFactorsAndExitProgram,
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

#[derive(Debug)]
struct CPU {
    registers: [usize; 6],
    instruction_pointer_index: usize,
    program: Vec<Instruction>,
}

impl CPU {
    fn from_input(input: &str) -> CPU {
        let mut lines = input.lines();
        let first_line = lines.next().unwrap();
        let instruction_pointer_index: usize = first_line[4..].parse().unwrap();
        let program: Vec<Instruction> = lines.filter_map(|s| s.parse().ok()).collect();

        CPU { instruction_pointer_index, program, registers: [0; 6] }
    }

    fn run(&mut self) {
        let mut stats = vec![0u64; self.program.len()];

        while self.registers[self.instruction_pointer_index] < self.program.len() {
            let instruction_pointer = self.registers[self.instruction_pointer_index];
            let Instruction { opcode, a, b, c } = self.program[instruction_pointer];
            stats[instruction_pointer] += 1;

            match opcode {
                Opcode::AddRegister => {
                    self.registers[c] = self.registers[a] + self.registers[b];
                }

                Opcode::AddImmediate => {
                    self.registers[c] = self.registers[a] + b;
                }

                Opcode::MultiplyRegister => {
                    self.registers[c] = self.registers[a] * self.registers[b];
                }

                Opcode::MultiplyImmediate => {
                    self.registers[c] = self.registers[a] * b;
                }

                Opcode::BitwiseANDRegister => {
                    self.registers[c] = self.registers[a] & self.registers[b];
                }

                Opcode::BitwiseANDImmediate => {
                    self.registers[c] = self.registers[a] & b;
                }

                Opcode::BitwiseORRegister => {
                    self.registers[c] = self.registers[a] | self.registers[b];
                }

                Opcode::BitwiseORImmediate => {
                    self.registers[c] = self.registers[a] | b;
                }

                Opcode::SetRegister => {
                    self.registers[c] = self.registers[a];
                }

                Opcode::SetImmediate => {
                    self.registers[c] = a;
                }

                Opcode::GreaterThanImmediateRegister => {
                    self.registers[c] = if a > self.registers[b] { 1 } else { 0 };
                }

                Opcode::GreaterThanRegisterImmediate => {
                    self.registers[c] = if self.registers[a] > b { 1 } else { 0 };
                }

                Opcode::GreaterThanRegisterRegister => {
                    self.registers[c] = if self.registers[a] > self.registers[b] { 1 } else { 0 };
                }

                Opcode::EqualImmediateRegister => {
                    self.registers[c] = if a == self.registers[b] { 1 } else { 0 };
                }

                Opcode::EqualRegisterImmediate => {
                    self.registers[c] = if self.registers[a] == b { 1 } else { 0 };
                }

                Opcode::EqualRegisterRegister => {
                    self.registers[c] = if self.registers[a] == self.registers[b] { 1 } else { 0 };
                }

                Opcode::SumFactorsAndExitProgram => {
                    // Not the most efficient, but is O(n) instead of O(n^2)
                    self.registers[c] =
                        (1..=self.registers[a]).filter(|x| self.registers[a] % x == 0).sum();
                    break;
                }
            }

            self.registers[self.instruction_pointer_index] += 1;
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut cpu = CPU::from_input(input);

    cpu.run();
    cpu.registers[0]
}

pub fn part2(input: &str) -> usize {
    let mut cpu = CPU::from_input(input);

    cpu.registers[0] = 1;

    // Haxx
    cpu.program[1] = Instruction { opcode: Opcode::SumFactorsAndExitProgram, a: 3, b: 0, c: 0 };

    cpu.run();
    cpu.registers[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input");
    const INPUT: &str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 7);
        assert_eq!(part1(INPUT), 3224);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 32_188_416);
    }
}
