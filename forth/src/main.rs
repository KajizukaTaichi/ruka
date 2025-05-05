mod compile;
mod keyword;
mod lexer;
mod parse;

use std::fs::read_to_string;

use compile::Context;
use keyword::{Keyword, Language};
use lexer::{Token, tokenize};
use parse::parse;
use ruka_vm::*;

fn main() {
    run(&Language::Machine);
    run(&Language::Japanese);
    run(&Language::Russian);
}

fn run(lang: &Language) {
    let code = read_to_string(format!(
        "example/{}.mind",
        match lang {
            Language::Japanese => "japanese",
            Language::Machine => "machine",
            Language::Russian => "russia",
        }
    ))
    .unwrap();
    println!("{code}");

    let keywords = Keyword::new(&lang);
    let ast = parse(tokenize(&code, &keywords), &keywords).unwrap();

    let output = compile!(ast => &mut Context { label_index: 0 });
    let asm_code = format!(
        "\tcal word_{}\n\thlt\n{output}",
        match lang {
            Language::Japanese => "始まり",
            Language::Machine => "main",
            Language::Russian => "главное",
        }
    );
    println!("{asm_code}");

    let bytecodes = asm(&asm_code).unwrap();
    let mut vm = RukaVM::new(bytecodes);
    vm.run().unwrap();
    vm.dump();
}

type Expr = Vec<Node>;
type Name = String;

#[derive(Clone, Debug)]
struct Define(Name, Expr);

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
    Load,
    Store,
    GreaterThan,
    User(Name),
}
