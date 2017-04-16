use compiler;
use virtual_machine;

#[derive(Debug)]
pub enum NodeType {
    Root,
    Forward,
    Backward,
    Increment,
    Decrement,
    Output,
    Input,
    While,
}

pub type NodeId = usize;

#[derive(Debug)]
pub struct Node {
    pub node_id: NodeId,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub ntype: NodeType,
}

#[derive(Debug)]
pub struct NodeArena {
    pub arena: Vec<Node>,
}

impl NodeArena {
    pub fn alloc(&mut self, ntype: NodeType, parent: Option<NodeId>) -> NodeId {
        let id = self.arena.len();
        let node = Node { node_id: id, parent: parent, children: Vec::new(), ntype: ntype };
        self.arena.push(node);
        id
    }

    pub fn get(&self, id: NodeId) -> &Node {
        &self.arena[id]
    }

    pub fn get_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.arena[id]
    }

    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        &self.get_mut(parent_id).children.push(child_id);
    }
}

impl Node {
    pub fn accept(&self, compiler: &compiler::Compiler) -> Vec<virtual_machine::Operation> {
        compiler.visit(self.node_id)
    }
}
