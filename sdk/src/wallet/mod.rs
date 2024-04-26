// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The IOTA Wallet Library

/// Constants used for the wallet and wallet operations.
pub(crate) mod constants;
/// The core module.
pub mod core;
#[cfg(any(feature = "stronghold", feature = "storage"))]
pub mod migration;
/// The wallet operations like address generation, syncing and creating transactions.
pub(crate) mod operations;
/// Types used in a wallet and returned from methods.
pub mod types;
/// Methods to update the wallet state.
pub(crate) mod update;

/// The ClientOptions to build the iota_client for interactions with the IOTA Tangle.
pub use crate::client::ClientBuilder as ClientOptions;

/// The error module.
pub mod error;
/// The event module.
#[cfg(feature = "events")]
#[cfg_attr(docsrs, doc(cfg(feature = "events")))]
pub mod events;
/// The storage module.
#[cfg(feature = "storage")]
#[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
pub mod storage;
/// The module for spawning tasks on a thread
pub(crate) mod task;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[cfg(feature = "participation")]
pub use self::operations::participation::{ParticipationEventWithNodes, ParticipationOverview};
use self::types::TransactionWithMetadata;
pub use self::{
    core::{Wallet, WalletBuilder},
    error::WalletError,
    operations::{
        output_claiming::OutputsToClaim,
        output_consolidation::ConsolidationParams,
        syncing::{
            options::{AccountSyncOptions, NftSyncOptions, WalletSyncOptions},
            SyncOptions,
        },
        transaction::{
            high_level::{
                account_block_issuer_keys::ModifyAccountBlockIssuerKey,
                create_account::CreateAccountParams,
                delegation::create::{
                    CreateDelegationParams, CreateDelegationTransaction, PreparedCreateDelegationTransaction,
                },
                minting::{
                    create_native_token::{
                        CreateNativeTokenParams, CreateNativeTokenTransaction, PreparedCreateNativeTokenTransaction,
                    },
                    mint_nfts::MintNftParams,
                },
                send::SendParams,
                send_mana::SendManaParams,
                send_native_tokens::SendNativeTokenParams,
                send_nft::SendNftParams,
                staking::begin::BeginStakingParams,
            },
            prepare_output::{Assets, Features, OutputParams, ReturnStrategy, StorageDeposit, Unlocks},
        },
    },
    types::OutputData,
};
use crate::{
    types::{
        api::core::OutputWithMetadataResponse,
        block::{
            output::{AccountId, AnchorId, DelegationId, FoundryId, NftId, OutputWithMetadata},
            payload::signed_transaction::{SignedTransactionPayload, TransactionId},
            slot::SlotIndex,
        },
    },
    wallet::types::InclusionState,
};

/// Options to filter outputs
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FilterOptions {
    /// Include all outputs where the included slot is below the specified slot.
    pub included_below_slot: Option<SlotIndex>,
    /// Include all outputs where the included slot is above the specified slot.
    pub included_above_slot: Option<SlotIndex>,
    /// Filter all outputs for the provided types (Basic = 3, Account = 4, Foundry = 5, NFT = 6).
    pub output_types: Option<Vec<u8>>,
    /// Return all account outputs matching these IDs.
    pub account_ids: Option<HashSet<AccountId>>,
    /// Return all anchor outputs matching these IDs.
    pub anchor_ids: Option<HashSet<AnchorId>>,
    /// Return all foundry outputs matching these IDs.
    pub foundry_ids: Option<HashSet<FoundryId>>,
    /// Return all nft outputs matching these IDs.
    pub nft_ids: Option<HashSet<NftId>>,
    /// Return all delegation outputs matching these IDs.
    pub delegation_ids: Option<HashSet<DelegationId>>,
}

pub(crate) fn build_transaction_from_payload_and_inputs(
    tx_id: TransactionId,
    tx_payload: SignedTransactionPayload,
    inputs: Vec<OutputWithMetadataResponse>,
) -> Result<TransactionWithMetadata, WalletError> {
    Ok(TransactionWithMetadata {
        payload: tx_payload.clone(),
        block_id: inputs.first().map(|i| *i.metadata.block_id()),
        inclusion_state: InclusionState::Confirmed,
        transaction_id: tx_id,
        network_id: tx_payload.transaction().network_id(),
        incoming: true,
        note: None,
        inputs: inputs
            .into_iter()
            .map(|input| OutputWithMetadata {
                output: input.output,
                metadata: input.metadata,
            })
            .collect(),
    })
}
