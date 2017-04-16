use ast;

#[derive(Debug)]
pub struct Parser {
    pub input: String,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser { input: input }
    }

    pub fn parse(&self, root_id: ast::NodeId, arena: &mut ast::NodeArena) {
        let mut current_node_id = Some(root_id);

        for chr in self.input.chars() {
            match chr {
                '>' => {
                    let new_node_id = arena.alloc(ast::NodeType::Forward, current_node_id);
                    arena.append_child(current_node_id.unwrap(), new_node_id);
                },
                '<' => {
                    let new_node_id = arena.alloc(ast::NodeType::Backward, current_node_id);
                    arena.append_child(current_node_id.unwrap(), new_node_id);
                },
                '+' => {
                    let new_node_id = arena.alloc(ast::NodeType::Increment, current_node_id);
                    arena.append_child(current_node_id.unwrap(), new_node_id);
                },
                '-' => {
                    let new_node_id = arena.alloc(ast::NodeType::Decrement, current_node_id);
                    arena.append_child(current_node_id.unwrap(), new_node_id);
                },
                '.' => {
                    let new_node_id = arena.alloc(ast::NodeType::Output, current_node_id);
                    arena.append_child(current_node_id.unwrap(), new_node_id);
                },
                ',' => {
                    let new_node_id = arena.alloc(ast::NodeType::Input, current_node_id);
                    arena.append_child(current_node_id.unwrap(), new_node_id);
                },
                '[' => {
                    let new_node_id = arena.alloc(ast::NodeType::While, current_node_id);
                    arena.append_child(current_node_id.unwrap(), new_node_id);
                    current_node_id = Some(new_node_id);
                },
                ']' => {
                    current_node_id = arena.get(current_node_id.unwrap()).parent;
                },
                ' ' | '\n' | '\r' => {
                    // read next character
                },
                _ => panic!("Invalid character: {}", chr),
            }
        }
    }
}
