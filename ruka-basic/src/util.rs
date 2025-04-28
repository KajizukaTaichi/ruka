pub const SPACE: [&str; 5] = [" ", "　", "\n", "\t", "\r"];
pub const OPERATOR: [&str; 15] = [
    "+", "-", "*", "/", "%", "^", "==", "!=", "<", ">", "<=", ">=", "&&", "||", "!",
];

pub fn include_letter(query: &str, chars: &Vec<String>, idx: usize) -> bool {
    chars
        .clone()
        .get(idx..idx + query.chars().count())
        .map(|i| query == i.concat())
        .unwrap_or(false)
}

#[macro_export]
macro_rules! join {
    ($x:expr) => {
        $x.join(&SPACE[0].to_string())
    };
}

#[macro_export]
macro_rules! ok {
    ($x:expr) => {
        if let Ok(x) = $x { Some(x) } else { None }
    };
}

#[macro_export]
macro_rules! cond {
    ($expr: expr) => {
        if $expr.contains("\n") {
            format!("{}\tmov cr, ar\n", $expr)
        } else {
            format!("\tmov cr, {}\n", $expr)
        }
    };
}
