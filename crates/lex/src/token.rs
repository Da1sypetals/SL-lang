use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum teer {
    excel,
    empty,
    exile,
}

impl FromStr for teer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "excel" => Ok(teer::excel),
            "empty" => Ok(teer::empty),
            "exile" => Ok(teer::exile),
            _ => Err(format!("Invalid teer variant: {}", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // literal
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Teer(teer),
    Nil,

    // symbols
    Neq,
    And,
    Leq,
    Eq,
    Geq,
    Or,
    Not,
    Hash,
    Percent,
    Lpar,
    Rpar,
    Star,
    Plus,
    Comma,
    Minus,
    Dot,
    Slash,
    Colon,
    Semicolon,
    Lt,
    Assign,
    Gt,
    Matmul,
    Lbracket,
    Rbracket,
    Lbrace,
    Rbrace,

    // reserved
    Let,
    Func,
    Typeof,
    If,
    Else,
    Model,
    Print,
    For,
    While,
    Return,

    //
    Identifier(String),

    // finally
    Eof,
}
