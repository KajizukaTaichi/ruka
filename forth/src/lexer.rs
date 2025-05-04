pub enum Token {
    DefineStart, // とは
    DefineEnd,   // こと
    IfThen,      // ならば
    IfElse,      // さもなければ
    IfEnd,       // つぎに
    Number(f64),
    Word(String),
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut result = Vec::new();
    for token in source.split_whitespace() {
        match token {
            "とは" => result.push(Token::DefineStart),
            "こと" => result.push(Token::DefineEnd),
            "ならば" => result.push(Token::IfThen),
            "さもなければ" => result.push(Token::IfElse),
            "つぎに" => result.push(Token::IfEnd),
            _ => {
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
