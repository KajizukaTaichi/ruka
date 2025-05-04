use crate::*;

pub struct Context {
    pub label_index: usize,
}

#[macro_export]
macro_rules! compile {
    ($node:expr => $ctx:expr) => {
        $node
            .iter()
            .map(|node| node.compile($ctx))
            .collect::<Vec<_>>()
            .concat()
    };
}

impl Define {
    pub fn compile(&self, ctx: &mut Context) -> String {
        let Define(name, body) = self;
        let body = compile!(body => ctx);
        format!("word_{name}:\n{body}\tret\n")
    }
}

impl Node {
    fn compile(&self, ctx: &mut Context) -> String {
        match self {
            Node::Value(n) => format!("\tpsh {n}\n"),
            Node::If(then, else_) => {
                let [then, else_] = [compile!(then => ctx), compile!(else_ => ctx)];
                let label = ctx.label_index;
                ctx.label_index += 1;
                format!(
                    "\tpop cr\n\tjmp cr, then_{label}\n\tjmp 1, else_{label}\nthen_{label}:\n{then}\tjmp 1, end_{label}\nelse_{label}:\n{else_}end_{label}:\n",
                )
            }
            Node::Call(word) => word.compile(),
        }
    }
}

impl Word {
    fn compile(&self) -> String {
        match self {
            Word::Add => format!("\tpop dr\n\tpop ar\n\tadd ar, dr\n\tpsh ar\n"),
            Word::Mul => format!("\tpop dr\n\tpop ar\n\tmul ar, dr\n\tpsh ar\n"),
            Word::Sub => format!("\tpop dr\n\tpop ar\n\tneg dr\n\tadd ar, dr\n\tpsh ar\n"),
            Word::Div => format!("\tpop dr\n\tpop ar\n\tinv dr\n\tadd ar, dr\n\tpsh ar\n"),
            Word::Equal => format!("\tpop dr\n\tpop ar\n\teql ar, dr\n\tpsh ar\n"),
            Word::LessThan => format!("\tpop dr\n\tpop ar\n\tles ar, dr\n\tpsh ar\n"),
            Word::GreaterThan => format!("\tpop ar\n\tpop dr\n\tles ar, dr\n\tpsh ar\n"),
            Word::User(name) => format!("\tcal word_{name}\n"),
        }
    }
}
