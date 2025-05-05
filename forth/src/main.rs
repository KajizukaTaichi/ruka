mod compile;
mod keyword;
mod lexer;
mod parse;

use compile::Context;
use keyword::{Keyword, Language};
use lexer::{Token, tokenize};
use parse::parse;
use ruka_vm::*;
use std::fs::read_to_string;

fn main() {
    let langs = [Language::Normal, Language::Japanese, Language::Russian];
    run(&langs[0]).unwrap();
}

fn run(lang: &Language) -> Option<()> {
    let Ok(code) = read_to_string(format!(
        "./forth/example/{}.4th",
        match lang {
            Language::Normal => "normal",
            Language::Japanese => "japanese",
            Language::Russian => "russian",
        }
    )) else {
        return None;
    };

    let keywords = Keyword::new(&lang);
    let ast = parse(tokenize(&code, &keywords), &keywords)?;

    let output = compile!(ast => &mut Context { label_index: 0 });
    let asm_code = format!(
        "\tcal word_{}\n\thlt\n{output}",
        match lang {
            Language::Normal => "main",
            Language::Japanese => "始まり",
            Language::Russian => "главное",
        }
    );

    let bytecodes = asm(&asm_code)?;
    let mut vm = RukaVM::new(bytecodes);
    vm.run()?;

    Some(())
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
