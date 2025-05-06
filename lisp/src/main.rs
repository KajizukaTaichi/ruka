use ruka_vm::{BasedMode, RukaVM, asm};

fn main() {
    println!("Hello, world!");
    run("(* 10 (+ 1 2 3))").unwrap();
}

fn run(source: &str) -> Option<f64> {
    let ast = Expr::parse(source)?;
    let code = ast.compile()?;
    let code = code + "\thlt\n";

    let mut vm = RukaVM::new(asm(&code)?);
    vm.start()?;

    Some(vm.returns(BasedMode::Register)?)
}

enum Expr {
    List(Vec<Expr>),
    Symbol(String),
    Value(f64),
}

impl Expr {
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

    fn parse(source: &str) -> Option<Expr> {
        let source = source.trim();
        if let Ok(n) = source.parse::<f64>() {
            Some(Expr::Value(n))
        } else if let Some(source) = source
            .strip_prefix("(")
            .map(|x| x.strip_suffix(")"))
            .flatten()
        {
            Some(Expr::List(
                tokenize(source)?
                    .iter()
                    .map(|x| Expr::parse(x))
                    .collect::<Option<Vec<Expr>>>()?,
            ))
        } else {
            Some(Expr::Symbol(source.to_string()))
        }
    }
}

fn tokenize(input: &str) -> Option<Vec<String>> {
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
                    if other.is_whitespace() && !in_quote {
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
