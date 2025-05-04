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
    let mut temp_then: Vec<Vec<Node>> = vec![vec![]];
    let mut temp_else: Vec<Vec<Node>> = vec![vec![]];
    let mut expr = Vec::new();

    for token in tokens {
        macro_rules! var {
            () => {
                match if_state {
                    IfState::Condition => &mut temp_body,
                    IfState::Then => temp_then.last_mut()?,
                    IfState::Else => temp_else.last_mut()?,
                }
            };
        }

        match (token.clone(), word_state.clone(), if_state.clone()) {
            (Token::Number(n), WordState::Body, _) => var!().push(Node::Value(n)),
            (Token::Word(name), WordState::Body, _) => var!().push(Node::Call(Word::parse(&name)?)),
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
            (Token::IfThen, WordState::Body, IfState::Condition) => {
                if_state = IfState::Then;
            }
            (Token::IfElse, WordState::Body, IfState::Then) => {
                if_state = IfState::Else;
            }
            (Token::IfEnd, WordState::Body, _) => {
                temp_body.push(Node::If(temp_then.pop()?, temp_else.pop()?));
                if_state = IfState::Condition;
                temp_then.clear();
                temp_else.clear();
            }
            _ => return None,
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
