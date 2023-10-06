// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, FromJson, JsonExt, ToJson, Value};

#[derive(Debug, PartialEq, Eq)]
pub enum SetError {
    Duplicate(String),
    Item(String),
}

#[cfg(feature = "std")]
impl std::error::Error for SetError {}

impl core::fmt::Display for SetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Duplicate(i) => write!(f, "duplicate set item: {i}"),
            Self::Item(e) => write!(f, "set item error: {e}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub enum OrderedSetError {
    Unordered(String),
}

#[cfg(feature = "std")]
impl std::error::Error for OrderedSetError {}

impl core::fmt::Display for OrderedSetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unordered(i) => write!(f, "unordered set item: {i}"),
        }
    }
}

#[cfg(feature = "std")]
mod std_hash {
    use super::*;

    impl<T: ToJson> ToJson for std::collections::HashSet<T> {
        fn to_json(&self) -> Value {
            Value::Array(self.iter().map(ToJson::to_json).collect())
        }
    }

    impl<T: FromJson + Eq + core::hash::Hash> FromJson for std::collections::HashSet<T>
    where
        T::Error: core::fmt::Display,
    {
        type Error = Error;

        fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if let Value::Array(s) = value {
                let mut set = Self::default();
                for value in s.into_iter() {
                    let v = value.to_value::<T>().map_err(|e| SetError::Item(e.to_string()))?;
                    if !set.insert(v) {
                        return Err(Error::Set(SetError::Duplicate(value.to_string())));
                    }
                }
                Ok(set)
            } else {
                Err(Error::wrong_type::<T>(value).into())
            }
        }
    }
}

impl<T: ToJson> ToJson for alloc::collections::BTreeSet<T> {
    fn to_json(&self) -> Value {
        Value::Array(self.iter().map(ToJson::to_json).collect())
    }
}

impl<T: FromJson + Eq + Ord + core::hash::Hash> FromJson for alloc::collections::BTreeSet<T>
where
    T::Error: core::fmt::Display,
{
    type Error = Error;

    fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if let Value::Array(s) = value {
            let mut set = Self::default();
            for value in s.into_iter() {
                let v = value.to_value::<T>().map_err(|e| SetError::Item(e.to_string()))?;
                if let Some(last) = set.last() {
                    match v.cmp(last) {
                        core::cmp::Ordering::Less => {
                            return Err(Error::OrderedSet(OrderedSetError::Unordered(value.to_string())));
                        }
                        core::cmp::Ordering::Equal => return Err(Error::Set(SetError::Duplicate(value.to_string()))),
                        _ => (),
                    }
                }
                set.insert(v);
            }
            Ok(set)
        } else {
            Err(Error::wrong_type::<T>(value).into())
        }
    }
}
