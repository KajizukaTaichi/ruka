use crate::*;

type Env = IndexMap<String, usize>;

impl Expr {
    pub fn compile(&self, env: &mut Env) -> Option<String> {
        Some(match self {
            Expr::Value(literal) => format!("\tmov ar, {literal}\n"),
            Expr::Symbol(name) => format!("\tlda ar, {}\r; {name}\n", env.get(name)?),
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
                        "var" => {
                            let Expr::Symbol(name) = list.get(1)? else {
                                return None;
                            };
                            let addr = env.get(name).unwrap_or(&env.len()).clone();
                            let body = list.get(2)?.compile(env)?;
                            env.insert(name.to_string(), addr);
                            format!("{body}\tsta {addr}, ar\t; {name}\n")
                        }
                        "fn" => {
                            let Expr::Symbol(name) = list.get(1)? else {
                                return None;
                            };
                            let Expr::List(args) = list.get(2)? else {
                                return None;
                            };
                            let args = args
                                .iter()
                                .rev()
                                .map(|x| {
                                    if let Expr::Symbol(name) = x {
                                        Some(name.to_owned())
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Option<Vec<String>>>()?
                                .iter()
                                .map(|arg| {
                                    let addr = env.len();
                                    env.insert(arg.to_string(), addr);
                                    format!("\tpop ar\n\tsta {addr}, ar\t; {arg}\n")
                                })
                                .collect::<Vec<String>>();
                            format!(
                                "\tjmp 1, end_{name}\nfunction_{name}:\n{args}{body}\tret\nend_{name}:\n",
                                body = list.get(3)?.compile(env)?,
                                args = args.concat()
                            )
                        }
                        name => format!(
                            "{}\tcal function_{name}\n",
                            list.iter()
                                .skip(1)
                                .map(|x| x.compile(env).map(|x| format!("{x}\npsh ar\t")))
                                .collect::<Option<Vec<String>>>()?
                                .concat()
                        ),
                    }
                }
                _ => return None,
            },
        })
    }
}
