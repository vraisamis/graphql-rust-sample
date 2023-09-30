use std::fmt::{Debug as DebugTrait, Display};
use std::hash::Hash;
use std::marker::PhantomData;
use std::str::FromStr;

use thiserror::Error;
use ulid::{DecodeError, Ulid};

// TODO: Entityの一意性についてかんがえる
pub trait Entity {
    fn entity_type() -> &'static str;
}

pub struct Identifier<T: Entity> {
    value: Ulid,
    _phantomdata: PhantomData<T>,
}
impl<T: Entity> Identifier<T> {
    pub fn gen() -> Self {
        Self::from(Ulid::new())
    }
    pub fn new(value: Ulid) -> Self {
        Self {
            value,
            _phantomdata: PhantomData,
        }
    }
}

impl<T, V> From<V> for Identifier<T>
where
    T: Entity,
    V: Into<Ulid>,
{
    fn from(value: V) -> Self {
        let ulid: Ulid = value.into();
        Self {
            value: ulid,
            _phantomdata: PhantomData,
        }
    }
}

impl<T: Entity> DebugTrait for Identifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("Identifier<{}>", T::entity_type()))
            .field(&self.value.to_string())
            .finish()
    }
}

impl<T: Entity> FromStr for Identifier<T> {
    type Err = IdentifierParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (type_name, ulid) = s
            .split_once('-')
            .ok_or(IdentifierParseError::NotContainsSeparator)?;
        let value = Ulid::from_str(ulid)?;
        let expected = T::entity_type();
        if type_name == T::entity_type() {
            Ok(Self::from(value))
        } else {
            Err(IdentifierParseError::InvalidTypePrefix {
                expected: expected.to_string(),
                actual: type_name.to_string(),
            })
        }
    }
}

impl<T: Entity> Display for Identifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", T::entity_type(), self.value)?;
        Ok(())
    }
}

impl<T: Entity> PartialEq for Identifier<T> {
    fn eq(&self, other: &Self) -> bool {
        // NOTE: Tは同じことが保証されているので entity_type は比較しなくていい
        self.value == other.value && self._phantomdata == other._phantomdata
    }
}

impl<T: Entity> Eq for Identifier<T> {}

impl<T: Entity> Clone for Identifier<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _phantomdata: self._phantomdata,
        }
    }
}

impl<T: Entity> Hash for Identifier<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self._phantomdata.hash(state);
    }
}

// Error
#[derive(Debug, Error)]
pub enum IdentifierParseError {
    #[error("文字列に `-` が含まれていません")]
    NotContainsSeparator,
    #[error("ULIDパース失敗: {0}")]
    ParseError(#[from] DecodeError),
    #[error("IDの型が違います (expected: {expected}, actual: {actual})")]
    InvalidTypePrefix { expected: String, actual: String },
}
