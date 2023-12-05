// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
};

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedTransactionData<O> {
    /// Transaction
    pub transaction: Transaction,
    /// Required input information for signing. Inputs need to be ordered by address type
    pub inputs_data: Vec<InputSigningData<O>>,
    /// Optional remainder output information
    pub remainder: Option<RemainderData<O>>,
}

/// PreparedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedTransactionDataDto<O> {
    /// Transaction
    pub transaction: TransactionDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData<O>>,
    /// Optional remainder output information
    pub remainder: Option<RemainderData<O>>,
}

impl<O: Clone> From<&PreparedTransactionData<O>> for PreparedTransactionDataDto<O> {
    fn from(value: &PreparedTransactionData<O>) -> Self {
        Self {
            transaction: TransactionDto::from(&value.transaction),
            inputs_data: value.inputs_data.clone(),
            remainder: value.remainder.clone(),
        }
    }
}

impl<O> TryFromDto<PreparedTransactionDataDto<O>> for PreparedTransactionData<O> {
    type Error = Error;

    fn try_from_dto_with_params_inner(
        dto: PreparedTransactionDataDto<O>,
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
pub struct SignedTransactionData<O> {
    /// Signed transaction payload
    pub payload: SignedTransactionPayload,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData<O>>,
}

/// SignedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedTransactionDataDto<O> {
    /// Signed transaction payload
    pub payload: SignedTransactionPayloadDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData<O>>,
}

impl<O: Clone> From<&SignedTransactionData<O>> for SignedTransactionDataDto<O> {
    fn from(value: &SignedTransactionData<O>) -> Self {
        Self {
            payload: SignedTransactionPayloadDto::from(&value.payload),
            inputs_data: value.inputs_data.clone(),
        }
    }
}

impl<O> TryFromDto<SignedTransactionDataDto<O>> for SignedTransactionData<O> {
    type Error = Error;

    fn try_from_dto_with_params_inner(
        dto: SignedTransactionDataDto<O>,
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
pub struct RemainderData<O> {
    /// The remainder output
    pub output: Output,
    /// The signing options for the remainder addresses
    pub signing_options: Option<O>,
    /// The remainder address
    pub address: Address,
}
