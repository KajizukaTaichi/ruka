#[derive(Clone, Debug)]
pub struct Keyword {
    pub add: String,
    pub sub: String,
    pub mul: String,
    pub div: String,
    pub equal: String,
    pub less_than: String,
    pub greater_than: String,
    pub load: String,
    pub store: String,
    pub define_start: String,
    pub define_end: String,
    pub if_then: String,
    pub if_else: String,
    pub if_end: String,
}

#[derive(Clone, Debug)]
pub enum Language {
    Machine,
    Japanese,
    Russian,
}

impl Keyword {
    pub fn new(lang: &Language) -> Self {
        match lang {
            Language::Machine => Keyword {
                add: "+".to_string(),
                sub: "-".to_string(),
                mul: "*".to_string(),
                div: "/".to_string(),
                equal: "=".to_string(),
                less_than: "<".to_string(),
                greater_than: ">".to_string(),
                load: "@".to_string(),
                store: "!".to_string(),
                define_start: ":".to_string(),
                define_end: ";".to_string(),
                if_then: "?".to_string(),
                if_else: "¥".to_string(),
                if_end: "~".to_string(),
            },
            Language::Japanese => Keyword {
                add: "足".to_string(),
                sub: "引".to_string(),
                mul: "掛".to_string(),
                div: "割".to_string(),
                equal: "等".to_string(),
                less_than: "小".to_string(),
                greater_than: "大".to_string(),
                load: "読".to_string(),
                store: "書".to_string(),
                define_start: "とは".to_string(),
                define_end: "こと。".to_string(),
                if_then: "ならば".to_string(),
                if_else: "でなければ".to_string(),
                if_end: "つぎに".to_string(),
            },
            Language::Russian => Keyword {
                add: "сложи".to_string(),             // 「сложить（加える）」の命令形
                sub: "вычти".to_string(),             // 「вычесть（引く）」の命令形
                mul: "умножь".to_string(),            // 「умножить（掛ける）」の命令形
                div: "раздели".to_string(),           // 「разделить（割る）」の命令形
                equal: "равно".to_string(),           // 「等しい」
                less_than: "меньше".to_string(),      // 「より小さい」
                greater_than: "больше".to_string(),   // 「より大きい」
                load: "читай".to_string(),            // 「читать（読む）」の命令形
                store: "запиши".to_string(),          // 「записать（書き込む）」の命令形
                define_start: "определи".to_string(), // 「определить（定義する）」の命令形
                define_end: "конец".to_string(),      // 「終わり」「end」
                if_then: "если".to_string(),          // if
                if_else: "иначе".to_string(),         // else
                if_end: "всё".to_string(),            // then（完了・締めの意味で）
            },
        }
    }
}
