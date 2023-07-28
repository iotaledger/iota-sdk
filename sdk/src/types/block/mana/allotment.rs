// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::RangeInclusive;

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use crate::types::block::{output::AccountId, Error};

/// The maximum number of allotments of a transaction.
pub const ALLOTMENT_COUNT_MAX: u16 = 128;
/// The range of valid numbers of allotments of a transaction.
pub const ALLOTMENT_COUNT_RANGE: RangeInclusive<u16> = 1..=ALLOTMENT_COUNT_MAX; // [1..128]

/// TODO
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Allotment {
    account_id: AccountId,
    mana: u64,
}

impl Allotment {
    pub fn new(account_id: AccountId, mana: u64) -> Self {
        Self { account_id, mana }
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

pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::{block::Error, TryFromDto, ValidationParams};

    /// TODO
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct AllotmentDto {
        pub account_id: AccountId,
        #[serde(with = "crate::utils::serde::string")]
        pub mana: u64,
    }

    impl From<&Allotment> for AllotmentDto {
        fn from(value: &Allotment) -> Self {
            Self {
                account_id: value.account_id,
                mana: value.mana,
            }
        }
    }

    impl TryFromDto for Allotment {
        type Dto = AllotmentDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, _params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            // TODO: we may want to validate the mana amount?
            Ok(Self::new(dto.account_id, dto.mana))
        }
    }
}
