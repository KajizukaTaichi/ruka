use crate::*;

pub struct Block(Vec<Stmt>);

impl Block {
    pub fn parse(source: String) -> Option<Block> {
        let mut result = vec![];
        let mut block: String = String::new();
        let mut nest = 0;
        let mut temp: Option<Stmt> = None;
        let mut is_else = false;

        for line in source.lines() {
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }

            if nest == 0 {
                if let Some(line) = source.strip_prefix("let") {
                    let (name, value) = line.split_once("=")?;
                    result.push(Stmt::Let(name.trim().to_string(), Expr::parse(value)?));
                } else if let Some(line) = source.strip_prefix("if") {
                    temp = Some(Stmt::If(Expr::parse(&line)?, Block(vec![]), None));
                    nest += 1
                } else if let Some(line) = source.strip_prefix("while") {
                    temp = Some(Stmt::While(Expr::parse(&line)?, Block(vec![])));
                    nest += 1
                }
            } else {
                if line == "end".to_string() {
                    if nest == 1 {
                        match temp.clone()? {
                            Stmt::If(expr, true_code, _) => {
                                result.push(if is_else {
                                    Stmt::If(expr, true_code, Some(Block::parse(block.clone())?))
                                } else {
                                    Stmt::If(expr, Block::parse(block.clone())?, None)
                                });
                                block.clear();
                            }
                            Stmt::While(expr, _) => {
                                result.push(Stmt::While(expr, Block::parse(block.clone())?));
                                block.clear();
                            }
                            _ => {}
                        }
                    } else {
                        block += &format!("{line}\n");
                    }
                    nest -= 1;
                } else if line == "else".to_string() {
                    if nest == 1 {
                        match temp.clone()? {
                            Stmt::If(expr, _, _) => {
                                if is_else {
                                    return None;
                                } else {
                                    temp = Some(Stmt::If(expr, Block::parse(block.clone())?, None));
                                    block.clear();
                                }
                                is_else = true;
                            }
                            _ => {}
                        }
                    } else {
                        block += &format!("{line}\n");
                    }
                } else if line.starts_with("if") {
                    nest += 1;
                    block += &format!("{line}\n");
                } else if line.starts_with("while") {
                    nest += 1;
                    block += &format!("{line}\n");
                } else {
                    block += &format!("{line}\n");
                }
            }
        }
        Some(Block(result))
    }

    pub fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        self.0
            .iter()
            .map(|x| x.compile(ctx))
            .collect::<Option<Vec<_>>>()
            .map(|x| x.concat())
    }
}
