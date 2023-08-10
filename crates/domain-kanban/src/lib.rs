use std::any::type_name;

use std::fmt::Debug as DebugTrait;
use std::marker::PhantomData;
use std::str::FromStr;

use thiserror::Error;
use ulid::Ulid;

pub mod board;
pub mod column;
pub mod user;

// TODO: Entityの一意性についてかんがえる

pub struct Identifier<T> {
    value: Ulid,
    _phantomdata: PhantomData<T>,
}
impl<T> Identifier<T> {
    pub fn new() -> Self {
        Self::from(Ulid::new())
    }
}

impl<T> From<Ulid> for Identifier<T> {
    fn from(value: Ulid) -> Self {
        Self {
            value,
            _phantomdata: PhantomData,
        }
    }
}

impl<T> DebugTrait for Identifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: Identifier<T> になってしまう
        // f.debug_tuple(type_name::<Self>())
        f.debug_tuple(&format!("Identifier<{}>", type_name::<T>()))
            .field(&self.value.to_string())
            .finish()
    }
}

impl<T> FromStr for Identifier<T> {
    type Err = <Ulid as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = Ulid::from_str(s)?;
        Ok(Self::from(value))
    }
}

// trait ModelInvariants
// where
//     Self: 'static + Sized,
// {
//     fn invariants() -> Vec<&'static Invariant<Self>>;
//     fn stisfy_invariants(self) -> InvariantResult<Self> {
//         let invariants = Self::invariants();
//         let init: Result<_, InvariantError> = Ok(self);
//         invariants
//             .iter()
//             .fold(init, |result, func| result.and_then(|v| func(v)))
//     }
// }
//
// type InvariantResult<T> = Result<T, InvariantError>;
// type Invariant<T> = dyn Fn(T) -> InvariantResult<T>;

#[derive(Debug, Error)]
pub enum InvariantError {
    #[error("不変条件違反: {0}")]
    ViolationError(String),
}
