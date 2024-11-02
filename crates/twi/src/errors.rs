use thiserror::Error;

use crate::runtime::gc::objects::ObjectInner;

#[derive(Debug, Error)]
pub enum TwiError {
    // ############### not found ###############
    #[error("`main` function is not found!")]
    MainNotFound,

    #[error("Type not found: {}", .0)]
    TypeNotFound(String),

    #[error("Identifier not found: {}", .0)]
    IdentifierNotFound(String),

    #[error("Model not found: {}", .0)]
    ModelNotFound(String),

    #[error("Member not found: {}", .0)]
    MemberNotFound(String),

    // ############### cannot ###############
    #[error("{:?} is not a model, cannot its member", .0)]
    CannotGetMember(String),

    #[error("{:?} is not a func, cannot call", .0)]
    CannotCall(String),

    // ############### type ###############
    #[error("Incompatible binary operation type: left {}, right: {}", left, right)]
    IncompatibleBinopType { left: String, right: String },

    #[error("Incompatible unary operation type: {}", .0)]
    IncompatibleUnopType(String),

    // ############### others ###############
    #[error("Invalid global definition: {} is not allowed in global scope", .0)]
    InvalidGlobalDefinition(String),

    // runtime
    #[error("Duplicate local binding: {}", .0)]
    DuplicateLocalBind(String), // ident

    #[error("Value of unexpected type: expected {}, got {}", expected, got)]
    UnexpectedType { expected: String, got: String },

    #[error("Unexpected statement: {}", .0)]
    UnexpectedStatement(String),

    #[error("Number of args mismatch: expected {}, got {}", expected, got)]
    ArgNumMismatch { expected: usize, got: usize },
}

pub type TwiResult<T> = Result<T, TwiError>;
