// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    address::Address,
    output::{
        rent::{Rent, RentBuilder},
        verify_output_amount,
    },
    protocol::ProtocolParameters,
    Error,
};

/// Defines the amount of IOTAs used as storage deposit that have to be returned to the return [`Address`].
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct StorageDepositReturnUnlockCondition {
    // The [`Address`] to return the amount to.
    return_address: Address,
    // Amount of IOTA coins the consuming transaction should deposit to `return_address`.
    #[packable(verify_with = verify_amount_packable)]
    amount: u64,
}

impl StorageDepositReturnUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of a
    /// [`StorageDepositReturnUnlockCondition`].
    pub const KIND: u8 = 1;

    /// Creates a new [`StorageDepositReturnUnlockCondition`].
    #[inline(always)]
    pub fn new(return_address: impl Into<Address>, amount: u64, token_supply: u64) -> Result<Self, Error> {
        verify_amount::<true>(amount, token_supply)?;

        Ok(Self {
            return_address: return_address.into(),
            amount,
        })
    }

    /// Returns the return address.
    #[inline(always)]
    pub fn return_address(&self) -> &Address {
        &self.return_address
    }

    /// Returns the amount.
    #[inline(always)]
    pub fn amount(&self) -> u64 {
        self.amount
    }
}

impl Rent for StorageDepositReturnUnlockCondition {
    fn build_weighted_bytes(&self, builder: RentBuilder) -> RentBuilder {
        builder
            // Kind
            .data_field::<u8>()
            // Return address
            .packable_data_field(&self.return_address)
            // Return amount
            .data_field::<u64>()
    }
}

fn verify_amount<const VERIFY: bool>(amount: u64, token_supply: u64) -> Result<(), Error> {
    if VERIFY {
        verify_output_amount(amount, token_supply).map_err(|_| Error::InvalidStorageDepositAmount(amount))?;
    }

    Ok(())
}

fn verify_amount_packable<const VERIFY: bool>(
    amount: &u64,
    protocol_parameters: &ProtocolParameters,
) -> Result<(), Error> {
    verify_amount::<VERIFY>(*amount, protocol_parameters.token_supply())
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::format;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        types::{block::Error, TryFromDto, ValidationParams},
        utils::serde::string,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StorageDepositReturnUnlockConditionDto {
        #[serde(rename = "type", deserialize_with = "deserialize_kind")]
        pub kind: u8,
        pub return_address: Address,
        #[serde(with = "string")]
        pub amount: u64,
    }

    fn deserialize_kind<'de, D>(d: D) -> Result<u8, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let kind = u8::deserialize(d)?;
        if kind != StorageDepositReturnUnlockCondition::KIND {
            return Err(serde::de::Error::custom(format!(
                "invalid storage deposit return unlock condition type: expected {}, found {}",
                StorageDepositReturnUnlockCondition::KIND,
                kind
            )));
        }
        Ok(kind)
    }

    impl From<&StorageDepositReturnUnlockCondition> for StorageDepositReturnUnlockConditionDto {
        fn from(value: &StorageDepositReturnUnlockCondition) -> Self {
            Self {
                kind: StorageDepositReturnUnlockCondition::KIND,
                return_address: value.return_address.clone(),
                amount: value.amount,
            }
        }
    }

    impl TryFromDto for StorageDepositReturnUnlockCondition {
        type Dto = StorageDepositReturnUnlockConditionDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            Ok(if let Some(token_supply) = params.token_supply() {
                Self::new(dto.return_address, dto.amount, token_supply)?
            } else {
                Self {
                    return_address: dto.return_address,
                    amount: dto.amount,
                }
            })
        }
    }

    impl Serialize for StorageDepositReturnUnlockCondition {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            StorageDepositReturnUnlockConditionDto::from(self).serialize(s)
        }
    }
}
