// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub enum Error {
    MissingValue(String),
    WrongType {
        expected: String,
        found: String,
    },
    InvalidKey {
        expected: String,
        found: String,
    },
    #[from]
    Array(super::array::ArrayError),
    #[from]
    Set(super::set::SetError),
    #[from]
    OrderedSet(super::set::OrderedSetError),
    Custom(String),
}

impl Error {
    pub fn wrong_type<E>(found: impl alloc::string::ToString) -> Self {
        Self::WrongType {
            expected: core::any::type_name::<E>().to_owned(),
            found: found.to_string(),
        }
    }

    pub fn invalid_key<K>(found: impl alloc::string::ToString) -> Self {
        Self::InvalidKey {
            expected: core::any::type_name::<K>().to_owned(),
            found: found.to_string(),
        }
    }

    pub fn missing_value<V>() -> Self {
        Self::MissingValue(core::any::type_name::<V>().to_owned())
    }

    pub fn custom<E: core::fmt::Display>(err: E) -> Self {
        Self::Custom(err.to_string())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingValue(v) => write!(f, "missing value: {v}"),
            Self::WrongType { expected, found } => {
                write!(f, "wrong type: expected {expected}, found {found}")
            }
            Self::InvalidKey { expected, found } => {
                write!(f, "invalid key: expected {expected}, found {found}")
            }
            Self::Array(e) => write!(f, "array error: {e}"),
            Self::Set(e) => write!(f, "set error: {e}"),
            Self::OrderedSet(e) => write!(f, "ordered set error: {e}"),
            Self::Custom(e) => e.fmt(f),
        }
    }
}
