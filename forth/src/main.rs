mod compile;
mod lexer;
mod parse;

use compile::Context;
use lexer::{Token, tokenize};
use parse::parse;
use ruka_vm::*;

fn main() {
    println!("こんな日本語プログラミング言語は好きですか？");
    let code = include_str!("../example.mind");
    println!("\nコード例\n```\n{code}```");
    let ast = parse(tokenize(code)).unwrap();

    let output = compile!(ast => &mut Context { label_index: 0 });
    let asm_code = format!("\tcal word_メイン\n\thlt\n{output}");
    println!("\nコンパイルされたアセンブリ\n```\n{asm_code}```");

    println!("\n仮想マシン(VM)のダンプ\n```");

    let bytecodes = asm(&asm_code).unwrap();
    let mut vm = RukaVM::new(bytecodes);
    vm.run().unwrap();
    vm.dump();

    println!("```");
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
