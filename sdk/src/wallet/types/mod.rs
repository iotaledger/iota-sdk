// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Address types used in the wallet
pub(crate) mod address;
pub(crate) mod balance;
#[cfg(feature = "participation")]
pub mod participation;

use std::str::FromStr;

use crypto::keys::bip44::Bip44;
use serde::{Deserialize, Serialize};

pub use self::balance::{Balance, BaseCoinBalance, NativeTokensBalance, RequiredStorageDeposit};
use crate::{
    client::{secret::types::InputSigningData, ClientError},
    types::{
        block::{
            address::Bech32Address,
            output::{Output, OutputId, OutputIdProof, OutputMetadata, OutputWithMetadata},
            payload::{
                signed_transaction::{dto::SignedTransactionPayloadDto, SignedTransactionPayload, TransactionId},
                PayloadError,
            },
            protocol::{CommittableAgeRange, ProtocolParameters},
            slot::SlotIndex,
            BlockId,
        },
        TryFromDto,
    },
    wallet::WalletError,
};

/// An output with additional data.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    /// The output itself.
    pub output: Output,
    /// The metadata of the output.
    pub metadata: OutputMetadata,
    /// The output ID proof.
    pub output_id_proof: OutputIdProof,
    /// The corresponding output ID.
    pub output_id: OutputId,
    /// The network ID the output belongs to.
    pub network_id: u64,
    /// Whether the output represents a remainder amount.
    pub remainder: bool,
}

impl OutputData {
    /// Returns whether the [`OutputMetadata`] is spent or not.
    pub fn is_spent(&self) -> bool {
        self.metadata.is_spent()
    }

    pub fn input_signing_data(
        &self,
        wallet_address: &Bech32Address,
        wallet_bip_path: Option<Bip44>,
        commitment_slot_index: impl Into<SlotIndex>,
        committable_age_range: CommittableAgeRange,
    ) -> Result<Option<InputSigningData>, WalletError> {
        let required_address = self
            .output
            .required_address(commitment_slot_index.into(), committable_age_range)?
            .ok_or(ClientError::ExpirationDeadzone)?;

        let chain = if let Some(required_ed25519) = required_address.backing_ed25519() {
            if let Some(backing_ed25519) = wallet_address.inner().backing_ed25519() {
                if required_ed25519 == backing_ed25519 {
                    wallet_bip_path
                } else {
                    // Different ed25519 chain than the wallet one.
                    None
                }
            } else {
                // Address can't be unlocked, wallet is not ed25519-based.
                return Ok(None);
            }
        } else {
            // Non-chain based address.
            None
        };

        Ok(Some(InputSigningData {
            output: self.output.clone(),
            output_metadata: self.metadata,
            chain,
        }))
    }
}

/// A transaction with metadata
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionWithMetadata {
    pub payload: SignedTransactionPayload,
    pub block_id: Option<BlockId>,
    pub inclusion_state: InclusionState,
    pub transaction_id: TransactionId,
    // network id to ignore outputs when set_client_options is used to switch to another network
    pub network_id: u64,
    // set if the transaction was created by the wallet or if it was sent by someone else and is incoming
    pub incoming: bool,
    pub note: Option<String>,
    /// Outputs that are used as input in the transaction. May not be all, because some may have already been deleted
    /// from the node.
    // serde(default) is needed so it doesn't break with old dbs
    pub inputs: Vec<OutputWithMetadata>,
}

/// Dto for a transaction with metadata
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionWithMetadataDto {
    /// The transaction payload
    pub payload: SignedTransactionPayloadDto,
    /// BlockId when it got sent to the Tangle
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    /// Inclusion state of the transaction
    pub inclusion_state: InclusionState,
    pub transaction_id: TransactionId,
    /// Network id to ignore outputs when set_client_options is used to switch to another network
    pub network_id: String,
    /// If the transaction was created by the wallet or if it was sent by someone else and is incoming
    pub incoming: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub inputs: Vec<OutputWithMetadata>,
}

impl From<&TransactionWithMetadata> for TransactionWithMetadataDto {
    fn from(value: &TransactionWithMetadata) -> Self {
        Self {
            payload: SignedTransactionPayloadDto::from(&value.payload),
            block_id: value.block_id,
            inclusion_state: value.inclusion_state,
            transaction_id: value.transaction_id,
            network_id: value.network_id.to_string(),
            incoming: value.incoming,
            note: value.note.clone(),
            inputs: value.inputs.clone(),
        }
    }
}

impl TryFromDto<TransactionWithMetadataDto> for TransactionWithMetadata {
    type Error = PayloadError;

    fn try_from_dto_with_params_inner(
        dto: TransactionWithMetadataDto,
        params: Option<&ProtocolParameters>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            payload: SignedTransactionPayload::try_from_dto_with_params_inner(dto.payload, params)?,
            block_id: dto.block_id,
            inclusion_state: dto.inclusion_state,
            transaction_id: dto.transaction_id,
            network_id: dto
                .network_id
                .parse::<u64>()
                .map_err(|e| PayloadError::NetworkId(e.to_string()))?,
            incoming: dto.incoming,
            note: dto.note,
            inputs: dto.inputs,
        })
    }
}

impl Serialize for TransactionWithMetadata {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        TransactionWithMetadataDto::from(self).serialize(serializer)
    }
}

/// Possible InclusionStates for transactions
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InclusionState {
    Pending,
    Accepted,
    Confirmed,
    Finalized,
    Conflicting,
    UnknownPruned,
}

/// The output kind enum.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OutputKind {
    /// Account output.
    Account,
    /// Basic output.
    Basic,
    /// Foundry output.
    Foundry,
    /// Nft output.
    Nft,
}

impl FromStr for OutputKind {
    type Err = WalletError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "Account" => Self::Account,
            "Basic" => Self::Basic,
            "Foundry" => Self::Foundry,
            "Nft" => Self::Nft,
            _ => return Err(WalletError::InvalidOutputKind(s.to_string())),
        };
        Ok(kind)
    }
}
