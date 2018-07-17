use std::io::*;
use std::process;

#[derive(Debug, Clone)]
pub enum InstructionType {
    Forward,
    Backward,
    Increment,
    Decrement,
    Output,
    Input,
    BranchIfZero,
    BranchUnlessZero,
    Leave,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub operand: Option<i32>,
}

#[derive(Debug)]
pub struct VirtualMachine {
    iseq: Vec<Instruction>,
    pc: u32,
    left_stack: Vec<u8>,
    right_stack: Vec<u8>,
}

impl VirtualMachine {
    pub fn new(iseq: Vec<Instruction>) -> VirtualMachine {
        VirtualMachine {
            iseq: iseq,
            pc: 0,
            left_stack: vec![0],
            right_stack: vec![],
        }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch(self.pc);
            self.execute(instruction);
        }
    }

    fn fetch(&self, pc: u32) -> Instruction {
        self.iseq[pc as usize].clone()
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.instruction_type {
            InstructionType::Forward => {
                if self.right_stack.len() < 1 {
                    self.left_stack.push(0);
                } else {
                    self.left_stack.push(self.right_stack.pop().unwrap());
                }
                self.pc += 1;
            }
            InstructionType::Backward => {
                self.right_stack.push(self.left_stack.pop().unwrap());
                self.pc += 1;
            }
            InstructionType::Increment => {
                let new_value = self.left_stack.pop().unwrap() + 1;
                self.left_stack.push(new_value);
                self.pc += 1;
            }
            InstructionType::Decrement => {
                let new_value = self.left_stack.pop().unwrap() - 1;
                self.left_stack.push(new_value);
                self.pc += 1;
            }
            InstructionType::Output => {
                let value = self.left_stack.pop();
                print!("{}", value.unwrap() as char);
                self.left_stack.push(value.unwrap());
                self.pc += 1;
            }
            InstructionType::Input => {
                self.left_stack.pop();
                let mut buf = [0u8; 1];
                ::std::io::stdin().read_exact(&mut buf).unwrap();
                self.left_stack.push(buf[0]);
                self.pc += 1;
            }
            InstructionType::BranchIfZero => {
                let value = self.left_stack.pop().unwrap();
                self.left_stack.push(value);

                if value == 0 {
                    self.pc = (self.pc as i32 + instruction.operand.unwrap()) as u32;
                } else {
                    self.pc += 1;
                }
            }
            InstructionType::BranchUnlessZero => {
                let value = self.left_stack.pop().unwrap();
                self.left_stack.push(value);

                if value != 0 {
                    self.pc = (self.pc as i32 + instruction.operand.unwrap()) as u32;
                } else {
                    self.pc += 1;
                }
            }
            InstructionType::Leave => {
                process::exit(0);
            }
        }
    }
}
