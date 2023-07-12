// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Address types used in the account
pub(crate) mod address;
pub(crate) mod balance;
#[cfg(feature = "participation")]
pub mod participation;

use std::str::FromStr;

use crypto::keys::bip44::Bip44;
use serde::{Deserialize, Deserializer, Serialize};

pub use self::{
    address::{AccountAddress, AddressWithUnspentOutputs},
    balance::{Balance, BaseCoinBalance, NativeTokensBalance, RequiredStorageDeposit},
};
use crate::{
    client::secret::types::InputSigningData,
    types::{
        api::core::response::OutputWithMetadataResponse,
        block::{
            address::{dto::AddressDto, Address},
            output::{
                dto::{OutputDto, OutputMetadataDto},
                AliasTransition, Output, OutputId, OutputMetadata,
            },
            payload::transaction::{dto::TransactionPayloadDto, TransactionId, TransactionPayload},
            BlockId,
        },
    },
    utils::serde::bip44::option_bip44,
    wallet::account::AccountDetails,
};

/// An output with metadata
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    /// The output id
    pub output_id: OutputId,
    pub metadata: OutputMetadata,
    /// The actual Output
    pub output: Output,
    /// If an output is spent
    pub is_spent: bool,
    /// Associated account address.
    pub address: Address,
    /// Network ID
    pub network_id: u64,
    pub remainder: bool,
    // bip44 path
    #[serde(with = "option_bip44")]
    pub chain: Option<Bip44>,
}

impl OutputData {
    pub fn input_signing_data(
        &self,
        account: &AccountDetails,
        current_time: u32,
        alias_transition: Option<AliasTransition>,
    ) -> crate::wallet::Result<Option<InputSigningData>> {
        let (unlock_address, _unlocked_alias_or_nft_address) =
            self.output
                .required_and_unlocked_address(current_time, &self.output_id, alias_transition)?;

        let chain = if unlock_address == self.address {
            self.chain
        } else if let Address::Ed25519(_) = unlock_address {
            if let Some(address) = account
                .addresses_with_unspent_outputs
                .iter()
                .find(|a| a.address.inner == unlock_address)
            {
                Some(
                    Bip44::new()
                        .with_coin_type(account.coin_type)
                        .with_account(account.index)
                        .with_change(address.internal as _)
                        .with_address_index(address.key_index),
                )
            } else {
                return Ok(None);
            }
        } else {
            // Alias and NFT addresses have no chain
            None
        };

        Ok(Some(InputSigningData {
            output: self.output.clone(),
            output_metadata: self.metadata.clone(),
            chain,
        }))
    }
}

/// Dto for an output with metadata
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputDataDto {
    /// The output id
    pub output_id: OutputId,
    /// The metadata of the output
    pub metadata: OutputMetadataDto,
    /// The actual Output
    pub output: OutputDto,
    /// If an output is spent
    pub is_spent: bool,
    /// Associated account address.
    pub address: AddressDto,
    /// Network ID
    pub network_id: String,
    /// Remainder
    pub remainder: bool,
    /// Bip32 path
    pub chain: Option<Bip44>,
}

impl From<&OutputData> for OutputDataDto {
    fn from(value: &OutputData) -> Self {
        Self {
            output_id: value.output_id,
            metadata: OutputMetadataDto::from(&value.metadata),
            output: OutputDto::from(&value.output),
            is_spent: value.is_spent,
            address: AddressDto::from(&value.address),
            network_id: value.network_id.to_string(),
            remainder: value.remainder,
            chain: value.chain,
        }
    }
}

/// A transaction with metadata
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub payload: TransactionPayload,
    pub block_id: Option<BlockId>,
    pub inclusion_state: InclusionState,
    // Transaction creation time
    pub timestamp: u128,
    pub transaction_id: TransactionId,
    // network id to ignore outputs when set_client_options is used to switch to another network
    pub network_id: u64,
    // set if the transaction was created by the wallet or if it was sent by someone else and is incoming
    pub incoming: bool,
    pub note: Option<String>,
    /// Outputs that are used as input in the transaction. May not be all, because some may have already been deleted
    /// from the node.
    // serde(default) is needed so it doesn't break with old dbs
    #[serde(default)]
    pub inputs: Vec<OutputWithMetadataResponse>,
}

/// Dto for a transaction with metadata
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDto {
    /// The transaction payload
    pub payload: TransactionPayloadDto,
    /// BlockId when it got sent to the Tangle
    pub block_id: Option<BlockId>,
    /// Inclusion state of the transaction
    pub inclusion_state: InclusionState,
    /// Timestamp
    pub timestamp: String,
    pub transaction_id: TransactionId,
    /// Network id to ignore outputs when set_client_options is used to switch to another network
    pub network_id: String,
    /// If the transaction was created by the wallet or if it was sent by someone else and is incoming
    pub incoming: bool,
    pub note: Option<String>,
    pub inputs: Vec<OutputWithMetadataResponse>,
}

impl From<&Transaction> for TransactionDto {
    fn from(value: &Transaction) -> Self {
        Self {
            payload: TransactionPayloadDto::from(&value.payload),
            block_id: value.block_id,
            inclusion_state: value.inclusion_state,
            timestamp: value.timestamp.to_string(),
            transaction_id: value.transaction_id,
            network_id: value.network_id.to_string(),
            incoming: value.incoming,
            note: value.note.clone(),
            inputs: value.inputs.clone(),
        }
    }
}

/// Possible InclusionStates for transactions
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum InclusionState {
    Pending,
    Confirmed,
    Conflicting,
    UnknownPruned,
}

/// The output kind enum.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OutputKind {
    /// Alias output.
    Alias,
    /// Basic output.
    Basic,
    /// Foundry output.
    Foundry,
    /// Nft output.
    Nft,
    /// Treasury output.
    Treasury,
}

impl FromStr for OutputKind {
    type Err = crate::wallet::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "Alias" => Self::Alias,
            "Basic" => Self::Basic,
            "Foundry" => Self::Foundry,
            "Nft" => Self::Nft,
            "Treasury" => Self::Treasury,
            _ => return Err(crate::wallet::Error::InvalidOutputKind(s.to_string())),
        };
        Ok(kind)
    }
}

/// The account identifier.
#[derive(Debug, Clone, Serialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum AccountIdentifier {
    /// Account alias as identifier.
    Alias(String),
    /// An index identifier.
    Index(u32),
}

// Custom deserialize because the index could also be encoded as String
impl<'de> Deserialize<'de> for AccountIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;
        let v = Value::deserialize(deserializer)?;
        Ok(match v.as_u64() {
            Some(number) => {
                let index: u32 =
                    u32::try_from(number).map_err(|_| D::Error::custom("account index is greater than u32::MAX"))?;
                Self::Index(index)
            }
            None => {
                let alias_or_index_str = v
                    .as_str()
                    .ok_or_else(|| D::Error::custom("accountIdentifier is no number or string"))?;
                Self::from(alias_or_index_str)
            }
        })
    }
}

// When the identifier is a string.
impl From<&str> for AccountIdentifier {
    fn from(value: &str) -> Self {
        u32::from_str(value).map_or_else(|_| Self::Alias(value.to_string()), Self::Index)
    }
}

impl From<String> for AccountIdentifier {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&String> for AccountIdentifier {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

// When the identifier is an index.
impl From<u32> for AccountIdentifier {
    fn from(value: u32) -> Self {
        Self::Index(value)
    }
}
