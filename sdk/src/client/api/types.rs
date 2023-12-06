// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use serde::{Deserialize, Serialize};

use crate::{
    client::secret::types::InputSigningData,
    types::{
        block::{
            address::Address,
            output::Output,
            payload::{
                signed_transaction::{
                    dto::{SignedTransactionPayloadDto, TransactionDto},
                    Transaction,
                },
                SignedTransactionPayload,
            },
            protocol::ProtocolParameters,
            Error,
        },
        TryFromDto,
    },
    utils::serde::bip44::option_bip44,
};

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedTransactionData {
    /// Transaction
    pub transaction: Transaction,
    /// Required input information for signing. Inputs need to be ordered by address type
    pub inputs_data: Vec<InputSigningData>,
    /// Optional remainder output information
    pub remainder: Option<RemainderData>,
}

/// PreparedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedTransactionDataDto {
    /// Transaction
    pub transaction: TransactionDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
    /// Optional remainder output information
    pub remainder: Option<RemainderData>,
}

impl From<&PreparedTransactionData> for PreparedTransactionDataDto {
    fn from(value: &PreparedTransactionData) -> Self {
        Self {
            transaction: TransactionDto::from(&value.transaction),
            inputs_data: value.inputs_data.clone(),
            remainder: value.remainder.clone(),
        }
    }
}

impl TryFromDto<PreparedTransactionDataDto> for PreparedTransactionData {
    type Error = Error;

    fn try_from_dto_with_params_inner(
        dto: PreparedTransactionDataDto,
        params: Option<&ProtocolParameters>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction: Transaction::try_from_dto_with_params_inner(dto.transaction, params)
                .map_err(|_| Error::InvalidField("transaction"))?,
            inputs_data: dto.inputs_data,
            remainder: dto.remainder,
        })
    }
}

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedTransactionData {
    /// Signed transaction payload
    pub payload: SignedTransactionPayload,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
}

/// SignedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedTransactionDataDto {
    /// Signed transaction payload
    pub payload: SignedTransactionPayloadDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
}

impl From<&SignedTransactionData> for SignedTransactionDataDto {
    fn from(value: &SignedTransactionData) -> Self {
        Self {
            payload: SignedTransactionPayloadDto::from(&value.payload),
            inputs_data: value.inputs_data.clone(),
        }
    }
}

impl TryFromDto<SignedTransactionDataDto> for SignedTransactionData {
    type Error = Error;

    fn try_from_dto_with_params_inner(
        dto: SignedTransactionDataDto,
        params: Option<&ProtocolParameters>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            payload: SignedTransactionPayload::try_from_dto_with_params_inner(dto.payload, params)
                .map_err(|_| Error::InvalidField("transaction_payload"))?,
            inputs_data: dto.inputs_data,
        })
    }
}

/// Data for a remainder output, used for ledger nano
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RemainderData {
    /// The remainder output
    pub output: Output,
    /// The chain derived from seed, for the remainder addresses
    #[serde(with = "option_bip44", default)]
    pub chain: Option<Bip44>,
    /// The remainder address
    pub address: Address,
}
