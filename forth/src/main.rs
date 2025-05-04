mod lexer;
use lexer::Token;

fn main() {
    println!("Hello, world!");
}

type Expr = Vec<Node>;
type Name = String;

#[derive(Clone)]
enum TopLevel {
    If(Expr, Expr),
    Define(Name, Expr),
}

#[derive(Clone)]
enum Node {
    Value(f64),
    Call(Name),
}

fn parse(tokens: Vec<Token>) -> Option<Vec<TopLevel>> {
    #[derive(Clone)]
    enum WordState {
        Name,
        Body,
    }
    #[derive(Clone)]
    enum IfState {
        Condition,
        Then,
        Else,
    }

    let mut word_state = WordState::Name;
    let mut if_state = IfState::Condition;

    let mut temp_name = None;
    let mut temp_body: Vec<Node> = vec![];
    let mut temp_then: Vec<Node> = vec![];
    let mut temp_else: Vec<Node> = vec![];
    let mut expr = Vec::new();

    for token in tokens {
        match (token, word_state.clone(), if_state.clone()) {
            (Token::Word(name), WordState::Name, IfState::Condition) => {
                temp_name = Some(name.to_owned())
            }
            (Token::DefineStart, WordState::Name, IfState::Condition) => {
                word_state = WordState::Body;
            }
            (Token::DefineEnd, WordState::Body, IfState::Condition) => {
                expr.push(TopLevel::Define(temp_name.clone()?, temp_body.clone()));
                word_state = WordState::Name;
            }
            (Token::Number(n), WordState::Body, IfState::Condition) => {
                temp_body.push(Node::Value(n))
            }
            (Token::Word(name), WordState::Body, IfState::Condition) => {
                temp_body.push(Node::Call(name.to_owned()))
            }
            (Token::IfThen, WordState::Body, IfState::Condition) => {
                if_state = IfState::Else;
            }
            (Token::Number(n), WordState::Body, IfState::Then) => {
                temp_then.push(Node::Value(n));
            }
            (Token::Word(name), WordState::Body, IfState::Then) => {
                temp_then.push(Node::Call(name.to_owned()))
            }
            (Token::IfElse, WordState::Body, IfState::Then) => {
                if_state = IfState::Else;
            }
            (Token::Number(n), WordState::Body, IfState::Else) => {
                temp_else.push(Node::Value(n));
            }
            (Token::Word(name), WordState::Body, IfState::Else) => {
                temp_else.push(Node::Call(name.to_owned()))
            }
            (Token::IfEnd, WordState::Body, _) => {
                expr.push(TopLevel::If(temp_then.clone(), temp_else.clone()));
                if_state = IfState::Condition;
            }
            _ => return None,
        }
    }

    Some(expr)
}
