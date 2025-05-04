use crate::*;

#[derive(Debug, Clone)]
pub enum Oper {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Eql(Expr, Expr),
    Les(Expr, Expr),
}

impl Oper {
    pub fn parse(source: &str) -> Option<Self> {
        let token_list: Vec<String> = tokenize(source, SPACE.as_ref(), true)?;
        let token = Expr::parse(token_list.last()?)?;
        let operator = token_list.get(token_list.len().checked_sub(2)?)?;
        let has_lhs = |len: usize| Expr::parse(&join!(token_list.get(..token_list.len() - len)?));
        Some(match operator.as_str() {
            "+" => Oper::Add(has_lhs(2)?, token),
            "-" => Oper::Sub(token, has_lhs(2)?),
            "*" => Oper::Mul(has_lhs(2)?, token),
            "/" => Oper::Div(token, has_lhs(2)?),
            "=" => Oper::Eql(has_lhs(2)?, token),
            "<" => Oper::Les(has_lhs(2)?, token),
            ">" => Oper::Les(token, has_lhs(2)?),
            _ => return None,
        })
    }

    pub fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        let codegen = |lhs: &Expr, rhs: &Expr, opecode: &str, ctx: &mut Compiler| {
            let [lhs, rhs] = [expr!(lhs.compile(ctx)?), expr!(rhs.compile(ctx)?)];
            let code = format!("{lhs}\tpsh ar\n{rhs}\tmov dr, ar\n\tpop ar\n\t{opecode} ar, dr\n");
            Some(code)
        };
        Some(match self {
            Oper::Add(lhs, rhs) => codegen(lhs, rhs, "add", ctx)?,
            Oper::Sub(lhs, rhs) => codegen(lhs, rhs, "neg ar\n\tadd", ctx)?,
            Oper::Mul(lhs, rhs) => codegen(lhs, rhs, "mul", ctx)?,
            Oper::Div(lhs, rhs) => codegen(lhs, rhs, "inv ar\n\tmul", ctx)?,
            Oper::Eql(lhs, rhs) => codegen(lhs, rhs, "eql", ctx)?,
            Oper::Les(lhs, rhs) => codegen(lhs, rhs, "les", ctx)?,
        })
    }
}
