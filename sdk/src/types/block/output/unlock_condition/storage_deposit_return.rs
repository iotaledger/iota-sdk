// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{address::Address, output::verify_output_amount, protocol::ProtocolParameters, Error};

/// Defines the amount of IOTAs used as storage deposit that have to be returned to the return [`Address`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
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
        verify_amount::<true>(&amount, &token_supply)?;

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

fn verify_amount<const VERIFY: bool>(amount: &u64, token_supply: &u64) -> Result<(), Error> {
    if VERIFY {
        verify_output_amount::<VERIFY>(amount, token_supply)
            .map_err(|_| Error::InvalidStorageDepositAmount(*amount))?;
    }

    Ok(())
}

fn verify_amount_packable<const VERIFY: bool>(
    amount: &u64,
    protocol_parameters: &ProtocolParameters,
) -> Result<(), Error> {
    verify_amount::<VERIFY>(amount, &protocol_parameters.token_supply())
}
