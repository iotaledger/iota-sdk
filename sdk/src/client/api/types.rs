// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::collections::BTreeMap;

use crypto::keys::bip44::Bip44;
use serde::{Deserialize, Serialize};

use crate::{
    client::secret::types::InputSigningData,
    types::{
        block::{
            address::Address,
            output::{AccountId, Output, OutputId},
            payload::{
                signed_transaction::{
                    dto::{SignedTransactionPayloadDto, TransactionDto},
                    Transaction,
                },
                PayloadError, SignedTransactionPayload,
            },
            protocol::ProtocolParameters,
        },
        TryFromDto,
    },
    utils::serde::{bip44::option_bip44, mana_rewards},
};

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedTransactionData {
    /// Transaction
    pub transaction: Transaction,
    /// Required input information for signing. Inputs need to be ordered by address type
    pub inputs_data: Vec<InputSigningData>,
    /// Remainder outputs information
    pub remainders: Vec<RemainderData>,
    /// Mana rewards
    pub mana_rewards: BTreeMap<OutputId, u64>,
    /// The block issuer id from which the BIC for the block containing this transaction should be burned.
    pub issuer_id: Option<AccountId>,
}

/// PreparedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedTransactionDataDto {
    /// Transaction
    pub transaction: TransactionDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
    /// Remainder outputs information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remainders: Vec<RemainderData>,
    /// Mana rewards
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty", with = "mana_rewards")]
    pub mana_rewards: BTreeMap<OutputId, u64>,
    /// The block issuer id from which the BIC for the block containing this transaction should be burned.
    pub issuer_id: Option<AccountId>,
}

impl From<&PreparedTransactionData> for PreparedTransactionDataDto {
    fn from(value: &PreparedTransactionData) -> Self {
        Self {
            transaction: TransactionDto::from(&value.transaction),
            inputs_data: value.inputs_data.clone(),
            remainders: value.remainders.clone(),
            mana_rewards: value.mana_rewards.clone(),
            issuer_id: value.issuer_id,
        }
    }
}

impl TryFromDto<PreparedTransactionDataDto> for PreparedTransactionData {
    type Error = PayloadError;

    fn try_from_dto_with_params_inner(
        dto: PreparedTransactionDataDto,
        params: Option<&ProtocolParameters>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction: Transaction::try_from_dto_with_params_inner(dto.transaction, params)?,
            inputs_data: dto.inputs_data,
            remainders: dto.remainders,
            mana_rewards: dto.mana_rewards,
            issuer_id: dto.issuer_id,
        })
    }
}

impl Serialize for PreparedTransactionData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        PreparedTransactionDataDto::from(self).serialize(serializer)
    }
}

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedTransactionData {
    /// Signed transaction payload
    pub payload: SignedTransactionPayload,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
    /// Mana rewards
    pub mana_rewards: BTreeMap<OutputId, u64>,
    /// The block issuer id from which the BIC for the block containing this transaction should be burned.
    pub issuer_id: Option<AccountId>,
}

/// SignedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedTransactionDataDto {
    /// Signed transaction payload
    pub payload: SignedTransactionPayloadDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
    /// Mana rewards
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty", with = "mana_rewards")]
    pub mana_rewards: BTreeMap<OutputId, u64>,
    /// The block issuer id from which the BIC for the block containing this transaction should be burned.
    pub issuer_id: Option<AccountId>,
}

impl From<&SignedTransactionData> for SignedTransactionDataDto {
    fn from(value: &SignedTransactionData) -> Self {
        Self {
            payload: SignedTransactionPayloadDto::from(&value.payload),
            inputs_data: value.inputs_data.clone(),
            mana_rewards: value.mana_rewards.clone(),
            issuer_id: value.issuer_id,
        }
    }
}

impl TryFromDto<SignedTransactionDataDto> for SignedTransactionData {
    type Error = PayloadError;

    fn try_from_dto_with_params_inner(
        dto: SignedTransactionDataDto,
        params: Option<&ProtocolParameters>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            payload: SignedTransactionPayload::try_from_dto_with_params_inner(dto.payload, params)?,
            inputs_data: dto.inputs_data,
            mana_rewards: dto.mana_rewards,
            issuer_id: dto.issuer_id,
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
