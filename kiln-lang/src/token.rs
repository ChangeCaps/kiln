#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Number(Number),
    Symbol(Symbol),
    Keyword(Keyword),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Float(f32),
    Int(i32),
    Uint(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    EqaulEqual,
    ColonColon,
    LtEqual,
    GtEqual,
    Equal,
    Colon,
    Plus,
    Minus,
    Asterisk,
    FSlash,
    Lt,
    Gt,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Enum,
    Union,
    Struct,
    Pub,
    Let,
    Fn,
}
