use crate::*;

struct Context {
    label_index: usize,
}

macro_rules! compile {
    ($node:expr => $ctx:expr) => {
        $node
            .iter()
            .map(|node| node.compile($ctx))
            .collect::<Vec<_>>()
            .join("\n\n")
    };
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
            Node::Call(word) => word.compile(ctx),
        }
    }
}
