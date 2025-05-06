mod compile;
mod keyword;
mod lexer;
mod parse;

use compile::Context;
use keyword::{Keyword, Language};
use lexer::{Token, tokenize};
use parse::parse;
use ruka_vm::*;
use std::env::args;
use std::fs::{File, read_to_string};
use std::io::Write;

fn main() {
    let args = args().collect::<Vec<_>>();
    let lang = args.get(1).cloned().unwrap_or("4th".to_owned());
    let lang = match lang.trim() {
        "ja" => Language::Japanese,
        "ru" => Language::Russian,
        "4th" => Language::Normal,
        _ => panic!(),
    };
    run(&lang).unwrap();
}

fn run(lang: &Language) -> Option<()> {
    let path = format!("./forth/example/{lang:?}.4th").to_lowercase();
    let Ok(code) = read_to_string(path) else {
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

    File::create("./forth/output.asm")
        .unwrap()
        .write_all(asm_code.as_bytes())
        .unwrap();

    let bytecodes = asm(&asm_code)?;
    let mut vm = RukaVM::new(bytecodes);
    vm.start()?;

    let ret = vm.returns(BasedMode::Stack);
    ret.map(|x| println!("{x}"));
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
