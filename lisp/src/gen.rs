use crate::*;

type Env = IndexMap<String, usize>;

impl Expr {
    pub fn compile(&self, env: &mut Env) -> Option<String> {
        Some(match self {
            Expr::Value(literal) => format!("\tmov ar, {literal}\n"),
            Expr::List(list) => match list.first()? {
                Expr::Symbol(symbol) => {
                    macro_rules! multi_args {
                        ($name: expr => $list: expr) => {{
                            let mut result = String::new();
                            result.push_str(&$list.get(1)?.compile(env)?);
                            for expr in $list.iter().skip(2) {
                                result.push_str("\tpsh ar\n");
                                result.push_str(&expr.compile(env)?);
                                result.push_str("\tmov dr, ar\n\tpop ar\n");
                                result.push_str(&format!("\t{} ar, dr\n", $name));
                            }
                            result
                        }};
                    }
                    match symbol.as_str() {
                        "+" => multi_args!("add" => list),
                        "*" => multi_args!("mul" => list),
                        "-" => multi_args!("neg dr\n\tadd" => list),
                        "/" => multi_args!("inv dr\n\tmul" => list),
                        name => format!("\tcal function_{name}\n"),
                    }
                }
                _ => return None,
            },
            _ => return None,
        })
    }
}
