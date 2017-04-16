use ast;
use virtual_machine;

#[derive(Debug)]
pub struct Compiler<'a> {
    ast_root_id: ast::NodeId,
    ast_arena: &'a mut ast::NodeArena,
}

impl<'a> Compiler<'a> {
    pub fn new(ast_root_id: ast::NodeId, ast_arena: &mut ast::NodeArena) -> Compiler {
        Compiler { ast_root_id: ast_root_id, ast_arena: ast_arena }
    }

    pub fn compile(&self) -> Vec<(virtual_machine::Instruction)> {
        let root = self.ast_arena.get(self.ast_root_id);
        let mut iseq = root.accept(self);
        iseq.push(virtual_machine::Instruction {
            operation_type: virtual_machine::InstructionType::Leave,
            operand: None,
        });
        iseq
    }

    pub fn visit(&self, node_id: usize) -> Vec<virtual_machine::Instruction> {
        let mut iseq = Vec::new();
        let node = self.ast_arena.get(node_id);

        match node.ntype {
            ast::NodeType::Forward => {
                iseq.push(virtual_machine::Instruction {
                    operation_type: virtual_machine::InstructionType::Forward,
                    operand: None,
                })
            },
            ast::NodeType::Backward => {
                iseq.push(virtual_machine::Instruction {
                    operation_type: virtual_machine::InstructionType::Backward,
                    operand: None,
                })
            },
            ast::NodeType::Increment => {
                iseq.push(virtual_machine::Instruction {
                    operation_type: virtual_machine::InstructionType::Increment,
                    operand: None,
                })
            },
            ast::NodeType::Decrement => {
                iseq.push(virtual_machine::Instruction {
                    operation_type: virtual_machine::InstructionType::Decrement,
                    operand: None,
                })
            },
            ast::NodeType::Output => {
                iseq.push(virtual_machine::Instruction {
                    operation_type: virtual_machine::InstructionType::Output,
                    operand: None,
                })
            },
            ast::NodeType::While => {
                let mut sub_iseq = Vec::new();
                let children = node.children.clone();

                for i in children {
                    let child = self.ast_arena.get(i);
                    sub_iseq.append(&mut child.accept(self));
                }

                let sub_iseq_length = sub_iseq.len() as i32;

                iseq.push(virtual_machine::Instruction {
                    operation_type: virtual_machine::InstructionType::BranchIfZero,
                    operand: Some(sub_iseq_length + 2),
                });
                iseq.append(&mut sub_iseq);
                iseq.push(virtual_machine::Instruction {
                    operation_type: virtual_machine::InstructionType::BranchUnlessZero,
                    operand: Some(-sub_iseq_length),
                });
            },
            ast::NodeType::Root => {
                let children = node.children.clone();

                for i in children {
                    let child = self.ast_arena.get(i);
                    iseq.append(&mut child.accept(self));
                }
            },
            _ => panic!("Invalid node!")
        }

        iseq
    }
}
