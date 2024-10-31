use lex::token::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Expected literal, found {:?}", .0)]
    NotLiteral(Token),

    #[error("Invalid sytnax: {}", .0)]
    InvalidSyntax(String), // message

    #[error("Function has duplicate arguments: {}", .0)]
    DuplicateArg(String), // message
}

pub type ParserResult<T> = Result<T, ParserError>;
