mod expr;
mod lexer;
mod oper;
mod stmt;
mod util;

use indexmap::IndexMap;
use ruka_vm::{BasedMode, RukaVM, asm};
use std::{fs::File, io::Write};
use util::{OPERATOR, SPACE, include_letter};
use {expr::Expr, lexer::tokenize, oper::Oper, stmt::Stmt};

fn main() {
    let mut compiler = Compiler {
        if_label_index: 0,
        while_label_index: 0,
        variables: IndexMap::new(),
    };
    let code = include_str!("../example.bas").trim();
    compiler.run(code).map(|x| println!(" = {x}"));
}

struct Compiler {
    if_label_index: usize,
    while_label_index: usize,
    variables: IndexMap<String, usize>,
}

impl Compiler {
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
        let mut result = String::new();
        let source = source.trim().to_lowercase();

        for (line, code) in source.lines().enumerate() {
            let (line, code) = code
                .trim()
                .split_once(" ")
                .map(|(line, code)| Some((ok!(line.parse::<usize>())?, code)))
                .flatten()
                .unwrap_or((line, code));

            if code.is_empty() || code.trim().starts_with("rem") {
                continue;
            }
            let stmt = Stmt::parse(code)?.compile(self)?;
            result.push_str(&format!("line_{line}:\n{stmt}\n"));
        }
        Some(result)
    }
}
