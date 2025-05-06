use crate::*;

impl Expr {
    pub fn parse(source: &str) -> Option<Expr> {
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
