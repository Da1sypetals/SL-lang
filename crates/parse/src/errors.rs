use lex::token::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Expected literal, found {:?}", .0)]
    NotLiteral(Token),

    #[error(
        "Uncompleted expression: parsed to\n >> index = {} \non token sequence\n >> {:?}",
        index,
        tokens
    )]
    UncompletedExpr { index: usize, tokens: Vec<Token> },

    #[error("Invalid sytnax: {}", .0)]
    InvalidSyntax(String), // message

    #[error("Unexpected EOF!")]
    UnexpectedEof, // message

    #[error("Function has duplicate arguments: {}", .0)]
    DuplicateArg(String), // arg

    #[error("Model has duplicate fields: {}", .0)]
    DuplicateField(String), // fieldname

    #[error("Invalid expression: {}", .0)]
    InvalidExpression(String), // message

    #[error("Index out of bounds: {}", .0)]
    IndexOob(usize), // message
}

pub type ParserResult<T> = Result<T, ParserError>;
