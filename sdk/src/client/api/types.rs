// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use serde::{Deserialize, Serialize};

use crate::{
    client::secret::types::{InputSigningData, InputSigningDataDto},
    types::{
        block::{
            address::Address,
            output::{dto::OutputDto, Output},
            payload::{
                signed_transaction::{
                    dto::{SignedTransactionPayloadDto, TransactionDto},
                    Transaction,
                },
                SignedTransactionPayload,
            },
            Error,
        },
        TryFromDto, ValidationParams,
    },
    utils::serde::bip44::option_bip44,
};

// TODO remove all essence occurrences

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
    pub inputs_data: Vec<InputSigningDataDto>,
    /// Optional remainder output information
    pub remainder: Option<RemainderDataDto>,
}

impl From<&PreparedTransactionData> for PreparedTransactionDataDto {
    fn from(value: &PreparedTransactionData) -> Self {
        Self {
            transaction: TransactionDto::from(&value.transaction),
            inputs_data: value.inputs_data.iter().map(InputSigningDataDto::from).collect(),
            remainder: value.remainder.as_ref().map(RemainderDataDto::from),
        }
    }
}

impl TryFromDto for PreparedTransactionData {
    type Dto = PreparedTransactionDataDto;
    type Error = Error;

    fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction: Transaction::try_from_dto_with_params(dto.transaction, &params)
                .map_err(|_| Error::InvalidField("transaction"))?,
            inputs_data: dto
                .inputs_data
                .into_iter()
                .map(|i| InputSigningData::try_from_dto_with_params(i, &params))
                .collect::<crate::client::Result<Vec<InputSigningData>>>()
                .map_err(|_| Error::InvalidField("input_data"))?,
            remainder: match dto.remainder {
                Some(remainder) => Some(
                    RemainderData::try_from_dto_with_params(remainder, &params)
                        .map_err(|_| Error::InvalidField("remainder"))?,
                ),
                None => None,
            },
        })
    }
}

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedTransactionData {
    /// Signed transaction payload
    pub transaction_payload: SignedTransactionPayload,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
}

/// SignedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedTransactionDataDto {
    /// Transaction essence
    pub transaction_payload: SignedTransactionPayloadDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningDataDto>,
}

impl From<&SignedTransactionData> for SignedTransactionDataDto {
    fn from(value: &SignedTransactionData) -> Self {
        Self {
            transaction_payload: SignedTransactionPayloadDto::from(&value.transaction_payload),
            inputs_data: value.inputs_data.iter().map(InputSigningDataDto::from).collect(),
        }
    }
}

impl TryFromDto for SignedTransactionData {
    type Dto = SignedTransactionDataDto;
    type Error = Error;

    fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_payload: SignedTransactionPayload::try_from_dto_with_params(dto.transaction_payload, &params)
                .map_err(|_| Error::InvalidField("transaction_payload"))?,
            inputs_data: dto
                .inputs_data
                .into_iter()
                .map(|i| InputSigningData::try_from_dto_with_params(i, &params))
                .collect::<crate::client::Result<Vec<InputSigningData>>>()
                .map_err(|_| Error::InvalidField("inputs_data"))?,
        })
    }
}

/// Data for a remainder output, used for ledger nano
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemainderData {
    /// The remainder output
    pub output: Output,
    /// The chain derived from seed, for the remainder addresses
    pub chain: Option<Bip44>,
    /// The remainder address
    pub address: Address,
}

/// Data for a remainder output, used for ledger nano
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RemainderDataDto {
    /// The remainder output
    pub output: OutputDto,
    /// The chain derived from seed, for the remainder addresses
    #[serde(with = "option_bip44", default)]
    pub chain: Option<Bip44>,
    /// The remainder address
    pub address: Address,
}

impl TryFromDto for RemainderData {
    type Dto = RemainderDataDto;
    type Error = Error;

    fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            output: Output::try_from_dto_with_params_inner(dto.output, params)?,
            chain: dto.chain,
            address: dto.address,
        })
    }
}
impl From<&RemainderData> for RemainderDataDto {
    fn from(remainder: &RemainderData) -> Self {
        Self {
            output: OutputDto::from(&remainder.output),
            chain: remainder.chain,
            address: remainder.address.clone(),
        }
    }
}
