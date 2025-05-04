mod lexer;
mod parse;

use lexer::Token;
use parse::parse;

fn main() {
    println!("Hello, world!");
    dbg!(parse(lexer::tokenize(
        "A とは 素数? ならば 1 2 足 さもなければ SEX! つぎに 表示する こと"
    )));
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
