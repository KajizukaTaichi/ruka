use crate::*;

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    If(Expr, Block, Option<Block>),
    While(Expr, Block),
    Sub(String, Vec<String>),
    Return(Expr),
    EndSub,
    ExitProgram,
}

impl Stmt {
    pub fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Stmt::Let(name, expr) => {
                let addr = ctx.variables.get(name).cloned();
                let addr = addr.unwrap_or(ctx.variables.len());
                ctx.variables.insert(name.to_string(), addr);
                let expr = expr.compile(ctx)?;
                format!("{}\tsta {addr}, ar\t; {name}\n", expr!(expr))
            }
            Stmt::If(expr, then, None) => {
                let expr = expr.compile(ctx)?;
                let then = then.compile(ctx)?;
                let label = ctx.if_label_index;
                ctx.if_label_index += 1;
                format!(
                    "{expr}\tjmp cr, if_then_{label}\n\tjmp 1, if_end_{label}\nif_then_{label}:\n{then}if_end_{label}:",
                    expr = cond!(expr),
                )
            }
            Stmt::If(expr, then, Some(els)) => {
                let expr = expr.compile(ctx)?;
                let then = then.compile(ctx)?;
                let els = els.compile(ctx)?;
                let label = ctx.if_label_index;
                ctx.if_label_index += 1;
                format!(
                    "{expr}\tjmp cr, if_then_{label}\n\tjmp 1, if_else_{label}\nif_then_{label}:\n{then}\tjmp 1, if_end_{label}\nif_end_{label}:{then}if_else_{label}:\n{els}if_end_{label}:",
                    expr = cond!(expr),
                )
            }
            Stmt::While(expr, block) => {
                let expr = expr.compile(ctx)?;
                let block = block.compile(ctx)?;
                let label = ctx.while_label_index;
                ctx.while_label_index += 1;
                format!(
                    "while_start_{label}:\n{expr}\tnor cr, cr\n\tjmp cr, while_end_{label}\n{block}jmp 1, while_start_{label}\nwhile_end_{label}:",
                    expr = cond!(expr),
                )
            }
            Stmt::Sub(name, args) => {
                let args = args.clone();
                format!(
                    "subroutine_{name}:\n{}",
                    args.iter()
                        .rev()
                        .map(|arg| {
                            let addr = ctx.variables.len();
                            ctx.variables.insert(arg.to_string(), addr);
                            format!("\tpop ar\n\tsta {addr}, ar\t; {arg}\n")
                        })
                        .collect::<Vec<_>>()
                        .concat()
                )
            }
            Stmt::Return(expr) => {
                let expr = expr.compile(ctx)?;
                format!("{}\tpsh ar\n\tret\n", expr!(expr))
            }
            Stmt::EndSub => "\tret\n".to_owned(),
            Stmt::ExitProgram => "\thlt\n".to_owned(),
        })
    }
}
