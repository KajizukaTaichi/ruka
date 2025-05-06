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
    Forth,
    Mind,
}

impl Keyword {
    pub fn new(lang: &Language) -> Self {
        match lang {
            Language::Forth => Keyword {
                add: String::from("+"),
                sub: String::from("-"),
                mul: String::from("*"),
                div: String::from("/"),
                equal: String::from("="),
                less_than: String::from("<"),
                greater_than: String::from(">"),
                load: String::from("@"),
                store: String::from("!"),
                define_start: String::from(":"),
                define_end: String::from(";"),
                if_then: String::from("if"),
                if_else: String::from("else"),
                if_end: String::from("then"),
            },
            Language::Mind => Keyword {
                add: String::from("足"),
                sub: String::from("引"),
                mul: String::from("掛"),
                div: String::from("割"),
                equal: String::from("等"),
                less_than: String::from("小"),
                greater_than: String::from("大"),
                load: String::from("読"),
                store: String::from("書"),
                define_start: String::from("とは"),
                define_end: String::from("こと。"),
                if_then: String::from("ならば"),
                if_else: String::from("でなければ"),
                if_end: String::from("つぎに"),
            },
        }
    }
}
