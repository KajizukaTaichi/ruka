use crate::*;

pub fn parse(tokens: Vec<Token>, keyword: &Keyword) -> Option<Vec<Define>> {
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
        macro_rules! var {
            () => {
                match if_state {
                    IfState::Condition => &mut temp_body,
                    IfState::Then => &mut temp_then,
                    IfState::Else => &mut temp_else,
                }
            };
        }

        match (token.clone(), word_state.clone(), if_state.clone()) {
            (Token::Number(n), WordState::Body, _) => var!().push(Node::Value(n)),
            (Token::Word(name), WordState::Body, _) => {
                var!().push(Node::Call(Word::parse(&name, keyword)?))
            }
            (Token::Word(name), WordState::Name, IfState::Condition) => {
                temp_name = Some(name.to_owned())
            }
            (Token::DefineStart, WordState::Name, IfState::Condition) => {
                word_state = WordState::Body;
            }
            (Token::DefineEnd, WordState::Body, IfState::Condition) => {
                expr.push(Define(temp_name.clone()?, temp_body.clone()));
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
                temp_body.push(Node::If(temp_then.clone(), temp_else.clone()));
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
    fn parse(source: &str, keyword: &Keyword) -> Option<Word> {
        Some(match source {
            _ if source == keyword.add => Word::Add,
            _ if source == keyword.sub => Word::Sub,
            _ if source == keyword.mul => Word::Mul,
            _ if source == keyword.div => Word::Div,
            _ if source == keyword.equal => Word::Equal,
            _ if source == keyword.less_than => Word::LessThan,
            _ if source == keyword.greater_than => Word::GreaterThan,
            _ if source == keyword.load => Word::Load,
            _ if source == keyword.store => Word::Store,
            _ => Word::User(source.to_string()),
        })
    }
}
