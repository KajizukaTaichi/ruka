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
    fn compile(&self) -> Option<String> {
        Some(match self {
            Expr::Value(literal) => format!("\tmov ar, {literal}\n"),
            Expr::List(list) => match list.first()? {
                Expr::Symbol(symbol) => {
                    macro_rules! multi_args {
                        ($name: expr => $list: expr) => {{
                            let mut result = String::new();
                            result.push_str("\tmov ar, 0\n");
                            for expr in $list.iter().skip(1) {
                                result.push_str("\tpsh ar\n");
                                result.push_str(&expr.compile()?);
                                result.push_str(&format!("\tpop dr\n\t{} ar, dr\n", $name));
                            }
                            result
                        }};
                    }
                    match symbol.as_str() {
                        "+" => multi_args!("add" => list),
                        "*" => multi_args!("mul" => list),
                        _ => return None,
                    }
                }
                _ => return None,
            },
            _ => return None,
        })
    }
}
