mod compile;
mod lexer;
mod parse;

use compile::Context;
use lexer::Token;
use parse::parse;

fn main() {
    println!("Hello, world!");
    println!(
        "{}",
        compile!(
            parse(lexer::tokenize(
                "メイン　とは　1　2 足 こと"
            ))
            .unwrap() => &mut Context { label_index: 0 }
        )
    );
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
