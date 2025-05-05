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

impl Keyword {
    pub fn japanese() -> Self {
        Keyword {
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
        }
    }
}
