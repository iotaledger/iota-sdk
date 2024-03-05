// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::Getters;
use serde::{Deserialize, Serialize, Serializer};

use crate::{
    client::api::PreparedTransactionDataDto,
    types::block::{
        address::Bech32Address,
        output::OutputWithMetadata,
        payload::signed_transaction::{dto::SignedTransactionPayloadDto, TransactionId},
    },
    wallet::{
        types::{InclusionState, OutputData},
        WalletError,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WalletEvent {
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerAddressGeneration(AddressData),
    NewOutput(Box<NewOutputEvent>),
    SpentOutput(Box<SpentOutputEvent>),
    TransactionInclusion(TransactionInclusionEvent),
    TransactionProgress(TransactionProgressEvent),
}

impl Serialize for WalletEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct TransactionProgressEvent_<'a> {
            progress: &'a TransactionProgressEvent,
        }

        #[derive(Serialize)]
        #[serde(untagged)]
        enum WalletEvent_<'a> {
            #[cfg(feature = "ledger_nano")]
            T0(&'a AddressData),
            T1(&'a NewOutputEvent),
            T2(&'a SpentOutputEvent),
            T3(&'a TransactionInclusionEvent),
            T4(TransactionProgressEvent_<'a>),
        }
        #[derive(Serialize)]
        struct TypedWalletEvent_<'a> {
            #[serde(rename = "type")]
            kind: u8,
            #[serde(flatten)]
            event: WalletEvent_<'a>,
        }
        let event = match self {
            #[cfg(feature = "ledger_nano")]
            Self::LedgerAddressGeneration(e) => TypedWalletEvent_ {
                kind: WalletEventType::LedgerAddressGeneration as u8,
                event: WalletEvent_::T0(e),
            },
            Self::NewOutput(e) => TypedWalletEvent_ {
                kind: WalletEventType::NewOutput as u8,
                event: WalletEvent_::T1(e),
            },
            Self::SpentOutput(e) => TypedWalletEvent_ {
                kind: WalletEventType::SpentOutput as u8,
                event: WalletEvent_::T2(e),
            },
            Self::TransactionInclusion(e) => TypedWalletEvent_ {
                kind: WalletEventType::TransactionInclusion as u8,
                event: WalletEvent_::T3(e),
            },
            Self::TransactionProgress(e) => TypedWalletEvent_ {
                kind: WalletEventType::TransactionProgress as u8,
                event: WalletEvent_::T4(TransactionProgressEvent_ { progress: e }),
            },
        };
        event.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WalletEvent {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct TransactionProgressEvent_ {
            progress: TransactionProgressEvent,
        }

        let value = serde_json::Value::deserialize(d)?;
        Ok(
            match WalletEventType::try_from(
                value
                    .get("type")
                    .and_then(serde_json::Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid event type"))? as u8,
            )
            .map_err(serde::de::Error::custom)?
            {
                #[cfg(feature = "ledger_nano")]
                WalletEventType::LedgerAddressGeneration => {
                    Self::LedgerAddressGeneration(AddressData::deserialize(value).map_err(|e| {
                        serde::de::Error::custom(format!("cannot deserialize LedgerAddressGeneration: {e}"))
                    })?)
                }
                WalletEventType::NewOutput => {
                    Self::NewOutput(Box::new(NewOutputEvent::deserialize(value).map_err(|e| {
                        serde::de::Error::custom(format!("cannot deserialize NewOutput: {e}"))
                    })?))
                }
                WalletEventType::SpentOutput => {
                    Self::SpentOutput(Box::new(SpentOutputEvent::deserialize(value).map_err(|e| {
                        serde::de::Error::custom(format!("cannot deserialize SpentOutput: {e}"))
                    })?))
                }
                WalletEventType::TransactionInclusion => {
                    Self::TransactionInclusion(TransactionInclusionEvent::deserialize(value).map_err(|e| {
                        serde::de::Error::custom(format!("cannot deserialize TransactionInclusion: {e}"))
                    })?)
                }
                WalletEventType::TransactionProgress => Self::TransactionProgress(
                    TransactionProgressEvent_::deserialize(value)
                        .map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize TransactionProgressEvent: {e}"))
                        })?
                        .progress,
                ),
            },
        )
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum WalletEventType {
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerAddressGeneration = 0,
    NewOutput = 1,
    SpentOutput = 2,
    TransactionInclusion = 3,
    TransactionProgress = 4,
}

impl TryFrom<u8> for WalletEventType {
    type Error = WalletError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let event_type = match value {
            #[cfg(feature = "ledger_nano")]
            0 => Self::LedgerAddressGeneration,
            1 => Self::NewOutput,
            2 => Self::SpentOutput,
            3 => Self::TransactionInclusion,
            4 => Self::TransactionProgress,
            _ => return Err(WalletError::InvalidEventType(value)),
        };
        Ok(event_type)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOutputEvent {
    /// The new output.
    pub output: OutputData,
    /// The transaction that created the output. Might be pruned and not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<SignedTransactionPayloadDto>,
    /// The inputs for the transaction that created the output. Might be pruned and not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_inputs: Option<Vec<OutputWithMetadata>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SpentOutputEvent {
    /// The spent output.
    pub output: OutputData,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInclusionEvent {
    pub transaction_id: TransactionId,
    pub inclusion_state: InclusionState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TransactionProgressEvent {
    /// Performing input selection.
    SelectingInputs,
    /// Generating remainder value deposit address.
    GeneratingRemainderDepositAddress(AddressData),
    /// Prepared transaction.
    PreparedTransaction(Box<PreparedTransactionDataDto>),
    /// Signing the transaction.
    SigningTransaction,
    /// Prepared transaction signing hash hex encoded, required for blindsigning with a ledger nano
    PreparedTransactionSigningHash(String),
    /// Prepared block signing input, required for blind signing with ledger nano
    PreparedBlockSigningInput(String),
    /// Broadcasting.
    Broadcasting,
}

impl Serialize for TransactionProgressEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct PreparedTransactionSigningHash_<'a> {
            signing_hash: &'a str,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct PreparedBlockSigningInput_<'a> {
            block_signing_input: &'a str,
        }

        #[derive(Serialize)]
        #[serde(untagged)]
        enum TransactionProgressEvent_<'a> {
            T0,
            T1(&'a AddressData),
            T2(&'a PreparedTransactionDataDto),
            T3,
            T4(PreparedTransactionSigningHash_<'a>),
            T5(PreparedBlockSigningInput_<'a>),
            T6,
        }
        #[derive(Serialize)]
        struct TypedTransactionProgressEvent_<'a> {
            #[serde(rename = "type")]
            kind: u8,
            #[serde(flatten)]
            event: TransactionProgressEvent_<'a>,
        }
        let event = match self {
            Self::SelectingInputs => TypedTransactionProgressEvent_ {
                kind: 0,
                event: TransactionProgressEvent_::T0,
            },
            Self::GeneratingRemainderDepositAddress(e) => TypedTransactionProgressEvent_ {
                kind: 1,
                event: TransactionProgressEvent_::T1(e),
            },
            Self::PreparedTransaction(e) => TypedTransactionProgressEvent_ {
                kind: 2,
                event: TransactionProgressEvent_::T2(e),
            },
            Self::SigningTransaction => TypedTransactionProgressEvent_ {
                kind: 3,
                event: TransactionProgressEvent_::T3,
            },
            Self::PreparedTransactionSigningHash(e) => TypedTransactionProgressEvent_ {
                kind: 4,
                event: TransactionProgressEvent_::T4(PreparedTransactionSigningHash_ { signing_hash: e }),
            },
            Self::PreparedBlockSigningInput(e) => TypedTransactionProgressEvent_ {
                kind: 5,
                event: TransactionProgressEvent_::T5(PreparedBlockSigningInput_ { block_signing_input: e }),
            },
            Self::Broadcasting => TypedTransactionProgressEvent_ {
                kind: 6,
                event: TransactionProgressEvent_::T6,
            },
        };
        event.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TransactionProgressEvent {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct PreparedTransactionSigningHash_ {
            signing_hash: String,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct PreparedBlockSigningInput_ {
            block_signing_input: String,
        }

        let value = serde_json::Value::deserialize(d)?;
        Ok(
            match value
                .get("type")
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| serde::de::Error::custom("invalid transaction progress event type"))?
                as u8
            {
                0 => Self::SelectingInputs,
                1 => Self::GeneratingRemainderDepositAddress(AddressData::deserialize(value).map_err(|e| {
                    serde::de::Error::custom(format!("cannot deserialize GeneratingRemainderDepositAddress: {e}"))
                })?),
                2 => Self::PreparedTransaction(Box::new(PreparedTransactionDataDto::deserialize(value).map_err(
                    |e| serde::de::Error::custom(format!("cannot deserialize PreparedTransactionDataDto: {e}")),
                )?)),
                3 => Self::SigningTransaction,
                4 => Self::PreparedTransactionSigningHash(
                    PreparedTransactionSigningHash_::deserialize(value)
                        .map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize PreparedTransactionSigningHash: {e}"))
                        })?
                        .signing_hash,
                ),
                5 => Self::PreparedBlockSigningInput(
                    PreparedBlockSigningInput_::deserialize(value)
                        .map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize PreparedBlockSigningInput: {e}"))
                        })?
                        .block_signing_input,
                ),
                6 => Self::Broadcasting,
                _ => return Err(serde::de::Error::custom("invalid transaction progress event type")),
            },
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddressConsolidationNeeded {
    /// The associated address.
    pub address: Bech32Address,
}

/// Address event data.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, PartialEq, Eq, Hash)]
#[getset(get = "pub")]
pub struct AddressData {
    /// The address.
    #[getset(get = "pub")]
    pub address: Bech32Address,
}
