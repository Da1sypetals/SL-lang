use lex::token::{teer, Token};

use crate::errors::ParserError;

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Teer(teer),
    Nil,
}

impl TryFrom<Token> for Literal {
    type Error = ParserError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Int(x) => Ok(Literal::Int(x)),
            Token::Float(x) => Ok(Literal::Float(x)),
            Token::String(x) => Ok(Literal::String(x)),
            Token::Bool(x) => Ok(Literal::Bool(x)),
            Token::Teer(x) => Ok(Literal::Teer(x)),
            Token::Nil => Ok(Literal::Nil),
            t => Err(ParserError::NotLiteral(t)),
        }
    }
}
