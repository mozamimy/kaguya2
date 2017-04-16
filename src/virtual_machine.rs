#[derive(Debug)]
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

#[derive(Debug)]
pub struct Instruction {
    pub operation_type: InstructionType,
    pub operand: Option<i32>,
}

#[derive(Debug)]
pub struct VirtualMachine {
    iseq: Vec<Instruction>,
    pc: u32,
    left_stack: Vec<u8>,
    right_stack: Vec<u8>,
}

// impl VirtualMachine {
//     pub fn run() {
//         let 
//     }
// }
