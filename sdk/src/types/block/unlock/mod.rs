// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod account;
mod anchor;
mod empty;
mod error;
mod multi;
mod nft;
mod reference;
mod signature;

use alloc::boxed::Box;
use core::ops::RangeInclusive;

use derive_more::{Deref, From};
use hashbrown::HashSet;
use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix, Packable};

pub use self::{
    account::AccountUnlock, anchor::AnchorUnlock, empty::EmptyUnlock, error::UnlockError, multi::MultiUnlock,
    nft::NftUnlock, reference::ReferenceUnlock, signature::SignatureUnlock,
};
use crate::types::block::{
    input::{INPUT_COUNT_MAX, INPUT_COUNT_RANGE, INPUT_INDEX_MAX},
    protocol::{WorkScore, WorkScoreParameters},
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
#[packable(unpack_error = UnlockError)]
#[packable(tag_type = u8, with_error = UnlockError::Kind)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
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
    /// An anchor unlock.
    #[packable(tag = AnchorUnlock::KIND)]
    Anchor(AnchorUnlock),
    /// An NFT unlock.
    #[packable(tag = NftUnlock::KIND)]
    Nft(NftUnlock),
    /// A multi unlock.
    #[packable(tag = MultiUnlock::KIND)]
    Multi(MultiUnlock),
    /// An empty unlock.
    #[packable(tag = EmptyUnlock::KIND)]
    Empty(EmptyUnlock),
}

impl Unlock {
    /// Returns the unlock kind of an [`Unlock`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Signature(_) => SignatureUnlock::KIND,
            Self::Reference(_) => ReferenceUnlock::KIND,
            Self::Account(_) => AccountUnlock::KIND,
            Self::Anchor(_) => AnchorUnlock::KIND,
            Self::Nft(_) => NftUnlock::KIND,
            Self::Multi(_) => MultiUnlock::KIND,
            Self::Empty(_) => EmptyUnlock::KIND,
        }
    }

    crate::def_is_as_opt!(Unlock: Signature, Reference, Account, Anchor, Nft, Multi, Empty);
}

impl WorkScore for Unlock {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        match self {
            Self::Signature(unlock) => unlock.work_score(params),
            Self::Reference(unlock) => unlock.work_score(params),
            Self::Account(unlock) => unlock.work_score(params),
            Self::Anchor(unlock) => unlock.work_score(params),
            Self::Nft(unlock) => unlock.work_score(params),
            Self::Multi(unlock) => unlock.work_score(params),
            Self::Empty(unlock) => unlock.work_score(params),
        }
    }
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
            Self::Anchor(unlock) => unlock.fmt(f),
            Self::Nft(unlock) => unlock.fmt(f),
            Self::Multi(unlock) => unlock.fmt(f),
            Self::Empty(unlock) => unlock.fmt(f),
        }
    }
}

pub(crate) type UnlockCount = BoundedU16<{ *UNLOCK_COUNT_RANGE.start() }, { *UNLOCK_COUNT_RANGE.end() }>;

/// A collection of unlocks.
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable)]
#[packable(unpack_error = UnlockError, with = |e| e.unwrap_item_err_or_else(|p| UnlockError::Count(p.into())))]
pub struct Unlocks(#[packable(verify_with = verify_unlocks)] BoxedSlicePrefix<Unlock, UnlockCount>);

impl Unlocks {
    /// Creates a new [`Unlocks`].
    pub fn new(unlocks: impl Into<Box<[Unlock]>>) -> Result<Self, UnlockError> {
        let unlocks: BoxedSlicePrefix<Unlock, UnlockCount> = unlocks.into().try_into().map_err(UnlockError::Count)?;

        verify_unlocks(&unlocks)?;

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

/// Verifies the consistency of non-multi unlocks.
/// Will error on multi unlocks as they can't be nested.
fn verify_non_multi_unlock<'a>(
    unlocks: &'a [Unlock],
    unlock: &'a Unlock,
    index: u16,
    seen_signatures: &mut HashSet<&'a SignatureUnlock>,
) -> Result<(), UnlockError> {
    match unlock {
        Unlock::Signature(signature) => {
            if !seen_signatures.insert(signature.as_ref()) {
                return Err(UnlockError::DuplicateSignature(index));
            }
        }
        Unlock::Reference(reference) => {
            if index == 0
                || reference.index() >= index
                || !matches!(unlocks[reference.index() as usize], Unlock::Signature(_))
            {
                return Err(UnlockError::Reference(index));
            }
        }
        Unlock::Account(account) => {
            if index == 0 || account.index() >= index {
                return Err(UnlockError::Account(index));
            }
        }
        Unlock::Anchor(anchor) => {
            if index == 0 || anchor.index() >= index {
                return Err(UnlockError::Anchor(index));
            }
        }
        Unlock::Nft(nft) => {
            if index == 0 || nft.index() >= index {
                return Err(UnlockError::Nft(index));
            }
        }
        Unlock::Multi(_) => return Err(UnlockError::MultiUnlockRecursion),
        Unlock::Empty(_) => {}
    }

    Ok(())
}

fn verify_unlocks(unlocks: &[Unlock]) -> Result<(), UnlockError> {
    let mut seen_signatures = HashSet::new();

    for (index, unlock) in (0u16..).zip(unlocks.iter()) {
        match unlock {
            Unlock::Multi(multi) => {
                for unlock in multi.unlocks() {
                    verify_non_multi_unlock(unlocks, unlock, index, &mut seen_signatures)?
                }
            }
            _ => verify_non_multi_unlock(unlocks, unlock, index, &mut seen_signatures)?,
        }
    }

    Ok(())
}

#[cfg(feature = "serde")]
crate::impl_deserialize_untagged!(Unlock: Signature, Reference, Account, Anchor, Nft, Multi, Empty);
