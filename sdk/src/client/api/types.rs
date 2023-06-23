// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::slip10::Chain;
use serde::{Deserialize, Serialize};

use crate::{
    client::secret::types::InputSigningData,
    types::block::{
        address::Address,
        output::Output,
        payload::{
            transaction::{
                dto::{TransactionEssenceDto, TransactionPayloadDto},
                TransactionEssence,
            },
            TransactionPayload,
        },
        protocol::ProtocolParameters,
        Error,
    },
};

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedTransactionData {
    /// Transaction essence
    pub essence: TransactionEssence,
    /// Required input information for signing. Inputs need to be ordered by address type
    pub inputs_data: Vec<InputSigningData>,
    /// Optional remainder output information
    pub remainder: Option<RemainderData>,
}

/// PreparedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedTransactionDataDto {
    /// Transaction essence
    pub essence: TransactionEssenceDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
    /// Optional remainder output information
    pub remainder: Option<RemainderData>,
}

impl From<&PreparedTransactionData> for PreparedTransactionDataDto {
    fn from(value: &PreparedTransactionData) -> Self {
        Self {
            essence: TransactionEssenceDto::from(&value.essence),
            inputs_data: value.inputs_data.clone(),
            remainder: value.remainder.clone(),
        }
    }
}

impl PreparedTransactionData {
    /// Conversion from [`PreparedTransactionDataDto`] to [`PreparedTransactionData`].
    pub fn try_from_dto(
        value: PreparedTransactionDataDto,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<Self, Error> {
        Ok(Self {
            essence: TransactionEssence::try_from_dto(value.essence, protocol_parameters)
                .map_err(|_| Error::InvalidField("essence"))?,
            inputs_data: value.inputs_data,
            remainder: value.remainder,
        })
    }

    /// Unverified conversion from [`PreparedTransactionDataDto`] to [`PreparedTransactionData`].
    pub fn try_from_dto_unverified(value: PreparedTransactionDataDto) -> Result<Self, Error> {
        Ok(Self {
            essence: TransactionEssence::try_from_dto_unverified(value.essence)
                .map_err(|_| Error::InvalidField("essence"))?,
            inputs_data: value.inputs_data,
            remainder: value.remainder,
        })
    }
}

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedTransactionData {
    /// Signed transaction payload
    pub transaction_payload: TransactionPayload,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
}

/// SignedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedTransactionDataDto {
    /// Transaction essence
    pub transaction_payload: TransactionPayloadDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
}

impl From<&SignedTransactionData> for SignedTransactionDataDto {
    fn from(value: &SignedTransactionData) -> Self {
        Self {
            transaction_payload: TransactionPayloadDto::from(&value.transaction_payload),
            inputs_data: value.inputs_data.clone(),
        }
    }
}

impl SignedTransactionData {
    /// Conversion from [`SignedTransactionDataDto`] to [`SignedTransactionData`].
    pub fn try_from_dto(
        value: SignedTransactionDataDto,
        protocol_parameters: &ProtocolParameters,
    ) -> Result<Self, Error> {
        Ok(Self {
            transaction_payload: TransactionPayload::try_from_dto(value.transaction_payload, protocol_parameters)
                .map_err(|_| Error::InvalidField("transaction_payload"))?,
            inputs_data: value.inputs_data,
        })
    }

    /// Unverified conversion from [`SignedTransactionDataDto`] to [`SignedTransactionData`].
    pub fn try_from_dto_unverified(value: SignedTransactionDataDto) -> Result<Self, Error> {
        Ok(Self {
            transaction_payload: TransactionPayload::try_from_dto_unverified(value.transaction_payload)
                .map_err(|_| Error::InvalidField("transaction_payload"))?,
            inputs_data: value.inputs_data,
        })
    }
}

/// Data for a remainder output, used for ledger nano
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RemainderData {
    /// The remainder output
    pub output: Output,
    /// The chain derived from seed, for the remainder addresses
    pub chain: Option<Chain>,
    /// The remainder address
    pub address: Address,
}
