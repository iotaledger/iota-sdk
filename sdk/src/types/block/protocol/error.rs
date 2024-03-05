// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::FromUtf8Error;
use core::convert::Infallible;

use crate::types::block::{address::AddressError, mana::ManaError, protocol::ProtocolParametersHash};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum ProtocolParametersError {
    #[display(fmt = "invalid network name: {_0}")]
    NetworkName(FromUtf8Error),
    #[display(fmt = "invalid mana decay factors")]
    ManaDecayFactors,
    StringPrefix(<u8 as TryFrom<usize>>::Error),
    #[display(fmt = "invalid protocol parameters hash: expected {expected} but got {actual}")]
    Hash {
        expected: ProtocolParametersHash,
        actual: ProtocolParametersHash,
    },
    #[from]
    ManaParameters(ManaError),
    #[from]
    Address(AddressError),
}

#[cfg(feature = "std")]
impl std::error::Error for ProtocolParametersError {}

impl From<Infallible> for ProtocolParametersError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
