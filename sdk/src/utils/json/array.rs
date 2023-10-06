// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, FromJson, ToJson, Value};

#[derive(Debug, PartialEq, Eq)]
pub enum ArrayError {
    WrongSize { expected: usize, found: usize },
    Item(String),
}

#[cfg(feature = "std")]
impl std::error::Error for ArrayError {}

impl core::fmt::Display for ArrayError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WrongSize { expected, found } => write!(f, "wrong array size: expected {expected}, found {found}"),
            Self::Item(e) => write!(f, "set item error: {e}"),
        }
    }
}

macro_rules! impl_json_array {
    ($type:ty) => {
        impl<T: ToJson> ToJson for $type {
            fn to_json(&self) -> Value {
                Value::Array(self.iter().map(ToJson::to_json).collect())
            }
        }

        impl<T: FromJson> FromJson for $type
        where
            T::Error: core::fmt::Display,
        {
            type Error = Error;

            fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
            where
                Self: Sized,
            {
                if let Value::Array(s) = value {
                    Ok(s.into_iter()
                        .map(FromJson::from_json)
                        .collect::<Result<_, T::Error>>()
                        .map_err(|e| ArrayError::Item(e.to_string()))?)
                } else {
                    Err(Error::wrong_type::<T>(value).into())
                }
            }
        }
    };
}
impl_json_array!(alloc::vec::Vec<T>);
impl_json_array!(alloc::boxed::Box<[T]>);

impl<T: ToJson> ToJson for [T] {
    fn to_json(&self) -> Value {
        Value::Array(self.iter().map(ToJson::to_json).collect())
    }
}

impl<T: ToJson, const N: usize> ToJson for [T; N] {
    fn to_json(&self) -> Value {
        Value::Array(self.iter().map(ToJson::to_json).collect())
    }
}

impl<T: FromJson, const N: usize> FromJson for [T; N]
where
    T::Error: core::fmt::Display,
{
    type Error = Error;

    fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if let Value::Array(s) = value {
            Ok(s.into_iter()
                .map(FromJson::from_json)
                .collect::<Result<Vec<T>, _>>()
                .map_err(|e| ArrayError::Item(e.to_string()))?
                .try_into()
                .map_err(|e: Vec<T>| ArrayError::WrongSize {
                    expected: N,
                    found: e.len(),
                })?)
        } else {
            Err(Error::wrong_type::<[T; N]>(value).into())
        }
    }
}
