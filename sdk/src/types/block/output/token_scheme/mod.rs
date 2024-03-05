// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod error;
mod simple;

pub use self::{error::TokenSchemeError, simple::SimpleTokenScheme};
use crate::types::block::protocol::{WorkScore, WorkScoreParameters};

///
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, derive_more::From, packable::Packable)]
#[packable(unpack_error = TokenSchemeError)]
#[packable(tag_type = u8, with_error = TokenSchemeError::Kind)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum TokenScheme {
    ///
    #[packable(tag = SimpleTokenScheme::KIND)]
    Simple(SimpleTokenScheme),
}

impl core::fmt::Debug for TokenScheme {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Simple(scheme) => scheme.fmt(f),
        }
    }
}

impl TokenScheme {
    /// Returns the token scheme kind of a [`TokenScheme`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Simple(_) => SimpleTokenScheme::KIND,
        }
    }

    crate::def_is_as_opt!(TokenScheme: Simple);
}

impl WorkScore for TokenScheme {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        match self {
            Self::Simple(simple) => simple.work_score(params),
        }
    }
}

#[cfg(feature = "serde")]
crate::impl_deserialize_untagged!(TokenScheme: Simple);
