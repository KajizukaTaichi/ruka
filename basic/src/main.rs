mod block;
mod expr;
mod lexer;
mod oper;
mod stmt;
mod util;

use indexmap::IndexMap;
use ruka_vm::{BasedMode, RukaVM, asm};
use std::{fs::File, io::Write};
use util::{OPERATOR, SPACE, include_letter};
use {block::Block, expr::Expr, lexer::tokenize, oper::Oper, stmt::Stmt};

fn main() {
    let mut compiler = Compiler::new();
    let code = include_str!("../example.bas").trim();
    compiler.run(code).map(|x| println!(" = {x}"));
}

struct Compiler {
    if_label_index: usize,
    while_label_index: usize,
    variables: IndexMap<String, usize>,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            if_label_index: 0,
            while_label_index: 0,
            variables: IndexMap::new(),
        }
    }

    fn run(&mut self, source: &str) -> Option<f64> {
        let assembly = &self.build(source)?;
        File::create("./basic/output.asm")
            .unwrap()
            .write_all(assembly.as_bytes())
            .unwrap();

        let bytecodes = asm(assembly)?;
        let mut vm = RukaVM::new(bytecodes);

        vm.start()?;
        Some(vm.returns(BasedMode::Register)?)
    }

    fn build(&mut self, source: &str) -> Option<String> {
        let source = source.trim().to_lowercase();
        let ast = Block::parse(source)?;
        Some(ast.compile(self)?)
    }
}
