use crate::*;

impl Expr {
    pub fn compile(&self) -> Option<String> {
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
