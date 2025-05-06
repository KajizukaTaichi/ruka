use ruka_vm::{RukaVM, asm};

fn main() {
    println!("Hello, world!");
    run().unwrap();
}

fn run() -> Option<()> {
    let ast = Expr::List(vec![
        Expr::Symbol(String::from("*")),
        Expr::Value(10.0),
        Expr::List(vec![
            Expr::Symbol(String::from("+")),
            Expr::Value(1.0),
            Expr::Value(2.0),
            Expr::Value(3.0),
        ]),
    ]);
    let code = ast.compile()?;
    let code = code + "\thlt\n";

    println!("{code}");
    RukaVM::new(asm(&code)?).start()?;
    Some(())
}

enum Expr {
    List(Vec<Expr>),
    Symbol(String),
    Value(f64),
}

impl Expr {
    fn parse(source: &str) -> Option<Expr> {}

    fn compile(&self) -> Option<String> {
        Some(match self {
            Expr::Value(literal) => format!("\tmov ar, {literal}\n"),
            Expr::List(list) => match list.first()? {
                Expr::Symbol(symbol) => {
                    macro_rules! multi_args {
                        ($name: expr => $list: expr, $init: expr) => {{
                            let mut result = String::new();
                            result.push_str(&format!("\tmov ar, {}\n", $init));
                            for expr in $list.iter().skip(1) {
                                result.push_str("\tpsh ar\n");
                                result.push_str(&expr.compile()?);
                                result.push_str(&format!("\tpop dr\n\t{} ar, dr\n", $name));
                            }
                            result
                        }};
                    }
                    match symbol.as_str() {
                        "+" => multi_args!("add" => list, 0.0),
                        "*" => multi_args!("mul" => list, 1.0),
                        _ => return None,
                    }
                }
                _ => return None,
            },
            _ => return None,
        })
    }
}

fn tokenize(input: &str, delimiter: &[char]) -> Option<Vec<String>> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;
    let mut is_escape = false;

    for c in input.chars() {
        if is_escape {
            current_token.push(match c {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                _ => c,
            });
            is_escape = false;
        } else {
            match c {
                '(' | '{' | '[' if !in_quote => {
                    current_token.push(c);
                    in_parentheses += 1;
                }
                ')' | '}' | ']' if !in_quote => {
                    current_token.push(c);
                    in_parentheses.checked_sub(1).map(|x| in_parentheses = x);
                }
                '"' | '\'' | '`' => {
                    in_quote = !in_quote;
                    current_token.push(c);
                }
                '\\' if in_quote => {
                    current_token.push(c);
                    is_escape = true;
                }
                other => {
                    if delimiter.contains(&other) && !in_quote {
                        if in_parentheses != 0 {
                            current_token.push(c);
                        } else if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                    } else {
                        current_token.push(c);
                    }
                }
            }
        }
    }

    // Syntax error check
    if is_escape || in_quote || in_parentheses != 0 {
        return None;
    }
    if !current_token.is_empty() {
        tokens.push(current_token.clone());
        current_token.clear();
    }
    Some(tokens)
}
