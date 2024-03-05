// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use crate::types::block::{
    address::AddressError,
    mana::ManaError,
    output::{
        feature::FeatureError, unlock_condition::UnlockConditionError, AccountId, AnchorId, NativeTokenError, NftId,
        TokenSchemeError,
    },
};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum OutputError {
    #[display(fmt = "invalid output kind: {_0}")]
    Kind(u8),
    #[display(fmt = "invalid output amount: {_0}")]
    Amount(u64),
    #[display(fmt = "consumed mana overflow")]
    ConsumedManaOverflow,
    #[display(fmt = "the return deposit ({deposit}) must be greater than the minimum output amount ({required})")]
    InsufficientStorageDepositReturnAmount { deposit: u64, required: u64 },
    #[display(fmt = "insufficient output amount: {amount} (should be at least {required})")]
    AmountLessThanMinimum { amount: u64, required: u64 },
    #[display(fmt = "storage deposit return of {deposit} exceeds the original output amount of {amount}")]
    StorageDepositReturnExceedsOutputAmount { deposit: u64, amount: u64 },
    #[display(fmt = "missing address unlock condition")]
    MissingAddressUnlockCondition,
    #[display(fmt = "missing governor address unlock condition")]
    MissingGovernorUnlockCondition,
    #[display(fmt = "missing state controller address unlock condition")]
    MissingStateControllerUnlockCondition,
    #[display(fmt = "null delegation validator ID")]
    NullDelegationValidatorId,
    #[display(fmt = "invalid foundry zero serial number")]
    InvalidFoundryZeroSerialNumber,
    #[display(fmt = "non zero state index or foundry counter while account ID is all zero")]
    NonZeroStateIndexOrFoundryCounter,
    #[display(fmt = "invalid stakes amount")]
    InvalidStakedAmount,
    #[display(fmt = "invalid validator address: {_0}")]
    ValidatorAddress(AddressError),
    #[display(fmt = "self deposit nft output, NFT ID {_0}")]
    SelfDepositNft(NftId),
    #[display(fmt = "self controlled anchor output, anchor ID {_0}")]
    SelfControlledAnchorOutput(AnchorId),
    #[display(fmt = "self deposit account output, account ID {_0}")]
    SelfDepositAccount(AccountId),
    #[from]
    UnlockCondition(UnlockConditionError),
    #[from]
    Feature(FeatureError),
    #[from]
    Mana(ManaError),
    #[from]
    NativeToken(NativeTokenError),
    #[from]
    TokenScheme(TokenSchemeError),
}

#[cfg(feature = "std")]
impl std::error::Error for OutputError {}

impl From<Infallible> for OutputError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
