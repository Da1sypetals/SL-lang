use thiserror::Error;

use crate::runtime::gc::objects::ObjectInner;

#[derive(Debug, Error)]
pub enum TwiError {
    #[error("Type not found: {}", .0)]
    TypeNotFound(String),

    #[error("Identifier not found: {}", .0)]
    IdentifierNotFound(String),

    #[error("Member not found: {}", .0)]
    MemberNotFound(String),

    #[error("{:?} is not a model, cannot its member", .0)]
    CannotGetMember(String),

    #[error("Invalid global definition: {} is not allowed in global scope", .0)]
    InvalidGlobalDefinition(String),

    #[error("`main` function is not found!")]
    MainNotFound,

    // runtime
    #[error("Duplicate local binding: {}", .0)]
    DuplicateLocalBind(String), // ident

    #[error("Value of unexpected type: expected {}, got {}", expected, got)]
    UnexpectedType { expected: String, got: String }, // ident

    #[error("Unexpected statement: {}", .0)]
    UnexpectedStatement(String), // ident
}

pub type TwiResult<T> = Result<T, TwiError>;
