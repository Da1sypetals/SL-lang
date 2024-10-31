use lex::token::Token;

use crate::{errors::ParserError, types::literal::Literal};

use super::root::Root;

#[derive(Debug, Clone)]
pub enum ExprNode {
    Literal(Literal),
    Identifer(String),
}

impl TryFrom<Vec<Token>> for ExprNode {
    type Error = ParserError;

    fn try_from(value: Vec<Token>) -> Result<Self, Self::Error> {
        // only 1 token
        if value.len() == 1 {
            let token = value[0].clone();
            // literal
            if let Ok(lit) = Literal::try_from(token.clone()) {
                return Ok(Self::Literal(lit));
            }
            // ident
            if let Token::Identifier(ident) = token.clone() {
                return Ok(Self::Identifer(ident));
            }
        }

        todo!()
    }
}
