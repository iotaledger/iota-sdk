// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{
    boxed::Box,
    string::{FromUtf8Error, String},
};
use core::{convert::Infallible, fmt};

use crypto::Error as CryptoError;
use packable::prefix::UnpackPrefixError;
use prefix_hex::Error as HexError;
use primitive_types::U256;

use super::slot::EpochIndex;
use crate::types::block::{
    context_input::RewardContextInputIndex,
    input::UtxoInput,
    mana::ManaAllotmentCount,
    output::{
        feature::FeatureCount, unlock_condition::UnlockConditionCount, AccountId, ChainId, MetadataFeatureLength,
        NativeTokenCount, NftId, OutputIndex, StateMetadataLength, TagFeatureLength,
    },
    payload::{ContextInputCount, InputCount, OutputCount, TagLength, TaggedDataLength},
    protocol::ProtocolParametersHash,
    public_key::PublicKeyCount,
    unlock::{UnlockCount, UnlockIndex},
};

/// Error occurring when creating/parsing/validating blocks.
#[derive(Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Error {
    ManaAllotmentsNotUniqueSorted,
    ConsumedAmountOverflow,
    ConsumedNativeTokensAmountOverflow,
    CreatedAmountOverflow,
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
    InvalidBlockKind(u8),
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
    InvalidContextInputKind(u8),
    InvalidContextInputCount(<ContextInputCount as TryFrom<usize>>::Error),
    InvalidEssenceKind(u8),
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
    InvalidBech32Hrp(String),
    InvalidBlockLength(usize),
    InvalidStateMetadataLength(<StateMetadataLength as TryFrom<usize>>::Error),
    InvalidManaValue(u64),
    InvalidMetadataFeatureLength(<MetadataFeatureLength as TryFrom<usize>>::Error),
    InvalidNativeTokenCount(<NativeTokenCount as TryFrom<usize>>::Error),
    InvalidNetworkName(FromUtf8Error),
    InvalidManaDecayFactors(UnpackOptionError<Infallible>),
    InvalidOption(Box<UnpackOptionError<Self>>),
    InvalidNftIndex(<UnlockIndex as TryFrom<u16>>::Error),
    InvalidOutputAmount(u64),
    InvalidOutputCount(<OutputCount as TryFrom<usize>>::Error),
    InvalidOutputKind(u8),
    InvalidManaAllotmentCount(<ManaAllotmentCount as TryFrom<usize>>::Error),
    // TODO this would now need to be generic, not sure if possible.
    // https://github.com/iotaledger/iota-sdk/issues/647
    // InvalidParentCount(<BoundedU8 as TryFrom<usize>>::Error),
    InvalidParentCount,
    InvalidPayloadKind(u32),
    InvalidPayloadLength {
        expected: usize,
        actual: usize,
    },
    InvalidProtocolParametersHash {
        expected: ProtocolParametersHash,
        actual: ProtocolParametersHash,
    },
    InvalidPublicKeyCount(<PublicKeyCount as TryFrom<usize>>::Error),
    InvalidReferenceIndex(<UnlockIndex as TryFrom<u16>>::Error),
    InvalidSignature,
    InvalidSignatureKind(u8),
    InvalidPublicKeyKind(u8),
    InvalidStartEpoch(EpochIndex),
    InvalidStringPrefix(<u8 as TryFrom<usize>>::Error),
    InvalidTaggedDataLength(<TaggedDataLength as TryFrom<usize>>::Error),
    InvalidTagFeatureLength(<TagFeatureLength as TryFrom<usize>>::Error),
    InvalidTagLength(<TagLength as TryFrom<usize>>::Error),
    InvalidTailTransactionHash,
    InvalidTokenSchemeKind(u8),
    InvalidTransactionAmountSum(u128),
    InvalidTransactionNativeTokensCount(u16),
    InvalidManaAllotmentSum(u128),
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
    PublicKeysNotUniqueSorted,
    RemainingBytesAfterBlock,
    SelfControlledAccountOutput(AccountId),
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
    DuplicateOutputChain(ChainId),
    InvalidField(&'static str),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ManaAllotmentsNotUniqueSorted => write!(f, "mana allotments are not unique and/or sorted"),
            Self::ConsumedAmountOverflow => write!(f, "consumed amount overflow"),
            Self::ConsumedNativeTokensAmountOverflow => write!(f, "consumed native tokens amount overflow"),
            Self::CreatedAmountOverflow => write!(f, "created amount overflow"),
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
            Self::InvalidBech32Hrp(err) => write!(f, "invalid bech32 hrp: {err}"),
            Self::InvalidBlockKind(k) => write!(f, "invalid block kind: {k}"),
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
                    "the return deposit ({deposit}) must be greater than the minimum storage deposit ({required})"
                )
            }
            Self::StorageDepositReturnExceedsOutputAmount { deposit, amount } => write!(
                f,
                "storage deposit return of {deposit} exceeds the original output amount of {amount}"
            ),
            Self::InvalidContextInputCount(count) => write!(f, "invalid context input count: {count}"),
            Self::InvalidContextInputKind(k) => write!(f, "invalid context input kind: {k}"),
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
            Self::InvalidStateMetadataLength(length) => write!(f, "invalid state metadata length: {length}"),
            Self::InvalidManaValue(mana) => write!(f, "invalid mana value: {mana}"),
            Self::InvalidMetadataFeatureLength(length) => {
                write!(f, "invalid metadata feature length: {length}")
            }
            Self::InvalidNativeTokenCount(count) => write!(f, "invalid native token count: {count}"),
            Self::InvalidNetworkName(err) => write!(f, "invalid network name: {err}"),
            Self::InvalidManaDecayFactors(err) => write!(f, "invalid mana decay factors: {err}"),
            Self::InvalidOption(err) => write!(f, "invalid optional field: {err}"),
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
            Self::InvalidPublicKeyCount(count) => write!(f, "invalid public key count: {count}"),
            Self::InvalidReferenceIndex(index) => write!(f, "invalid reference index: {index}"),
            Self::InvalidSignature => write!(f, "invalid signature provided"),
            Self::InvalidSignatureKind(k) => write!(f, "invalid signature kind: {k}"),
            Self::InvalidPublicKeyKind(k) => write!(f, "invalid public key kind: {k}"),
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
            Self::InvalidTailTransactionHash => write!(f, "invalid tail transaction hash"),
            Self::InvalidTokenSchemeKind(k) => write!(f, "invalid token scheme kind {k}"),
            Self::InvalidTransactionAmountSum(value) => write!(f, "invalid transaction amount sum: {value}"),
            Self::InvalidTransactionNativeTokensCount(count) => {
                write!(f, "invalid transaction native tokens count: {count}")
            }
            Self::InvalidManaAllotmentSum(value) => write!(f, "invalid mana allotment sum: {value}"),
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
            Self::ProtocolVersionMismatch { expected, actual } => {
                write!(f, "protocol version mismatch: expected {expected} but got {actual}")
            }
            Self::PublicKeysNotUniqueSorted => write!(f, "public keys are not unique and/or sorted"),
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
            Self::TooManyCommitmentInputs => write!(f, "too many commitment inputs"),
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

#[derive(Debug)]
pub struct UnpackOptionError<E>(packable::option::UnpackOptionError<E>);

impl<E: PartialEq> PartialEq for UnpackOptionError<E> {
    fn eq(&self, other: &Self) -> bool {
        use packable::option::UnpackOptionError as OtherErr;
        match (&self.0, &other.0) {
            (OtherErr::UnknownTag(t1), OtherErr::UnknownTag(t2)) => t1 == t2,
            (OtherErr::Inner(e1), OtherErr::Inner(e2)) => e1 == e2,
            _ => false,
        }
    }
}
impl<E: Eq> Eq for UnpackOptionError<E> {}

impl<E> UnpackOptionError<E> {
    fn map_opt_err<F: Fn(E) -> U, U>(self, f: F) -> UnpackOptionError<U> {
        use packable::option::UnpackOptionError as OtherErr;
        UnpackOptionError(match self.0 {
            OtherErr::UnknownTag(t) => OtherErr::UnknownTag(t),
            OtherErr::Inner(e) => OtherErr::Inner(f(e)),
        })
    }
}

#[cfg(feature = "std")]
impl<E: fmt::Debug + fmt::Display> std::error::Error for UnpackOptionError<E> {}

impl<E: fmt::Display> fmt::Display for UnpackOptionError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use packable::option::UnpackOptionError as OtherErr;
        match &self.0 {
            OtherErr::UnknownTag(t) => write!(f, "unknown tag: {t}"),
            OtherErr::Inner(e) => write!(f, "{e}"),
        }
    }
}

pub(crate) trait UnpackPrefixOptionErrorExt<E> {
    fn into_opt_error(self) -> UnpackOptionError<E>;
}

impl<E> UnpackPrefixOptionErrorExt<E> for packable::option::UnpackOptionError<E> {
    fn into_opt_error(self) -> UnpackOptionError<E> {
        UnpackOptionError(self)
    }
}

impl<E> UnpackPrefixOptionErrorExt<E> for packable::option::UnpackOptionError<UnpackPrefixError<E, Infallible>> {
    fn into_opt_error(self) -> UnpackOptionError<E> {
        use packable::option::UnpackOptionError as OtherErr;
        UnpackOptionError(match self {
            Self::UnknownTag(t) => OtherErr::UnknownTag(t),
            Self::Inner(e) => OtherErr::Inner(e.into_item_err()),
        })
    }
}

impl<E: Into<Self>> From<packable::option::UnpackOptionError<E>> for Error {
    fn from(value: packable::option::UnpackOptionError<E>) -> Self {
        Self::InvalidOption(value.into_opt_error().map_opt_err(Into::into).into())
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
