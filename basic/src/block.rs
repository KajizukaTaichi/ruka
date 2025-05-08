use crate::*;

#[derive(Debug, Clone)]
pub struct Block(Vec<Stmt>);

impl Block {
    pub fn parse(source: String) -> Option<Block> {
        let mut result = vec![];
        let mut block: String = String::new();
        let mut temp: Option<Stmt> = None;
        let mut nest = 0;
        let mut is_else = false;

        for line in source.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if nest == 0 {
                if let Some(line) = line.strip_prefix("let") {
                    let (name, value) = line.split_once("=")?;
                    result.push(Stmt::Let(name.trim().to_string(), Expr::parse(value)?));
                } else if let Some(line) = line.strip_prefix("if") {
                    temp = Some(Stmt::If(Expr::parse(&line)?, Block(vec![]), None));
                    nest += 1
                } else if let Some(line) = line.strip_prefix("while") {
                    temp = Some(Stmt::While(Expr::parse(&line)?, Block(vec![])));
                    nest += 1
                } else if let Some(line) = line.strip_prefix("sub") {
                    let (name, args) = line.trim_end_matches(')').split_once('(')?;
                    let args = args.split(',').map(|s| s.trim().to_string()).collect();
                    result.push(Stmt::Sub(name.trim().to_string(), args));
                } else if line == "end sub" {
                    result.push(Stmt::EndSub);
                } else if line == "exit program" {
                    result.push(Stmt::ExitProgram);
                } else if let Some(line) = line.strip_prefix("return") {
                    result.push(Stmt::Return(Expr::parse(&line)?));
                }
            } else {
                if nest == 1 {
                    if line == "end if" {
                        match temp.clone()? {
                            Stmt::If(expr, true_code, _) => {
                                result.push(if is_else {
                                    Stmt::If(expr, true_code, Some(Block::parse(block.clone())?))
                                } else {
                                    Stmt::If(expr, Block::parse(block.clone())?, None)
                                });
                                block.clear();
                            }
                            _ => return None,
                        }
                    } else if line == "end while" {
                        match temp.clone()? {
                            Stmt::While(expr, _) => {
                                result.push(Stmt::While(expr, Block::parse(block.clone())?));
                                block.clear();
                            }
                            _ => return None,
                        }
                        nest -= 1;
                    } else if line == "else" {
                        match temp.clone()? {
                            Stmt::If(expr, _, _) if !is_else => {
                                temp = Some(Stmt::If(expr, Block::parse(block.clone())?, None));
                                block.clear();
                                is_else = true;
                            }
                            _ => return None,
                        }
                    }
                } else {
                    if line.starts_with("if") {
                        block += &format!("{line}\n");
                        nest += 1;
                    } else if line.starts_with("while") {
                        block += &format!("{line}\n");
                        nest += 1;
                    } else {
                        block += &format!("{line}\n");
                    }
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
