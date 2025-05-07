mod r#gen;
mod lexer;
mod parse;

use indexmap::IndexMap;
use lexer::tokenize;
use ruka_vm::{BasedMode, RukaVM, asm};
use std::{fs::File, io::Write};

fn main() {
    println!("Hello, world!");
    run(include_str!("../example.scm")).map(|x| println!(" = {x}"));
}

fn run(source: &str) -> Option<f64> {
    let env = &mut IndexMap::new();
    let code = tokenize(source)?
        .iter()
        .map(|code| Expr::parse(&code).and_then(|ast| ast.compile(env)))
        .collect::<Option<Vec<_>>>()?;
    let code = format!("{}\thlt\n", code.concat());

    File::create("./lisp/output.asm")
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap();

    let mut vm = RukaVM::new(asm(&code)?);
    vm.start()?;

    Some(vm.returns(BasedMode::Register)?)
}

enum Expr {
    List(Vec<Expr>),
    Symbol(String),
    Value(f64),
}
