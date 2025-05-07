mod r#gen;
mod lexer;
mod parse;

use lexer::tokenize;

use ruka_vm::{BasedMode, RukaVM, asm};

fn main() {
    println!("Hello, world!");
    run("(* 2 (- 10 5) ) (+ 1 2 3))").map(|x| println!(" = {x}"));
}

fn run(source: &str) -> Option<f64> {
    let ast = Expr::parse(source)?;
    let code = ast.compile()?;
    let code = code + "\thlt\n";

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
