// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::Packable;

use crate::types::block::{output::AccountId, protocol::ProtocolParameters, Error};

/// An allotment of Mana which will be added upon commitment of the slot in which the containing transaction was issued,
/// in the form of Block Issuance Credits to the account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Packable)]
#[packable(unpack_error = Error)]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct ManaAllotment {
    pub(crate) account_id: AccountId,
    #[packable(verify_with = verify_mana)]
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

impl ManaAllotment {
    pub fn new(account_id: AccountId, mana: u64, protocol_params: &ProtocolParameters) -> Result<Self, Error> {
        verify_mana::<true>(&mana, protocol_params)?;

        Ok(Self { account_id, mana })
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn mana(&self) -> u64 {
        self.mana
    }
}

fn verify_mana<const VERIFY: bool>(mana: &u64, params: &ProtocolParameters) -> Result<(), Error> {
    if VERIFY && *mana > params.mana_parameters().max_mana() {
        return Err(Error::InvalidManaValue(*mana));
    }

    Ok(())
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

    impl TryFromDto<ManaAllotmentDto> for ManaAllotment {
        type Error = Error;

        fn try_from_dto_with_params_inner(
            dto: ManaAllotmentDto,
            params: Option<&ProtocolParameters>,
        ) -> Result<Self, Self::Error> {
            Ok(if let Some(params) = params {
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
