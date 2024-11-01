use thiserror::Error;

use crate::runtime::gc::objects::ObjectInner;

#[derive(Debug, Error)]
pub enum TwiError {
    #[error("Type not found: {}", .0)]
    TypeNotFound(String),
    //
    #[error("Member not found: {}", .0)]
    MemberNotFound(String),

    #[error("{:?} is not a model, cannot its member", .0)]
    CannotGetMember(String),
}

pub type TwiResult<T> = Result<T, TwiError>;
