// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;

use crate::types::block::{
    address::{Address, NftAddress},
    output::OutputId,
};

impl_id!(pub NftId, 32, "TODO.");

#[cfg(feature = "serde")]
string_serde_impl!(NftId);

impl From<&OutputId> for NftId {
    fn from(output_id: &OutputId) -> Self {
        Self::from(output_id.hash())
    }
}

impl NftId {
    ///
    pub fn or_from_output_id(self, output_id: &OutputId) -> Self {
        if self.is_null() { Self::from(output_id) } else { self }
    }

    /// Returns the bech32 encoding of the nft ID.
    pub fn to_bech32(&self, bech32_hrp: &str) -> String {
        Address::Nft(NftAddress::new(*self)).to_bech32(bech32_hrp)
    }
}

#[allow(missing_docs)]
pub mod dto {
    use alloc::string::ToString;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::error::dto::DtoError;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct NftIdDto(pub String);

    impl From<&NftId> for NftIdDto {
        fn from(value: &NftId) -> Self {
            Self(value.to_string())
        }
    }

    impl TryFrom<&NftIdDto> for NftId {
        type Error = DtoError;

        fn try_from(value: &NftIdDto) -> Result<Self, Self::Error> {
            value.0.parse::<Self>().map_err(|_| DtoError::InvalidField("NFT id"))
        }
    }
}
