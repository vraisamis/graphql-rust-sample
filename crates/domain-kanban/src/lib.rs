pub mod board;
pub mod column;
pub mod user;

#[allow(unused)]
mod todo_invariants {
    use thiserror::Error;

    trait ModelInvariants
    where
        Self: 'static + Sized,
    {
        fn invariants() -> Vec<&'static Invariant<Self>>;
        fn stisfy_invariants(self) -> InvariantResult<Self> {
            let invariants = Self::invariants();
            let init: Result<_, InvariantError> = Ok(self);
            invariants
                .iter()
                .fold(init, |result, func| result.and_then(func))
        }
    }

    type InvariantResult<T> = Result<T, InvariantError>;
    type Invariant<T> = dyn Fn(T) -> InvariantResult<T>;

    #[derive(Debug, Error)]
    pub enum InvariantError {
        #[error("不変条件違反: {0}")]
        ViolationError(String),
    }
}
