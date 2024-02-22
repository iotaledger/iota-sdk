// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
#[allow(missing_docs)]
pub struct ConversionError(String);

impl ConversionError {
    #[cfg(feature = "std")]
    pub fn new<E: std::error::Error>(e: E) -> Self {
        Self(e.to_string())
    }

    #[cfg(not(feature = "std"))]
    pub fn new<E: core::fmt::Display>(e: E) -> Self {
        Self(e.to_string())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ConversionError {}

pub trait ConvertTo<T>: Send + Sized {
    fn convert(self) -> Result<T, ConversionError>;

    fn convert_unchecked(self) -> T {
        self.convert().unwrap()
    }
}

impl<T: Send + Sized> ConvertTo<T> for T {
    fn convert(self) -> Result<T, ConversionError> {
        Ok(self)
    }

    fn convert_unchecked(self) -> T {
        self
    }
}

impl<T: Copy + Send + Sync + Sized> ConvertTo<T> for &T {
    fn convert(self) -> Result<T, ConversionError> {
        Ok(*self)
    }

    fn convert_unchecked(self) -> T {
        *self
    }
}
