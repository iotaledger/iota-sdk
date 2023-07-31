// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::{FromUtf8Error, String};
use core::{convert::Infallible, fmt, num::ParseIntError};

use crypto::Error as CryptoError;
// use packable::bounded::BoundedU8;
use prefix_hex::Error as HexError;
use primitive_types::U256;

use crate::types::block::{
    input::UtxoInput,
    output::{
        feature::FeatureCount, unlock_condition::UnlockConditionCount, AccountId, ChainId, MetadataFeatureLength,
        NativeTokenCount, NftId, OutputIndex, StateMetadataLength, TagFeatureLength,
    },
    payload::{InputCount, OutputCount, TagLength, TaggedDataLength},
    unlock::{UnlockCount, UnlockIndex},
};

/// Error occurring when creating/parsing/validating blocks.
#[derive(Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Error {
    ConsumedAmountOverflow,
    ConsumedNativeTokensAmountOverflow,
    CreatedAmountOverflow,
    CreatedNativeTokensAmountOverflow,
    Crypto(CryptoError),
    DuplicateSignatureUnlock(u16),
    DuplicateUtxo(UtxoInput),
    ExpirationUnlockConditionZero,
    FeaturesNotUniqueSorted,
    InputUnlockCountMismatch { input_count: usize, unlock_count: usize },
    InvalidAddress,
    InvalidAddressKind(u8),
    InvalidAccountIndex(<UnlockIndex as TryFrom<u16>>::Error),
    InvalidStorageDepositAmount(u64),
    // The above is used by `Packable` to denote out-of-range values. The following denotes the actual amount.
    InsufficientStorageDepositAmount { amount: u64, required: u64 },
    StorageDepositReturnExceedsOutputAmount { deposit: u64, amount: u64 },
    InsufficientStorageDepositReturnAmount { deposit: u64, required: u64 },
    InvalidEssenceKind(u8),
    InvalidFeatureCount(<FeatureCount as TryFrom<usize>>::Error),
    InvalidFeatureKind(u8),
    InvalidFoundryOutputSupply { minted: U256, melted: U256, max: U256 },
    Hex(HexError),
    InvalidInputKind(u8),
    InvalidInputCount(<InputCount as TryFrom<usize>>::Error),
    InvalidInputOutputIndex(<OutputIndex as TryFrom<u16>>::Error),
    InvalidBech32Hrp(String),
    InvalidBlockLength(usize),
    InvalidStateMetadataLength(<StateMetadataLength as TryFrom<usize>>::Error),
    InvalidMetadataFeatureLength(<MetadataFeatureLength as TryFrom<usize>>::Error),
    InvalidNativeTokenCount(<NativeTokenCount as TryFrom<usize>>::Error),
    InvalidNetworkName(FromUtf8Error),
    InvalidNftIndex(<UnlockIndex as TryFrom<u16>>::Error),
    InvalidOutputAmount(u64),
    InvalidOutputCount(<OutputCount as TryFrom<usize>>::Error),
    InvalidOutputKind(u8),
    // TODO this would now need to be generic, not sure if possible.
    // https://github.com/iotaledger/iota-sdk/issues/647
    // InvalidParentCount(<BoundedU8 as TryFrom<usize>>::Error),
    InvalidParentCount,
    InvalidPayloadKind(u32),
    InvalidPayloadLength { expected: usize, actual: usize },
    InvalidReferenceIndex(<UnlockIndex as TryFrom<u16>>::Error),
    InvalidSignature,
    InvalidSignatureKind(u8),
    InvalidPublicKeyKind(u8),
    InvalidStringPrefix(<u8 as TryFrom<usize>>::Error),
    InvalidTaggedDataLength(<TaggedDataLength as TryFrom<usize>>::Error),
    InvalidTagFeatureLength(<TagFeatureLength as TryFrom<usize>>::Error),
    InvalidTagLength(<TagLength as TryFrom<usize>>::Error),
    InvalidTailTransactionHash,
    InvalidTokenSchemeKind(u8),
    InvalidTransactionAmountSum(u128),
    InvalidTransactionNativeTokensCount(u16),
    InvalidUnlockCount(<UnlockCount as TryFrom<usize>>::Error),
    InvalidUnlockKind(u8),
    InvalidUnlockReference(u16),
    InvalidUnlockAccount(u16),
    InvalidUnlockNft(u16),
    InvalidUnlockConditionCount(<UnlockConditionCount as TryFrom<usize>>::Error),
    InvalidUnlockConditionKind(u8),
    InvalidFoundryZeroSerialNumber,
    MissingAddressUnlockCondition,
    MissingGovernorUnlockCondition,
    MissingStateControllerUnlockCondition,
    NativeTokensNotUniqueSorted,
    NativeTokensNullAmount,
    NativeTokensOverflow,
    NetworkIdMismatch { expected: u64, actual: u64 },
    NonDisjointParents,
    NonZeroStateIndexOrFoundryCounter,
    ParentsNotUniqueSorted,
    ParseInt(ParseIntError),
    ProtocolVersionMismatch { expected: u8, actual: u8 },
    RemainingBytesAfterBlock,
    SelfControlledAccountOutput(AccountId),
    SelfDepositNft(NftId),
    SignaturePublicKeyMismatch { expected: String, actual: String },
    StorageDepositReturnOverflow,
    TimelockUnlockConditionZero,
    UnallowedFeature { index: usize, kind: u8 },
    UnallowedUnlockCondition { index: usize, kind: u8 },
    UnlockConditionsNotUniqueSorted,
    UnsupportedOutputKind(u8),
    DuplicateOutputChain(ChainId),
    InvalidField(&'static str),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConsumedAmountOverflow => write!(f, "consumed amount overflow"),
            Self::ConsumedNativeTokensAmountOverflow => write!(f, "consumed native tokens amount overflow"),
            Self::CreatedAmountOverflow => write!(f, "created amount overflow"),
            Self::CreatedNativeTokensAmountOverflow => write!(f, "created native tokens amount overflow"),
            Self::Crypto(e) => write!(f, "cryptographic error: {e}"),
            Self::DuplicateSignatureUnlock(index) => {
                write!(f, "duplicate signature unlock at index: {index}")
            }
            Self::DuplicateUtxo(utxo) => write!(f, "duplicate UTXO {utxo:?} in inputs"),
            Self::ExpirationUnlockConditionZero => {
                write!(
                    f,
                    "expiration unlock condition with milestone index and timestamp set to 0",
                )
            }
            Self::FeaturesNotUniqueSorted => write!(f, "features are not unique and/or sorted"),
            Self::InputUnlockCountMismatch {
                input_count,
                unlock_count,
            } => {
                write!(
                    f,
                    "input count and unlock count mismatch: {input_count} != {unlock_count}",
                )
            }
            Self::InvalidAddress => write!(f, "invalid address provided"),
            Self::InvalidAddressKind(k) => write!(f, "invalid address kind: {k}"),
            Self::InvalidAccountIndex(index) => write!(f, "invalid account index: {index}"),
            Self::InvalidBech32Hrp(err) => write!(f, "invalid bech32 hrp: {err}"),
            Self::InvalidStorageDepositAmount(amount) => {
                write!(f, "invalid storage deposit amount: {amount}")
            }
            Self::InsufficientStorageDepositAmount { amount, required } => {
                write!(
                    f,
                    "insufficient output amount for storage deposit: {amount} (should be at least {required})"
                )
            }
            Self::InsufficientStorageDepositReturnAmount { deposit, required } => {
                write!(
                    f,
                    "the return deposit ({deposit}) must be greater than the minimum storage deposit ({required})"
                )
            }
            Self::StorageDepositReturnExceedsOutputAmount { deposit, amount } => write!(
                f,
                "storage deposit return of {deposit} exceeds the original output amount of {amount}"
            ),
            Self::InvalidEssenceKind(k) => write!(f, "invalid essence kind: {k}"),
            Self::InvalidFeatureCount(count) => write!(f, "invalid feature count: {count}"),
            Self::InvalidFeatureKind(k) => write!(f, "invalid feature kind: {k}"),
            Self::InvalidFoundryOutputSupply { minted, melted, max } => write!(
                f,
                "invalid foundry output supply: minted {minted}, melted {melted} max {max}",
            ),
            Self::Hex(error) => write!(f, "hex error: {error}"),
            Self::InvalidInputKind(k) => write!(f, "invalid input kind: {k}"),
            Self::InvalidInputCount(count) => write!(f, "invalid input count: {count}"),
            Self::InvalidInputOutputIndex(index) => write!(f, "invalid input or output index: {index}"),
            Self::InvalidBlockLength(length) => write!(f, "invalid block length {length}"),
            Self::InvalidStateMetadataLength(length) => write!(f, "invalid state metadata length {length}"),
            Self::InvalidMetadataFeatureLength(length) => {
                write!(f, "invalid metadata feature length {length}")
            }
            Self::InvalidNativeTokenCount(count) => write!(f, "invalid native token count: {count}"),
            Self::InvalidNetworkName(err) => write!(f, "invalid network name: {err}"),
            Self::InvalidNftIndex(index) => write!(f, "invalid nft index: {index}"),
            Self::InvalidOutputAmount(amount) => write!(f, "invalid output amount: {amount}"),
            Self::InvalidOutputCount(count) => write!(f, "invalid output count: {count}"),
            Self::InvalidOutputKind(k) => write!(f, "invalid output kind: {k}"),
            Self::InvalidParentCount => {
                write!(f, "invalid parents count")
            }
            Self::InvalidPayloadKind(k) => write!(f, "invalid payload kind: {k}"),
            Self::InvalidPayloadLength { expected, actual } => {
                write!(f, "invalid payload length: expected {expected} but got {actual}")
            }
            Self::InvalidReferenceIndex(index) => write!(f, "invalid reference index: {index}"),
            Self::InvalidSignature => write!(f, "invalid signature provided"),
            Self::InvalidSignatureKind(k) => write!(f, "invalid signature kind: {k}"),
            Self::InvalidPublicKeyKind(k) => write!(f, "invalid public key kind: {k}"),
            Self::InvalidStringPrefix(p) => write!(f, "invalid string prefix: {p}"),
            Self::InvalidTaggedDataLength(length) => {
                write!(f, "invalid tagged data length {length}")
            }
            Self::InvalidTagFeatureLength(length) => {
                write!(f, "invalid tag feature length {length}")
            }
            Self::InvalidTagLength(length) => {
                write!(f, "invalid tag length {length}")
            }
            Self::InvalidTailTransactionHash => write!(f, "invalid tail transaction hash"),
            Self::InvalidTokenSchemeKind(k) => write!(f, "invalid token scheme kind {k}"),
            Self::InvalidTransactionAmountSum(value) => write!(f, "invalid transaction amount sum: {value}"),
            Self::InvalidTransactionNativeTokensCount(count) => {
                write!(f, "invalid transaction native tokens count: {count}")
            }
            Self::InvalidUnlockCount(count) => write!(f, "invalid unlock count: {count}"),
            Self::InvalidUnlockKind(k) => write!(f, "invalid unlock kind: {k}"),
            Self::InvalidUnlockReference(index) => {
                write!(f, "invalid unlock reference: {index}")
            }
            Self::InvalidUnlockAccount(index) => {
                write!(f, "invalid unlock account: {index}")
            }
            Self::InvalidUnlockNft(index) => {
                write!(f, "invalid unlock nft: {index}")
            }
            Self::InvalidUnlockConditionCount(count) => write!(f, "invalid unlock condition count: {count}"),
            Self::InvalidUnlockConditionKind(k) => write!(f, "invalid unlock condition kind: {k}"),
            Self::InvalidFoundryZeroSerialNumber => write!(f, "invalid foundry zero serial number"),
            Self::MissingAddressUnlockCondition => write!(f, "missing address unlock condition"),
            Self::MissingGovernorUnlockCondition => write!(f, "missing governor unlock condition"),
            Self::MissingStateControllerUnlockCondition => write!(f, "missing state controller unlock condition"),
            Self::NativeTokensNotUniqueSorted => write!(f, "native tokens are not unique and/or sorted"),
            Self::NativeTokensNullAmount => write!(f, "native tokens null amount"),
            Self::NativeTokensOverflow => write!(f, "native tokens overflow"),
            Self::NetworkIdMismatch { expected, actual } => {
                write!(f, "network ID mismatch: expected {expected} but got {actual}")
            }
            Self::NonDisjointParents => {
                write!(f, "weak parents are not disjoint to strong or shallow like parents")
            }
            Self::NonZeroStateIndexOrFoundryCounter => {
                write!(
                    f,
                    "non zero state index or foundry counter while account ID is all zero"
                )
            }
            Self::ParentsNotUniqueSorted => {
                write!(f, "parents are not unique and/or sorted")
            }
            Self::ParseInt(e) => {
                write!(f, "parse int error: {e}")
            }
            Self::ProtocolVersionMismatch { expected, actual } => {
                write!(f, "protocol version mismatch: expected {expected} but got {actual}")
            }
            Self::RemainingBytesAfterBlock => {
                write!(f, "remaining bytes after block")
            }
            Self::SelfControlledAccountOutput(account_id) => {
                write!(f, "self controlled account output, account ID {account_id}")
            }
            Self::SelfDepositNft(nft_id) => {
                write!(f, "self deposit nft output, NFT ID {nft_id}")
            }
            Self::SignaturePublicKeyMismatch { expected, actual } => {
                write!(f, "signature public key mismatch: expected {expected} but got {actual}",)
            }
            Self::StorageDepositReturnOverflow => {
                write!(f, "storage deposit return overflow",)
            }
            Self::TimelockUnlockConditionZero => {
                write!(
                    f,
                    "timelock unlock condition with milestone index and timestamp set to 0",
                )
            }
            Self::UnallowedFeature { index, kind } => {
                write!(f, "unallowed feature at index {index} with kind {kind}")
            }
            Self::UnallowedUnlockCondition { index, kind } => {
                write!(f, "unallowed unlock condition at index {index} with kind {kind}")
            }
            Self::UnlockConditionsNotUniqueSorted => write!(f, "unlock conditions are not unique and/or sorted"),
            Self::UnsupportedOutputKind(k) => write!(f, "unsupported output kind: {k}"),
            Self::DuplicateOutputChain(chain_id) => write!(f, "duplicate output chain {chain_id}"),
            Self::InvalidField(field) => write!(f, "invalid field: {field}"),
        }
    }
}

impl From<CryptoError> for Error {
    fn from(error: CryptoError) -> Self {
        Self::Crypto(error)
    }
}

impl From<Infallible> for Error {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}
