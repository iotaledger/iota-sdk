// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::ToString;
use core::str::FromStr;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::bounded::BoundedU16;

use crate::types::block::{output::OUTPUT_INDEX_RANGE, payload::signed_transaction::TransactionId, Error};

pub(crate) type OutputIndex = BoundedU16<{ *OUTPUT_INDEX_RANGE.start() }, { *OUTPUT_INDEX_RANGE.end() }>;

/// The identifier of an [`Output`](crate::types::block::output::Output).
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, packable::Packable)]
#[packable(unpack_error = Error)]
pub struct OutputId {
    transaction_id: TransactionId,
    index: u16,
}

impl OutputId {
    /// The length of a [`OutputId`].
    pub const LENGTH: usize = TransactionId::LENGTH + core::mem::size_of::<OutputIndex>();

    /// Creates a new [`OutputId`].
    pub fn new(transaction_id: TransactionId, index: u16) -> Self {
        Self { transaction_id, index }
    }

    /// Returns the [`TransactionId`] of an [`OutputId`].
    #[inline(always)]
    pub fn transaction_id(&self) -> &TransactionId {
        &self.transaction_id
    }

    /// Returns the index of an [`OutputId`].
    #[inline(always)]
    pub fn index(&self) -> u16 {
        self.index
    }

    /// Splits an [`OutputId`] into its [`TransactionId`] and index.
    #[inline(always)]
    pub fn split(self) -> (TransactionId, u16) {
        (self.transaction_id, self.index())
    }

    /// Hash the [`OutputId`] with BLAKE2b-256.
    #[inline(always)]
    pub fn hash(&self) -> [u8; 32] {
        Blake2b256::digest(self.to_bytes()).into()
    }

    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut buffer = [0u8; Self::LENGTH];
        let (transaction_id, index) = buffer.split_at_mut(TransactionId::LENGTH);
        transaction_id.copy_from_slice(self.transaction_id.as_ref());
        index.copy_from_slice(&self.index().to_le_bytes());
        buffer
    }
}

#[cfg(feature = "serde")]
crate::string_serde_impl!(OutputId);

impl From<[u8; Self::LENGTH]> for OutputId {
    fn from(bytes: [u8; Self::LENGTH]) -> Self {
        #[repr(C)]
        #[allow(dead_code)]
        struct Layout {
            transaction_id: TransactionId,
            index: [u8; 2],
        }
        unsafe {
            let Layout { transaction_id, index } = core::mem::transmute(bytes);
            Self::new(transaction_id, u16::from_le_bytes(index))
        }
    }
}

impl FromStr for OutputId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(
            prefix_hex::decode::<[u8; Self::LENGTH]>(s).map_err(Error::Hex)?,
        ))
    }
}

impl core::fmt::Display for OutputId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.to_bytes()))
    }
}

impl core::fmt::Debug for OutputId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OutputId")
            .field("id", &self.to_string())
            .field("transaction_id", &self.transaction_id)
            .field("output_index", &self.index)
            .finish()
    }
}
