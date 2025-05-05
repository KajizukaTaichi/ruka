use crate::*;

#[derive(Clone, Debug)]
pub enum Token {
    DefineStart, // とは
    DefineEnd,   // こと
    IfThen,      // ならば
    IfElse,      // でなければ
    IfEnd,       // つぎに
    Number(f64),
    Word(String),
}

pub fn tokenize(source: &str, keyword: &Keyword) -> Vec<Token> {
    let mut result = Vec::new();
    for token in source.split_whitespace() {
        match token {
            _ if token == keyword.define_start => result.push(Token::DefineStart),
            _ if token == keyword.define_end => result.push(Token::DefineEnd),
            _ if token == keyword.if_then => result.push(Token::IfThen),
            _ if token == keyword.if_else => result.push(Token::IfElse),
            _ if token == keyword.if_end => result.push(Token::IfEnd),
            _ => {
                let token = trim_japanese(token);
                if let Ok(num) = token.parse::<f64>() {
                    result.push(Token::Number(num));
                } else {
                    result.push(Token::Word(token.to_string()));
                }
            }
        }
    }
    result
}

pub fn trim_japanese(mut source: &str) -> String {
    let suffix: Vec<char> = (0x3041..=0x3096)
        .chain(0x309D..=0x309F)
        .filter_map(std::char::from_u32)
        .collect();
    loop {
        let mut is_trimmed = false;
        for i in suffix.clone() {
            if let Some(stripped) = source.strip_suffix(i) {
                is_trimmed = true;
                source = stripped;
            }
        }
        if !is_trimmed {
            break source.to_string();
        }
    }
}
