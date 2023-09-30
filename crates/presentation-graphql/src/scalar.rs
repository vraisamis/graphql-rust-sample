// DataloaderするのにIDの型が個別のほうが嬉しい
use async_graphql::InputValueError;
use async_graphql::Scalar;
use async_graphql::ScalarType;
use async_graphql::Value;
use std::fmt::Debug as DebugTrait;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct Id<T> {
    value: String,
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn value(&self) -> &str {
        &self.value
    }
}

// derives
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self._phantom == other._phantom
    }
}
impl<T> Eq for Id<T> {}
impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self._phantom.hash(state);
    }
}
impl<T> DebugTrait for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Id")
            .field("value", &self.value)
            .field("_phantom", &self._phantom)
            .finish()
    }
}
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _phantom: self._phantom.clone(),
        }
    }
}

// `Into<String>` な型は `Into<Id<T>>` になるようにする
impl<T, U> From<U> for Id<T>
where
    U: Into<String>,
{
    fn from(value: U) -> Self {
        Self {
            value: value.into(),
            _phantom: PhantomData,
        }
    }
}

#[Scalar]
impl<T: Send + Sync> ScalarType for Id<T> {
    fn parse(value: Value) -> async_graphql::InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(Self {
                value,
                _phantom: PhantomData,
            })
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        let v: &str = &self.value;
        Value::from(v)
    }
}
