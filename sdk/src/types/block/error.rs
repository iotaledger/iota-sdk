// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::{FromUtf8Error, String};
use core::{convert::Infallible, fmt};

use bech32::primitives::hrp::Error as Bech32HrpError;
use crypto::Error as CryptoError;
use prefix_hex::Error as HexError;
use primitive_types::U256;

use super::slot::EpochIndex;
use crate::types::block::{
    address::WeightedAddressCount,
    context_input::RewardContextInputIndex,
    input::UtxoInput,
    mana::ManaAllotmentCount,
    output::{
        feature::{BlockIssuerKeyCount, FeatureCount},
        unlock_condition::UnlockConditionCount,
        AccountId, AnchorId, ChainId, MetadataFeatureKeyLength, MetadataFeatureLength, MetadataFeatureValueLength,
        NativeTokenCount, NftId, OutputIndex, TagFeatureLength,
    },
    payload::{
        tagged_data::{TagLength, TaggedDataLength},
        ContextInputCount, InputCount, OutputCount,
    },
    protocol::ProtocolParametersHash,
    unlock::{UnlockCount, UnlockIndex, UnlocksCount},
};

/// Error occurring when creating/parsing/validating blocks.
#[derive(Debug, PartialEq, Eq)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Error {
    ManaAllotmentsNotUniqueSorted,
    ConsumedAmountOverflow,
    ConsumedManaOverflow,
    ConsumedNativeTokensAmountOverflow,
    CreatedAmountOverflow,
    CreatedManaOverflow,
    CreatedNativeTokensAmountOverflow,
    Crypto(CryptoError),
    DuplicateBicAccountId(AccountId),
    DuplicateRewardInputIndex(u16),
    DuplicateSignatureUnlock(u16),
    DuplicateUtxo(UtxoInput),
    ExpirationUnlockConditionZero,
    FeaturesNotUniqueSorted,
    InputUnlockCountMismatch {
        input_count: usize,
        unlock_count: usize,
    },
    InvalidAddress,
    InvalidAddressKind(u8),
    InvalidAccountIndex(<UnlockIndex as TryFrom<u16>>::Error),
    InvalidAnchorIndex(<UnlockIndex as TryFrom<u16>>::Error),
    InvalidBlockBodyKind(u8),
    NonAsciiMetadataKey(Vec<u8>),
    InvalidRewardInputIndex(<RewardContextInputIndex as TryFrom<u16>>::Error),
    InvalidStorageDepositAmount(u64),
    /// Invalid transaction failure reason byte.
    InvalidTransactionFailureReason(u8),
    // The above is used by `Packable` to denote out-of-range values. The following denotes the actual amount.
    InsufficientStorageDepositAmount {
        amount: u64,
        required: u64,
    },
    StorageDepositReturnExceedsOutputAmount {
        deposit: u64,
        amount: u64,
    },
    InsufficientStorageDepositReturnAmount {
        deposit: u64,
        required: u64,
    },
    InvalidAddressWeight(u8),
    InvalidMultiAddressThreshold(u16),
    InvalidMultiAddressCumulativeWeight {
        cumulative_weight: u16,
        threshold: u16,
    },
    InvalidWeightedAddressCount(<WeightedAddressCount as TryFrom<usize>>::Error),
    InvalidMultiUnlockCount(<UnlocksCount as TryFrom<usize>>::Error),
    MultiUnlockRecursion,
    WeightedAddressesNotUniqueSorted,
    InvalidContextInputKind(u8),
    InvalidContextInputCount(<ContextInputCount as TryFrom<usize>>::Error),
    InvalidFeatureCount(<FeatureCount as TryFrom<usize>>::Error),
    InvalidFeatureKind(u8),
    InvalidFoundryOutputSupply {
        minted: U256,
        melted: U256,
        max: U256,
    },
    Hex(HexError),
    InvalidInputKind(u8),
    InvalidInputCount(<InputCount as TryFrom<usize>>::Error),
    InvalidInputOutputIndex(<OutputIndex as TryFrom<u16>>::Error),
    InvalidBech32Hrp(Bech32HrpError),
    InvalidCapabilitiesCount(<u8 as TryFrom<usize>>::Error),
    InvalidCapabilityByte {
        index: usize,
        byte: u8,
    },
    InvalidBlockLength(usize),
    InvalidManaValue(u64),
    InvalidMetadataFeature(String),
    InvalidMetadataFeatureLength(<MetadataFeatureLength as TryFrom<usize>>::Error),
    InvalidMetadataFeatureKeyLength(<MetadataFeatureKeyLength as TryFrom<usize>>::Error),
    InvalidMetadataFeatureValueLength(<MetadataFeatureValueLength as TryFrom<usize>>::Error),
    InvalidNativeTokenCount(<NativeTokenCount as TryFrom<usize>>::Error),
    InvalidNetworkName(FromUtf8Error),
    InvalidManaDecayFactors,
    InvalidNftIndex(<UnlockIndex as TryFrom<u16>>::Error),
    InvalidOutputAmount(u64),
    InvalidOutputCount(<OutputCount as TryFrom<usize>>::Error),
    InvalidOutputKind(u8),
    InvalidManaAllotmentCount(<ManaAllotmentCount as TryFrom<usize>>::Error),
    // TODO this would now need to be generic, not sure if possible.
    // https://github.com/iotaledger/iota-sdk/issues/647
    // InvalidParentCount(<BoundedU8 as TryFrom<usize>>::Error),
    InvalidParentCount,
    InvalidPayloadKind(u8),
    InvalidPayloadLength {
        expected: usize,
        actual: usize,
    },
    InvalidProtocolParametersHash {
        expected: ProtocolParametersHash,
        actual: ProtocolParametersHash,
    },
    InvalidBlockIssuerKeyCount(<BlockIssuerKeyCount as TryFrom<usize>>::Error),
    InvalidReferenceIndex(<UnlockIndex as TryFrom<u16>>::Error),
    InvalidSignature,
    InvalidSignatureKind(u8),
    InvalidBlockIssuerKeyKind(u8),
    InvalidStartEpoch(EpochIndex),
    InvalidStringPrefix(<u8 as TryFrom<usize>>::Error),
    InvalidTaggedDataLength(<TaggedDataLength as TryFrom<usize>>::Error),
    InvalidTagFeatureLength(<TagFeatureLength as TryFrom<usize>>::Error),
    InvalidTagLength(<TagLength as TryFrom<usize>>::Error),
    InvalidTokenSchemeKind(u8),
    InvalidTransactionAmountSum(u128),
    InvalidManaAllotmentSum {
        max: u64,
        sum: u128,
    },
    InvalidUnlockCount(<UnlockCount as TryFrom<usize>>::Error),
    InvalidUnlockKind(u8),
    InvalidUnlockReference(u16),
    InvalidUnlockAccount(u16),
    InvalidUnlockNft(u16),
    InvalidUnlockAnchor(u16),
    InvalidUnlockConditionCount(<UnlockConditionCount as TryFrom<usize>>::Error),
    InvalidUnlockConditionKind(u8),
    InvalidFoundryZeroSerialNumber,
    MissingAddressUnlockCondition,
    MissingGovernorUnlockCondition,
    MissingStateControllerUnlockCondition,
    MissingSlotIndex,
    NativeTokensNotUniqueSorted,
    NativeTokensNullAmount,
    NativeTokensOverflow,
    NetworkIdMismatch {
        expected: u64,
        actual: u64,
    },
    NonDisjointParents,
    NonZeroStateIndexOrFoundryCounter,
    ParentsNotUniqueSorted,
    ProtocolVersionMismatch {
        expected: u8,
        actual: u8,
    },
    BlockIssuerKeysNotUniqueSorted,
    RemainingBytesAfterBlock,
    SelfControlledAnchorOutput(AnchorId),
    SelfDepositAccount(AccountId),
    SelfDepositNft(NftId),
    SignaturePublicKeyMismatch {
        expected: String,
        actual: String,
    },
    StorageDepositReturnOverflow,
    TimelockUnlockConditionZero,
    TooManyCommitmentInputs,
    UnallowedFeature {
        index: usize,
        kind: u8,
    },
    UnallowedUnlockCondition {
        index: usize,
        kind: u8,
    },
    UnlockConditionsNotUniqueSorted,
    UnsupportedOutputKind(u8),
    // TODO use Address::kind_str when available in 2.0 ?
    UnsupportedAddressKind(u8),
    DuplicateOutputChain(ChainId),
    InvalidField(&'static str),
    NullDelegationValidatorId,
    InvalidEpochDelta {
        created: EpochIndex,
        target: EpochIndex,
    },
    TrailingCapabilityBytes,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ManaAllotmentsNotUniqueSorted => write!(f, "mana allotments are not unique and/or sorted"),
            Self::ConsumedAmountOverflow => write!(f, "consumed amount overflow"),
            Self::ConsumedManaOverflow => write!(f, "consumed mana overflow"),
            Self::ConsumedNativeTokensAmountOverflow => write!(f, "consumed native tokens amount overflow"),
            Self::CreatedAmountOverflow => write!(f, "created amount overflow"),
            Self::CreatedManaOverflow => write!(f, "created mana overflow"),
            Self::CreatedNativeTokensAmountOverflow => write!(f, "created native tokens amount overflow"),
            Self::Crypto(e) => write!(f, "cryptographic error: {e}"),
            Self::DuplicateBicAccountId(account_id) => write!(f, "duplicate BIC account id: {account_id}"),
            Self::DuplicateRewardInputIndex(idx) => write!(f, "duplicate reward input index: {idx}"),
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
            Self::InvalidAnchorIndex(index) => write!(f, "invalid anchor index: {index}"),
            Self::InvalidBech32Hrp(e) => write!(f, "invalid bech32 hrp: {e}"),
            Self::InvalidCapabilitiesCount(e) => write!(f, "invalid capabilities count: {e}"),
            Self::InvalidCapabilityByte { index, byte } => {
                write!(f, "invalid capability byte at index {index}: {byte:x}")
            }
            Self::InvalidBlockBodyKind(k) => write!(f, "invalid block body kind: {k}"),
            Self::NonAsciiMetadataKey(b) => write!(f, "non ASCII key: {b:?}"),
            Self::InvalidRewardInputIndex(idx) => write!(f, "invalid reward input index: {idx}"),
            Self::InvalidStorageDepositAmount(amount) => {
                write!(f, "invalid storage deposit amount: {amount}")
            }
            Self::InvalidTransactionFailureReason(reason_byte) => {
                write!(f, "invalid transaction failure reason byte: {reason_byte}")
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
                    "the return deposit ({deposit}) must be greater than the minimum output amount ({required})"
                )
            }
            Self::StorageDepositReturnExceedsOutputAmount { deposit, amount } => write!(
                f,
                "storage deposit return of {deposit} exceeds the original output amount of {amount}"
            ),
            Self::InvalidContextInputCount(count) => write!(f, "invalid context input count: {count}"),
            Self::InvalidAddressWeight(w) => write!(f, "invalid address weight: {w}"),
            Self::InvalidMultiAddressThreshold(t) => write!(f, "invalid multi address threshold: {t}"),
            Self::InvalidMultiAddressCumulativeWeight {
                cumulative_weight,
                threshold,
            } => {
                write!(
                    f,
                    "invalid multi address cumulative weight {cumulative_weight} < threshold {threshold}"
                )
            }
            Self::InvalidWeightedAddressCount(count) => write!(f, "invalid weighted address count: {count}"),
            Self::InvalidMultiUnlockCount(count) => write!(f, "invalid multi unlock count: {count}"),
            Self::MultiUnlockRecursion => write!(f, "multi unlock recursion"),
            Self::WeightedAddressesNotUniqueSorted => {
                write!(f, "weighted addresses are not unique and/or sorted")
            }
            Self::InvalidContextInputKind(k) => write!(f, "invalid context input kind: {k}"),
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
            Self::InvalidManaValue(mana) => write!(f, "invalid mana value: {mana}"),
            Self::InvalidMetadataFeature(e) => {
                write!(f, "invalid metadata feature: {e}")
            }
            Self::InvalidMetadataFeatureLength(length) => {
                write!(f, "invalid metadata feature length: {length}")
            }
            Self::InvalidMetadataFeatureKeyLength(length) => {
                write!(f, "invalid metadata feature key length: {length}")
            }
            Self::InvalidMetadataFeatureValueLength(length) => {
                write!(f, "invalid metadata feature value length: {length}")
            }
            Self::InvalidNativeTokenCount(count) => write!(f, "invalid native token count: {count}"),
            Self::InvalidNetworkName(err) => write!(f, "invalid network name: {err}"),
            Self::InvalidManaDecayFactors => write!(f, "invalid mana decay factors"),
            Self::InvalidNftIndex(index) => write!(f, "invalid nft index: {index}"),
            Self::InvalidOutputAmount(amount) => write!(f, "invalid output amount: {amount}"),
            Self::InvalidOutputCount(count) => write!(f, "invalid output count: {count}"),
            Self::InvalidOutputKind(k) => write!(f, "invalid output kind: {k}"),
            Self::InvalidManaAllotmentCount(count) => write!(f, "invalid mana allotment count: {count}"),
            Self::InvalidParentCount => {
                write!(f, "invalid parents count")
            }
            Self::InvalidPayloadKind(k) => write!(f, "invalid payload kind: {k}"),
            Self::InvalidPayloadLength { expected, actual } => {
                write!(f, "invalid payload length: expected {expected} but got {actual}")
            }
            Self::InvalidProtocolParametersHash { expected, actual } => {
                write!(
                    f,
                    "invalid protocol parameters hash: expected {expected} but got {actual}"
                )
            }
            Self::InvalidBlockIssuerKeyCount(count) => write!(f, "invalid block issuer key count: {count}"),
            Self::InvalidReferenceIndex(index) => write!(f, "invalid reference index: {index}"),
            Self::InvalidSignature => write!(f, "invalid signature provided"),
            Self::InvalidSignatureKind(k) => write!(f, "invalid signature kind: {k}"),
            Self::InvalidBlockIssuerKeyKind(k) => write!(f, "invalid block issuer key kind: {k}"),
            Self::InvalidStartEpoch(index) => write!(f, "invalid start epoch: {index}"),
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
            Self::InvalidTokenSchemeKind(k) => write!(f, "invalid token scheme kind {k}"),
            Self::InvalidTransactionAmountSum(value) => write!(f, "invalid transaction amount sum: {value}"),
            Self::InvalidManaAllotmentSum { max, sum } => {
                write!(f, "invalid mana allotment sum: {sum} greater than max of {max}")
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
            Self::InvalidUnlockAnchor(index) => {
                write!(f, "invalid unlock anchor: {index}")
            }
            Self::InvalidUnlockConditionCount(count) => write!(f, "invalid unlock condition count: {count}"),
            Self::InvalidUnlockConditionKind(k) => write!(f, "invalid unlock condition kind: {k}"),
            Self::InvalidFoundryZeroSerialNumber => write!(f, "invalid foundry zero serial number"),
            Self::MissingAddressUnlockCondition => write!(f, "missing address unlock condition"),
            Self::MissingGovernorUnlockCondition => write!(f, "missing governor unlock condition"),
            Self::MissingStateControllerUnlockCondition => write!(f, "missing state controller unlock condition"),
            Self::MissingSlotIndex => write!(f, "missing slot index"),
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
            Self::ProtocolVersionMismatch { expected, actual } => {
                write!(f, "protocol version mismatch: expected {expected} but got {actual}")
            }
            Self::BlockIssuerKeysNotUniqueSorted => write!(f, "block issuer keys are not unique and/or sorted"),
            Self::RemainingBytesAfterBlock => {
                write!(f, "remaining bytes after block")
            }
            Self::SelfControlledAnchorOutput(anchor_id) => {
                write!(f, "self controlled anchor output, anchor ID {anchor_id}")
            }
            Self::SelfDepositNft(nft_id) => {
                write!(f, "self deposit nft output, NFT ID {nft_id}")
            }
            Self::SelfDepositAccount(account_id) => {
                write!(f, "self deposit account output, account ID {account_id}")
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
            Self::TooManyCommitmentInputs => write!(f, "too many commitment inputs"),
            Self::UnallowedFeature { index, kind } => {
                write!(f, "unallowed feature at index {index} with kind {kind}")
            }
            Self::UnallowedUnlockCondition { index, kind } => {
                write!(f, "unallowed unlock condition at index {index} with kind {kind}")
            }
            Self::UnlockConditionsNotUniqueSorted => write!(f, "unlock conditions are not unique and/or sorted"),
            Self::UnsupportedOutputKind(k) => write!(f, "unsupported output kind: {k}"),
            Self::UnsupportedAddressKind(k) => write!(f, "unsupported address kind: {k}"),
            Self::DuplicateOutputChain(chain_id) => write!(f, "duplicate output chain {chain_id}"),
            Self::InvalidField(field) => write!(f, "invalid field: {field}"),
            Self::NullDelegationValidatorId => write!(f, "null delegation validator ID"),
            Self::InvalidEpochDelta { created, target } => {
                write!(f, "invalid epoch delta: created {created}, target {target}")
            }
            Self::TrailingCapabilityBytes => write!(f, "capability bytes have trailing zeroes"),
        }
    }
}

impl From<Bech32HrpError> for Error {
    fn from(error: Bech32HrpError) -> Self {
        Self::InvalidBech32Hrp(error)
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
