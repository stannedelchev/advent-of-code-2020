use crate::common::Problem;
use itertools::Itertools;

pub struct Day08 {
    instructions: Vec<Instruction>,
}

#[derive(Clone)]
enum InstructionOperation {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

#[derive(Clone)]
struct Instruction {
    value: InstructionOperation,
    is_executed: bool,
}

impl Instruction {
    fn invert_value(&mut self) {
        self.value = match self.value {
            InstructionOperation::Nop(arg) => InstructionOperation::Jmp(arg),
            InstructionOperation::Jmp(arg) => InstructionOperation::Nop(arg),
            InstructionOperation::Acc(arg) => InstructionOperation::Acc(arg),
        }
    }
}

impl From<&str> for InstructionOperation {
    fn from(s: &str) -> Self {
        let operation = &s[0..3];
        let argument = &s[4..];
        match operation {
            "nop" => InstructionOperation::Nop(argument.parse::<isize>().unwrap()),
            "acc" => InstructionOperation::Acc(argument.parse::<isize>().unwrap()),
            "jmp" => InstructionOperation::Jmp(argument.parse::<isize>().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl Day08 {
    pub fn new(input: String) -> Self {
        let instructions = input
            .lines()
            .map(|l| Instruction {
                value: l.into(),
                is_executed: false,
            })
            .collect();

        Day08 { instructions }
    }
}

struct Cpu {
    instructions: Vec<Instruction>,
    state: CpuState,
}

impl Cpu {
    fn new(mut instructions: Vec<Instruction>) -> Self {
        for i in instructions.iter_mut() {
            i.is_executed = false;
        }

        Cpu {
            instructions,
            state: CpuState::new(),
        }
    }
}

#[derive(Copy, Clone)]
struct CpuState {
    ip: usize,
    accumulator: isize,
}

impl CpuState {
    fn new() -> Self {
        CpuState {
            ip: 0,
            accumulator: 0,
        }
    }

    fn with_next_ip(&self) -> Self {
        CpuState {
            ip: self.ip + 1,
            ..*self
        }
    }

    fn with_ip_offset(&self, offset: isize) -> Self {
        CpuState {
            ip: match offset {
                offset if offset < 0 => self.ip - (offset * -1) as usize,
                offset if offset > 0 => self.ip + offset as usize,
                _ => self.ip,
            },
            ..*self
        }
    }
}

enum ExecutionResult {
    Ok,
    Loop,
    Terminated,
}

impl Cpu {
    fn fetch(&self) -> &Instruction {
        &self.instructions[self.state.ip]
    }

    fn execute(&self, instruction: &Instruction) -> (ExecutionResult, CpuState) {
        if instruction.is_executed {
            return (ExecutionResult::Loop, self.state);
        }

        let new_state = match instruction.value {
            InstructionOperation::Nop(_) => self.state.with_next_ip(),
            InstructionOperation::Acc(argument) => CpuState {
                ip: self.state.ip + 1,
                accumulator: self.state.accumulator + argument,
            },
            InstructionOperation::Jmp(argument) => self.state.with_ip_offset(argument),
        };

        if new_state.ip >= self.instructions.len() {
            return (ExecutionResult::Terminated, new_state);
        }

        (ExecutionResult::Ok, new_state)
    }

    fn run(&mut self) -> ExecutionResult {
        loop {
            let ip = self.state.ip;
            let instruction = self.fetch();
            let (result, state) = self.execute(instruction);
            self.state = state;

            let mut instruct = self.instructions.get_mut(ip).unwrap();
            instruct.is_executed = true;

            match result {
                ExecutionResult::Loop => return result,
                ExecutionResult::Terminated => return result,
                _ => {}
            }
        }
    }
}

impl Problem for Day08 {
    fn part1(&self) -> String {
        let mut cpu = Cpu::new(self.instructions.clone());
        loop {
            let exec_result = cpu.run();

            if let ExecutionResult::Loop = exec_result {
                return cpu.state.accumulator.to_string();
            }
        }
    }

    fn part2(&self) -> String {
        let instructions = self.instructions.clone();

        let jmp_nop_indexes = instructions
            .iter()
            .enumerate()
            .filter(|(_, instruction)| match instruction.value {
                InstructionOperation::Acc(_) | InstructionOperation::Jmp(_) => true,
                _ => false,
            })
            .map(|(idx, _)| idx)
            .collect_vec();

        for idx in jmp_nop_indexes {
            let mut replaced_instructions = instructions.clone();

            replaced_instructions[idx].invert_value();

            let mut cpu = Cpu::new(replaced_instructions);
            match cpu.run() {
                ExecutionResult::Terminated => return cpu.state.accumulator.to_string(),
                _ => {}
            }
        }

        unreachable!()
    }
}
