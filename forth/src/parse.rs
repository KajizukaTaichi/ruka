use crate::*;

pub fn parse(tokens: Vec<Token>) -> Option<Vec<TopLevel>> {
    #[derive(Clone, Debug)]
    enum WordState {
        Name,
        Body,
    }
    #[derive(Clone, Debug)]
    enum CtrlState {
        Condition,
        Then,
        Else,
        While,
    }

    let mut word_state = WordState::Name;
    let mut ctrl_state = CtrlState::Condition;

    let mut temp_name = None;
    let mut temp_body: Vec<Node> = vec![];
    let mut temp_then: Vec<Node> = vec![];
    let mut temp_else: Vec<Node> = vec![];
    let mut temp_loop: Vec<Node> = vec![];
    let mut expr = Vec::new();

    for token in tokens {
        match (token.clone(), word_state.clone(), ctrl_state.clone()) {
            (Token::Word(name), WordState::Name, CtrlState::Condition) => {
                temp_name = Some(name.to_owned())
            }
            (Token::DefineStart, WordState::Name, CtrlState::Condition) => {
                word_state = WordState::Body;
            }
            (Token::DefineEnd, WordState::Body, CtrlState::Condition) => {
                expr.push(TopLevel::Define(temp_name.clone()?, temp_body.clone()));
                word_state = WordState::Name;
                temp_name = None;
                temp_body.clear()
            }
            (Token::Number(n), WordState::Body, CtrlState::Condition) => {
                temp_body.push(Node::Value(n))
            }
            (Token::Word(name), WordState::Body, CtrlState::Condition) => {
                temp_body.push(Node::Call(Word::parse(&name)?))
            }
            (Token::IfThen, WordState::Body, CtrlState::Condition) => {
                ctrl_state = CtrlState::Then;
            }
            (Token::WhileStart, WordState::Body, CtrlState::Condition) => {
                ctrl_state = CtrlState::While;
            }
            (Token::IfElse, WordState::Body, CtrlState::Then) => {
                ctrl_state = CtrlState::Else;
            }
            (Token::Number(n), WordState::Body, CtrlState::Then) => {
                temp_then.push(Node::Value(n));
            }
            (Token::Word(name), WordState::Body, CtrlState::Then) => {
                temp_then.push(Node::Call(Word::parse(&name)?))
            }
            (Token::Number(n), WordState::Body, CtrlState::Else) => {
                temp_else.push(Node::Value(n));
            }
            (Token::Word(name), WordState::Body, CtrlState::Else) => {
                temp_else.push(Node::Call(Word::parse(&name)?))
            }
            (Token::Number(n), WordState::Body, CtrlState::While) => {
                temp_loop.push(Node::Value(n));
            }
            (Token::Word(name), WordState::Body, CtrlState::While) => {
                temp_loop.push(Node::Call(Word::parse(&name)?))
            }
            (Token::IfEnd, WordState::Body, _) => {
                temp_body.push(Node::If(temp_then.clone(), temp_else.clone()));
                ctrl_state = CtrlState::Condition;
                temp_then.clear();
                temp_else.clear();
            }
            (Token::WhileEnd, WordState::Body, _) => {
                temp_body.push(Node::While(temp_loop.clone()));
                ctrl_state = CtrlState::Condition;
                temp_loop.clear();
            }
            _ => {
                dbg!((token.clone(), &word_state, &ctrl_state));
                return None;
            }
        }
    }

    Some(expr)
}

impl Word {
    fn parse(source: &str) -> Option<Word> {
        Some(match source {
            "足" => Word::Add,
            "引" => Word::Sub,
            "掛" => Word::Mul,
            "割" => Word::Div,
            "等" => Word::Equal,
            "小" => Word::LessThan,
            "大" => Word::GreaterThan,
            _ => Word::User(source.to_string()),
        })
    }
}
