use std::any::type_name;
use std::fmt::Debug as DebugTrait;
use std::marker::PhantomData;
use std::str::FromStr;

use ulid::Ulid;

mod board;
mod column;
mod user;

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
        f.debug_tuple("Identifier")
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
