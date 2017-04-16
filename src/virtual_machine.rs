#[derive(Debug)]
pub enum OperationType {
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
pub struct Operation {
    pub operation_type: OperationType,
    pub operand: Option<i32>,
}
