// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use crate::types::block::{
    signature::SignatureError,
    unlock::{multi::UnlocksCount, UnlockCount, UnlockIndex},
};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum UnlockError {
    #[display(fmt = "invalid unlock kind: {_0}")]
    Kind(u8),
    #[display(fmt = "invalid unlock count: {_0}")]
    Count(<UnlockCount as TryFrom<usize>>::Error),
    #[display(fmt = "invalid unlock reference: {_0}")]
    Reference(u16),
    #[display(fmt = "invalid unlock account: {_0}")]
    Account(u16),
    #[display(fmt = "invalid unlock nft: {_0}")]
    Nft(u16),
    #[display(fmt = "invalid unlock anchor: {_0}")]
    Anchor(u16),
    #[display(fmt = "duplicate signature unlock at index: {_0}")]
    DuplicateSignature(u16),
    #[display(fmt = "multi unlock recursion")]
    MultiUnlockRecursion,
    #[display(fmt = "invalid account index: {_0}")]
    AccountIndex(<UnlockIndex as TryFrom<u16>>::Error),
    #[display(fmt = "invalid anchor index: {_0}")]
    AnchorIndex(<UnlockIndex as TryFrom<u16>>::Error),
    #[display(fmt = "invalid nft index: {_0}")]
    NftIndex(<UnlockIndex as TryFrom<u16>>::Error),
    #[display(fmt = "invalid reference index: {_0}")]
    ReferenceIndex(<UnlockIndex as TryFrom<u16>>::Error),
    #[display(fmt = "invalid multi unlock count: {_0}")]
    MultiUnlockCount(<UnlocksCount as TryFrom<usize>>::Error),
    #[from]
    Signature(SignatureError),
}

#[cfg(feature = "std")]
impl std::error::Error for UnlockError {}

impl From<Infallible> for UnlockError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
