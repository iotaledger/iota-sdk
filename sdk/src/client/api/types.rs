// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use serde::{Deserialize, Serialize};

use crate::{
    client::secret::types::{InputSigningData, InputSigningDataDto},
    types::block::{
        address::{dto::AddressDto, Address},
        output::{dto::OutputDto, Output},
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
    utils::serde::bip44::option_bip44,
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
    pub inputs_data: Vec<InputSigningDataDto>,
    /// Optional remainder output information
    pub remainder: Option<RemainderDataDto>,
}

impl From<&PreparedTransactionData> for PreparedTransactionDataDto {
    fn from(value: &PreparedTransactionData) -> Self {
        Self {
            essence: TransactionEssenceDto::from(&value.essence),
            inputs_data: value.inputs_data.iter().map(InputSigningDataDto::from).collect(),
            remainder: value.remainder.as_ref().map(RemainderDataDto::from),
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
            inputs_data: value
                .inputs_data
                .into_iter()
                .map(|i| InputSigningData::try_from_dto(i, protocol_parameters.token_supply()))
                .collect::<crate::client::Result<Vec<InputSigningData>>>()
                .map_err(|_| Error::InvalidField("input_data"))?,
            remainder: match value.remainder {
                Some(remainder) => Some(
                    RemainderData::try_from_dto(remainder, protocol_parameters.token_supply())
                        .map_err(|_| Error::InvalidField("remainder"))?,
                ),
                None => None,
            },
        })
    }

    /// Unverified conversion from [`PreparedTransactionDataDto`] to [`PreparedTransactionData`].
    pub fn try_from_dto_unverified(value: PreparedTransactionDataDto) -> Result<Self, Error> {
        Ok(Self {
            essence: TransactionEssence::try_from_dto_unverified(value.essence)
                .map_err(|_| Error::InvalidField("essence"))?,
            inputs_data: value
                .inputs_data
                .into_iter()
                .map(InputSigningData::try_from_dto_unverified)
                .collect::<crate::client::Result<Vec<InputSigningData>>>()
                .map_err(|_| Error::InvalidField("inputs_data"))?,
            remainder: match value.remainder {
                Some(remainder) => Some(
                    RemainderData::try_from_dto_unverified(remainder).map_err(|_| Error::InvalidField("remainder"))?,
                ),
                None => None,
            },
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
    pub inputs_data: Vec<InputSigningDataDto>,
}

impl From<&SignedTransactionData> for SignedTransactionDataDto {
    fn from(value: &SignedTransactionData) -> Self {
        Self {
            transaction_payload: TransactionPayloadDto::from(&value.transaction_payload),
            inputs_data: value.inputs_data.iter().map(InputSigningDataDto::from).collect(),
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
            inputs_data: value
                .inputs_data
                .into_iter()
                .map(|i| InputSigningData::try_from_dto(i, protocol_parameters.token_supply()))
                .collect::<crate::client::Result<Vec<InputSigningData>>>()
                .map_err(|_| Error::InvalidField("input_data"))?,
        })
    }

    /// Unverified conversion from [`SignedTransactionDataDto`] to [`SignedTransactionData`].
    pub fn try_from_dto_unverified(value: SignedTransactionDataDto) -> Result<Self, Error> {
        Ok(Self {
            transaction_payload: TransactionPayload::try_from_dto_unverified(value.transaction_payload)
                .map_err(|_| Error::InvalidField("transaction_payload"))?,
            inputs_data: value
                .inputs_data
                .into_iter()
                .map(InputSigningData::try_from_dto_unverified)
                .collect::<crate::client::Result<Vec<InputSigningData>>>()
                .map_err(|_| Error::InvalidField("inputs_data"))?,
        })
    }
}

/// Data for a remainder output, used for ledger nano
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
    #[serde(with = "option_bip44")]
    pub chain: Option<Bip44>,
    /// The remainder address
    pub address: AddressDto,
}

impl RemainderData {
    pub(crate) fn try_from_dto(remainder: RemainderDataDto, token_supply: u64) -> crate::client::Result<Self> {
        Ok(Self {
            output: Output::try_from_dto(remainder.output, token_supply)?,
            chain: remainder.chain,
            address: Address::try_from(remainder.address)?,
        })
    }

    pub(crate) fn try_from_dto_unverified(remainder: RemainderDataDto) -> crate::client::Result<Self> {
        Ok(Self {
            output: Output::try_from_dto_unverified(remainder.output)?,
            chain: remainder.chain,
            address: Address::try_from(remainder.address)?,
        })
    }
}

impl From<&RemainderData> for RemainderDataDto {
    fn from(remainder: &RemainderData) -> Self {
        Self {
            output: OutputDto::from(&remainder.output),
            chain: remainder.chain,
            address: AddressDto::from(&remainder.address),
        }
    }
}
