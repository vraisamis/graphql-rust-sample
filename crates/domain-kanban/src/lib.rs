pub mod board;
pub mod column;
pub mod user;

#[allow(unused)]
mod todo_invariants {
    use thiserror::Error;

    pub trait ModelInvariants
    where
        Self: 'static + Sized,
    {
        fn invariants() -> Vec<&'static Invariant<Self>>;
        fn stisfy_invariants_ref(&self) -> InvariantResult<&Self> {
            let init: Result<_, InvariantError> = Ok(());
            Self::invariants()
                .iter()
                .fold(init, |result, func| result.and_then(|_| func(self)))
                .map(|_| self)
        }
        fn stisfy_invariants_mut(&mut self) -> InvariantResult<&mut Self> {
            let init: Result<_, InvariantError> = Ok(());
            Self::invariants()
                .iter()
                .fold(init, |result, func| result.and_then(|_| func(&self)))
                .map(|_| self)
        }
        fn stisfy_invariants(self) -> InvariantResult<Self> {
            let init: Result<_, InvariantError> = Ok(());
            Self::invariants()
                .iter()
                .fold(init, |result, func| result.and_then(|_| func(&self)))
                .map(|_| self)
        }
    }

    pub type InvariantResult<T> = Result<T, InvariantError>;
    pub type Invariant<T> = dyn Fn(&T) -> InvariantResult<()>;

    #[derive(Debug, Error, Clone, PartialEq, Eq)]
    pub enum InvariantError {
        #[error("不変条件違反: {0}")]
        ViolationError(String),
    }
}
