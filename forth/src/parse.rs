use crate::*;

pub fn parse(tokens: Vec<Token>) -> Option<Vec<TopLevel>> {
    #[derive(Clone, Debug)]
    enum WordState {
        Name,
        Body,
    }
    #[derive(Clone, Debug)]
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
        match (token.clone(), word_state.clone(), if_state.clone()) {
            (Token::Word(name), WordState::Name, IfState::Condition) => {
                temp_name = Some(name.to_owned())
            }
            (Token::DefineStart, WordState::Name, IfState::Condition) => {
                word_state = WordState::Body;
            }
            (Token::DefineEnd, WordState::Body, IfState::Condition) => {
                expr.push(TopLevel::Define(temp_name.clone()?, temp_body.clone()));
                word_state = WordState::Name;
                temp_name = None;
                temp_body.clear()
            }
            (Token::Number(n), WordState::Body, IfState::Condition) => {
                temp_body.push(Node::Value(n))
            }
            (Token::Word(name), WordState::Body, IfState::Condition) => {
                temp_body.push(Node::Call(Word::parse(&name)))
            }
            (Token::IfThen, WordState::Body, IfState::Condition) => {
                if_state = IfState::Then;
            }
            (Token::Number(n), WordState::Body, IfState::Then) => {
                temp_then.push(Node::Value(n));
            }
            (Token::Word(name), WordState::Body, IfState::Then) => {
                temp_then.push(Node::Call(Word::parse(&name)))
            }
            (Token::IfElse, WordState::Body, IfState::Then) => {
                if_state = IfState::Else;
            }
            (Token::Number(n), WordState::Body, IfState::Else) => {
                temp_else.push(Node::Value(n));
            }
            (Token::Word(name), WordState::Body, IfState::Else) => {
                temp_else.push(Node::Call(Word::parse(&name)))
            }
            (Token::IfEnd, WordState::Body, _) => {
                temp_body.push(Node::If(temp_then.clone(), temp_else.clone()));
                if_state = IfState::Condition;
                temp_then.clear();
                temp_else.clear();
            }
            _ => {
                dbg!((token.clone(), &word_state, &if_state));
                return None;
            }
        }
    }

    Some(expr)
}

impl Word {
    fn parse(source: &str) -> Word {
        match source {
            "足" => Word::Add,
            "引" => Word::Sub,
            "掛" => Word::Mul,
            "割" => Word::Div,
            "等" => Word::Equal,
            "小" => Word::LessThan,
            "大" => Word::GreaterThan,
            _ => Word::User(source.to_string()),
        }
    }
}
