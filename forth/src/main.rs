mod compile;
mod lexer;
mod parse;

use compile::Context;
use lexer::Token;
use parse::parse;
use ruka_vm::*;

fn main() {
    println!("Hello, world!");
    let code = "メイン　とは　1　2 足 こと";

    let ast = parse(lexer::tokenize(code)).unwrap();
    let output = compile!(ast => &mut Context { label_index: 0 });
    let asm_code = format!("\tcal word_メイン\n\thlt\n{output}");
    println!("{asm_code}");

    let mut vm = RukaVM::new(asm(&asm_code).unwrap());
    vm.run().unwrap();
    vm.dump();
}

type Expr = Vec<Node>;
type Name = String;

#[derive(Clone, Debug)]
enum TopLevel {
    Define(Name, Expr),
}

#[derive(Clone, Debug)]
enum Node {
    If(Expr, Expr),
    Value(f64),
    Call(Word),
}

#[derive(Clone, Debug)]
enum Word {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    LessThan,
    GreaterThan,
    User(Name),
}
