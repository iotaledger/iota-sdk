// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use super::MAX_THEORETICAL_MANA;
use crate::types::block::{output::AccountId, Error};

/// An allotment of Mana which will be added upon commitment of the slot in which the containing transaction was issued,
/// in the form of Block Issuance Credits to the account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "camelCase"))]
pub struct Allotment {
    pub(crate) account_id: AccountId,
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) mana: u64,
}

impl PartialOrd for Allotment {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Allotment {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.account_id.cmp(&other.account_id)
    }
}

impl Allotment {
    pub fn new(account_id: AccountId, mana: u64) -> Result<Self, Error> {
        if mana > MAX_THEORETICAL_MANA {
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

impl Packable for Allotment {
    type UnpackError = Error;
    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.account_id.pack(packer)?;
        self.mana.pack(packer)?;
        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let account_id = AccountId::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        let mana = u64::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        Ok(Self { account_id, mana })
    }
}

#[cfg(feature = "serde")]
mod dto {
    use serde::Deserialize;

    use super::*;
    use crate::utils::serde::string;

    impl<'de> Deserialize<'de> for Allotment {
        fn deserialize<D>(d: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct AllotmentDto {
                account_id: AccountId,
                #[serde(with = "string")]
                mana: u64,
            }

            impl TryFrom<AllotmentDto> for Allotment {
                type Error = Error;

                fn try_from(value: AllotmentDto) -> Result<Self, Self::Error> {
                    Self::new(value.account_id, value.mana)
                }
            }

            AllotmentDto::deserialize(d)?
                .try_into()
                .map_err(serde::de::Error::custom)
        }
    }
}
