// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::output::{AccountId, FoundryId, NftId, OutputId};

///
#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChainId {
    ///
    Account(AccountId),
    ///
    Foundry(FoundryId),
    ///
    Nft(NftId),
}

impl core::fmt::Debug for ChainId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut formatter = f.debug_tuple("ChainId");
        match self {
            Self::Account(id) => formatter.field(id),
            Self::Foundry(id) => formatter.field(id),
            Self::Nft(id) => formatter.field(id),
        };
        formatter.finish()
    }
}

impl ChainId {
    ///
    pub fn is_null(&self) -> bool {
        match self {
            Self::Account(id) => id.is_null(),
            Self::Foundry(id) => id.is_null(),
            Self::Nft(id) => id.is_null(),
        }
    }

    ///
    pub fn or_from_output_id(self, output_id: &OutputId) -> Self {
        if !self.is_null() {
            return self;
        }

        match self {
            Self::Account(_) => Self::Account(AccountId::from(output_id)),
            Self::Foundry(_) => self,
            Self::Nft(_) => Self::Nft(NftId::from(output_id)),
        }
    }
}

impl core::fmt::Display for ChainId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Account(id) => write!(f, "{id}"),
            Self::Foundry(id) => write!(f, "{id}"),
            Self::Nft(id) => write!(f, "{id}"),
        }
    }
}
