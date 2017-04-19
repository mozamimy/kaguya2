extern crate kaguya2;

use std::env;
use std::fs::File;
use std::io::Read;

use kaguya2::parser;
use kaguya2::ast;
use kaguya2::compiler;
use kaguya2::virtual_machine;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let mut file = File::open(&filepath).expect("Couldn't open file");

    let mut script = String::new();
    file.read_to_string(&mut script).expect("Couldn't read file");
    let script = script;

    let parser = parser::Parser::new(script);

    let arena = &mut ast::NodeArena { arena: Vec::new() };
    let root_id = arena.alloc(ast::NodeType::Root, None);

    parser.parse(root_id, arena);

    let compiler = compiler::Compiler::new(root_id, arena);
    let iseq = compiler.compile();

    let virtual_machine = &mut virtual_machine::VirtualMachine::new(iseq);
    virtual_machine.run();
}
