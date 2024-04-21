use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InvariantError {
    #[error("不変条件違反: {0}")]
    ViolationError(String),
}
pub type InvariantResult<T> = Result<T, InvariantError>;
