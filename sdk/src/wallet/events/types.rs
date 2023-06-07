// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::Getters;
use serde::{Deserialize, Serialize, Serializer};

use crate::{
    client::api::PreparedTransactionDataDto,
    types::{
        api::core::response::OutputWithMetadataResponse,
        block::{
            address::Bech32Address,
            payload::transaction::{dto::TransactionPayloadDto, TransactionId},
        },
    },
    wallet::account::types::{InclusionState, OutputDataDto},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Associated account index.
    pub account_index: u32,
    /// The event
    pub event: WalletEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WalletEvent {
    ConsolidationRequired,
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
            T0,
            #[cfg(feature = "ledger_nano")]
            T1(&'a AddressData),
            T2(&'a NewOutputEvent),
            T3(&'a SpentOutputEvent),
            T4(&'a TransactionInclusionEvent),
            T5(TransactionProgressEvent_<'a>),
        }
        #[derive(Serialize)]
        struct TypedWalletEvent_<'a> {
            #[serde(rename = "type")]
            kind: u8,
            #[serde(flatten)]
            event: WalletEvent_<'a>,
        }
        let event = match self {
            Self::ConsolidationRequired => TypedWalletEvent_ {
                kind: WalletEventType::ConsolidationRequired as u8,
                event: WalletEvent_::T0,
            },
            #[cfg(feature = "ledger_nano")]
            Self::LedgerAddressGeneration(e) => TypedWalletEvent_ {
                kind: WalletEventType::LedgerAddressGeneration as u8,
                event: WalletEvent_::T1(e),
            },
            Self::NewOutput(e) => TypedWalletEvent_ {
                kind: WalletEventType::NewOutput as u8,
                event: WalletEvent_::T2(e),
            },
            Self::SpentOutput(e) => TypedWalletEvent_ {
                kind: WalletEventType::SpentOutput as u8,
                event: WalletEvent_::T3(e),
            },
            Self::TransactionInclusion(e) => TypedWalletEvent_ {
                kind: WalletEventType::TransactionInclusion as u8,
                event: WalletEvent_::T4(e),
            },
            Self::TransactionProgress(e) => TypedWalletEvent_ {
                kind: WalletEventType::TransactionProgress as u8,
                event: WalletEvent_::T5(TransactionProgressEvent_ { progress: e }),
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
                WalletEventType::ConsolidationRequired => Self::ConsolidationRequired,
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
pub enum WalletEventType {
    ConsolidationRequired = 0,
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerAddressGeneration = 1,
    NewOutput = 2,
    SpentOutput = 3,
    TransactionInclusion = 4,
    TransactionProgress = 5,
}

impl TryFrom<u8> for WalletEventType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let event_type = match value {
            0 => Self::ConsolidationRequired,
            #[cfg(feature = "ledger_nano")]
            1 => Self::LedgerAddressGeneration,
            2 => Self::NewOutput,
            3 => Self::SpentOutput,
            4 => Self::TransactionInclusion,
            5 => Self::TransactionProgress,
            _ => return Err(format!("invalid event type {value}")),
        };
        Ok(event_type)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOutputEvent {
    /// The new output.
    pub output: OutputDataDto,
    /// The transaction that created the output. Might be pruned and not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<TransactionPayloadDto>,
    /// The inputs for the transaction that created the output. Might be pruned and not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_inputs: Option<Vec<OutputWithMetadataResponse>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SpentOutputEvent {
    /// The spent output.
    pub output: OutputDataDto,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInclusionEvent {
    pub transaction_id: TransactionId,
    pub inclusion_state: InclusionState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransactionProgressEvent {
    /// Performing input selection.
    SelectingInputs,
    /// Generating remainder value deposit address.
    GeneratingRemainderDepositAddress(AddressData),
    /// Prepared transaction.
    PreparedTransaction(Box<PreparedTransactionDataDto>),
    /// Prepared transaction essence hash hex encoded, required for blindsigning with a ledger nano
    PreparedTransactionEssenceHash(String),
    /// Signing the transaction.
    SigningTransaction,
    /// Performing PoW.
    PerformingPow,
    /// Broadcasting.
    Broadcasting,
}

impl Serialize for TransactionProgressEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct PreparedTransactionEssenceHash_<'a> {
            hash: &'a str,
        }

        #[derive(Serialize)]
        #[serde(untagged)]
        enum TransactionProgressEvent_<'a> {
            T0,
            T1(&'a AddressData),
            T2(&'a PreparedTransactionDataDto),
            T3(PreparedTransactionEssenceHash_<'a>),
            T4,
            T5,
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
            Self::PreparedTransactionEssenceHash(e) => TypedTransactionProgressEvent_ {
                kind: 3,
                event: TransactionProgressEvent_::T3(PreparedTransactionEssenceHash_ { hash: e }),
            },
            Self::SigningTransaction => TypedTransactionProgressEvent_ {
                kind: 4,
                event: TransactionProgressEvent_::T4,
            },
            Self::PerformingPow => TypedTransactionProgressEvent_ {
                kind: 5,
                event: TransactionProgressEvent_::T5,
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
        struct PreparedTransactionEssenceHash_ {
            hash: String,
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
                3 => Self::PreparedTransactionEssenceHash(
                    PreparedTransactionEssenceHash_::deserialize(value)
                        .map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize PreparedTransactionEssenceHash: {e}"))
                        })?
                        .hash,
                ),
                4 => Self::SigningTransaction,
                5 => Self::PerformingPow,
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
