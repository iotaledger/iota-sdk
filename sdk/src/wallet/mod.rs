// Copyright 2021 IOTA Stiftung
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
    error::Error,
    operations::{
        output_claiming::OutputsToClaim,
        output_consolidation::ConsolidationParams,
        syncing::{
            options::{WalletSyncOptions, AliasSyncOptions, NftSyncOptions},
            SyncOptions,
        },
        transaction::{
            high_level::{
                create_account::CreateAccountParams,
                minting::{
                    create_native_token::{
                        CreateNativeTokenParams, CreateNativeTokenTransactionDto,
                        PreparedCreateNativeTokenTransactionDto,
                    },
                    mint_nfts::MintNftParams,
                },
                send::SendParams,
                send_native_tokens::SendNativeTokensParams,
                send_nft::SendNftParams,
            },
            prepare_output::{Assets, Features, OutputParams, ReturnStrategy, StorageDeposit, Unlocks},
            RemainderValueStrategy, TransactionOptions,
        },
    },
    types::OutputDataDto,
};
use crate::{
    types::{
        api::core::OutputWithMetadataResponse,
        block::{
            output::{AccountId, FoundryId, NftId},
            payload::signed_transaction::{SignedTransactionPayload, TransactionId},
        },
    },
    wallet::types::InclusionState,
};

/// The wallet Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Options to filter outputs
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FilterOptions {
    /// Filter all outputs where the booked milestone index is below the specified timestamp
    pub lower_bound_booked_timestamp: Option<u32>,
    /// Filter all outputs where the booked milestone index is above the specified timestamp
    pub upper_bound_booked_timestamp: Option<u32>,
    /// Filter all outputs for the provided types (Basic = 3, Account = 4, Foundry = 5, NFT = 6).
    pub output_types: Option<Vec<u8>>,
    /// Return all account outputs matching these IDs.
    pub account_ids: Option<HashSet<AccountId>>,
    /// Return all foundry outputs matching these IDs.
    pub foundry_ids: Option<HashSet<FoundryId>>,
    /// Return all nft outputs matching these IDs.
    pub nft_ids: Option<HashSet<NftId>>,
}

pub(crate) fn build_transaction_from_payload_and_inputs(
    tx_id: TransactionId,
    tx_payload: SignedTransactionPayload,
    inputs: Vec<OutputWithMetadataResponse>,
) -> crate::wallet::Result<TransactionWithMetadata> {
    Ok(TransactionWithMetadata {
        payload: tx_payload.clone(),
        block_id: inputs.first().map(|i| *i.metadata.block_id()),
        inclusion_state: InclusionState::Confirmed,
        timestamp: 0,
        // TODO use slot index since milestone_timestamp_spent is gone
        // inputs
        //     .first()
        //     .and_then(|i| i.metadata.milestone_timestamp_spent.map(|t| t as u128 * 1000))
        //     .unwrap_or_else(|| crate::utils::unix_timestamp_now().as_millis()),
        transaction_id: tx_id,
        network_id: tx_payload.transaction().network_id(),
        incoming: true,
        note: None,
        inputs,
    })
}
