mod r#gen;
mod lexer;
mod parse;

use indexmap::IndexMap;
use lexer::tokenize;
use ruka_vm::{BasedMode, RukaVM, asm};

fn main() {
    println!("Hello, world!");
    run("(var x 10) (+ x 2)").map(|x| println!(" = {x}"));
}

fn run(source: &str) -> Option<f64> {
    let env = &mut IndexMap::new();
    let code = tokenize(source)?
        .iter()
        .map(|code| Expr::parse(&code).and_then(|ast| ast.compile(env)))
        .collect::<Option<Vec<_>>>()?;
    let code = format!("{}\thlt\n", code.concat());

    println!("{code}");

    let mut vm = RukaVM::new(asm(&code)?);
    vm.start()?;

    Some(vm.returns(BasedMode::Register)?)
}

enum Expr {
    List(Vec<Expr>),
    Symbol(String),
    Value(f64),
}
