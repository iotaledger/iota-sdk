// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod nft;
mod reference;
mod signature;

use alloc::boxed::Box;
use core::ops::RangeInclusive;

use derive_more::{Deref, From};
use hashbrown::HashSet;
use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix, Packable};

pub use self::{account::AccountUnlock, nft::NftUnlock, reference::ReferenceUnlock, signature::SignatureUnlock};
use crate::types::block::{
    input::{INPUT_COUNT_MAX, INPUT_COUNT_RANGE, INPUT_INDEX_MAX},
    Error,
};

/// The maximum number of unlocks of a transaction.
pub const UNLOCK_COUNT_MAX: u16 = INPUT_COUNT_MAX; // 128
/// The range of valid numbers of unlocks of a transaction.
pub const UNLOCK_COUNT_RANGE: RangeInclusive<u16> = INPUT_COUNT_RANGE; // [1..128]
/// The maximum index of unlocks of a transaction.
pub const UNLOCK_INDEX_MAX: u16 = INPUT_INDEX_MAX - 1; // 126
/// The range of valid indices of unlocks of a transaction that can be referenced in Reference, Alias or NFT unlocks.
pub const UNLOCK_INDEX_RANGE: RangeInclusive<u16> = 0..=UNLOCK_INDEX_MAX; // [0..126]

pub(crate) type UnlockIndex = BoundedU16<{ *UNLOCK_INDEX_RANGE.start() }, { *UNLOCK_INDEX_RANGE.end() }>;

/// Defines the mechanism by which a transaction input is authorized to be consumed.
#[derive(Clone, Eq, PartialEq, Hash, From, Packable)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidUnlockKind)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum Unlock {
    /// A signature unlock.
    #[packable(tag = SignatureUnlock::KIND)]
    #[from(ignore)]
    Signature(Box<SignatureUnlock>),
    /// A reference unlock.
    #[packable(tag = ReferenceUnlock::KIND)]
    Reference(ReferenceUnlock),
    /// An account unlock.
    #[packable(tag = AccountUnlock::KIND)]
    Account(AccountUnlock),
    /// An NFT unlock.
    #[packable(tag = NftUnlock::KIND)]
    Nft(NftUnlock),
}

impl From<SignatureUnlock> for Unlock {
    fn from(value: SignatureUnlock) -> Self {
        Self::Signature(value.into())
    }
}

impl core::fmt::Debug for Unlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Signature(unlock) => unlock.fmt(f),
            Self::Reference(unlock) => unlock.fmt(f),
            Self::Account(unlock) => unlock.fmt(f),
            Self::Nft(unlock) => unlock.fmt(f),
        }
    }
}

impl Unlock {
    /// Returns the unlock kind of an [`Unlock`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Signature(_) => SignatureUnlock::KIND,
            Self::Reference(_) => ReferenceUnlock::KIND,
            Self::Account(_) => AccountUnlock::KIND,
            Self::Nft(_) => NftUnlock::KIND,
        }
    }
}

pub(crate) type UnlockCount = BoundedU16<{ *UNLOCK_COUNT_RANGE.start() }, { *UNLOCK_COUNT_RANGE.end() }>;

/// A collection of unlocks.
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable)]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidUnlockCount(p.into())))]
pub struct Unlocks(#[packable(verify_with = verify_unlocks)] BoxedSlicePrefix<Unlock, UnlockCount>);

impl Unlocks {
    /// Creates a new [`Unlocks`].
    pub fn new(unlocks: impl Into<Box<[Unlock]>>) -> Result<Self, Error> {
        let unlocks: BoxedSlicePrefix<Unlock, UnlockCount> =
            unlocks.into().try_into().map_err(Error::InvalidUnlockCount)?;

        verify_unlocks::<true>(&unlocks, &())?;

        Ok(Self(unlocks))
    }

    /// Gets an [`Unlock`] from an [`Unlocks`].
    /// Returns the referenced unlock if the requested unlock was a reference.
    pub fn get(&self, index: usize) -> Option<&Unlock> {
        match self.0.get(index) {
            Some(Unlock::Reference(reference)) => self.0.get(reference.index() as usize),
            Some(unlock) => Some(unlock),
            None => None,
        }
    }
}

fn verify_unlocks<const VERIFY: bool>(unlocks: &[Unlock], _: &()) -> Result<(), Error> {
    if VERIFY {
        let mut seen_signatures = HashSet::new();

        for (index, unlock) in (0u16..).zip(unlocks.iter()) {
            match unlock {
                Unlock::Signature(signature) => {
                    if !seen_signatures.insert(signature) {
                        return Err(Error::DuplicateSignatureUnlock(index));
                    }
                }
                Unlock::Reference(reference) => {
                    if index == 0
                        || reference.index() >= index
                        || !matches!(unlocks[reference.index() as usize], Unlock::Signature(_))
                    {
                        return Err(Error::InvalidUnlockReference(index));
                    }
                }
                Unlock::Account(account) => {
                    if index == 0 || account.index() >= index {
                        return Err(Error::InvalidUnlockAccount(index));
                    }
                }
                Unlock::Nft(nft) => {
                    if index == 0 || nft.index() >= index {
                        return Err(Error::InvalidUnlockNft(index));
                    }
                }
            }
        }
    }

    Ok(())
}
