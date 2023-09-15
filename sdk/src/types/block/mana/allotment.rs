// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use crate::types::block::{output::AccountId, protocol::ProtocolParameters, Error};

/// An allotment of Mana which will be added upon commitment of the slot in which the containing transaction was issued,
/// in the form of Block Issuance Credits to the account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ManaAllotment {
    pub(crate) account_id: AccountId,
    pub(crate) mana: u64,
}

impl PartialOrd for ManaAllotment {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ManaAllotment {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.account_id.cmp(&other.account_id)
    }
}

impl core::borrow::Borrow<AccountId> for ManaAllotment {
    fn borrow(&self) -> &AccountId {
        &self.account_id
    }
}

impl ManaAllotment {
    pub fn new(account_id: AccountId, mana: u64, protocol_params: &ProtocolParameters) -> Result<Self, Error> {
        if mana > protocol_params.mana_structure().max_mana() {
            return Err(Error::InvalidManaValue(mana));
        }
        Ok(Self { account_id, mana })
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn mana(&self) -> u64 {
        self.mana
    }
}

impl Packable for ManaAllotment {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.account_id.pack(packer)?;
        self.mana.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let account_id = AccountId::unpack::<_, VERIFY>(unpacker, &()).coerce()?;
        let mana = u64::unpack::<_, VERIFY>(unpacker, &()).coerce()?;

        Self::new(account_id, mana, visitor).map_err(UnpackError::Packable)
    }
}

#[cfg(feature = "serde")]
pub(super) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{types::TryFromDto, utils::serde::string};

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ManaAllotmentDto {
        pub account_id: AccountId,
        #[serde(with = "string")]
        pub mana: u64,
    }

    impl From<&ManaAllotment> for ManaAllotmentDto {
        fn from(value: &ManaAllotment) -> Self {
            Self {
                account_id: value.account_id,
                mana: value.mana,
            }
        }
    }

    impl TryFromDto for ManaAllotment {
        type Dto = ManaAllotmentDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(
            dto: Self::Dto,
            params: crate::types::ValidationParams<'_>,
        ) -> Result<Self, Self::Error> {
            Ok(if let Some(params) = params.protocol_parameters() {
                Self::new(dto.account_id, dto.mana, params)?
            } else {
                Self {
                    account_id: dto.account_id,
                    mana: dto.mana,
                }
            })
        }
    }
}
