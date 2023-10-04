// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Constants used for the wallet and wallet operations.
pub(crate) mod constants;
/// The wallet operations like address generation, syncing and creating transactions.
pub(crate) mod operations;
/// Types used in a wallet and returned from methods.
pub mod types;
/// Methods to update the wallet state.
pub(crate) mod update;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[cfg(feature = "participation")]
pub use self::operations::participation::{AccountParticipationOverview, ParticipationEventWithNodes};
use self::types::{address::AddressWithUnspentOutputs, Balance, OutputData, Transaction};
pub use self::{
    operations::{
        output_claiming::OutputsToClaim,
        output_consolidation::ConsolidationParams,
        syncing::{
            options::{AccountSyncOptions, AliasSyncOptions, NftSyncOptions},
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
            },
            prepare_output::{Assets, Features, OutputParams, ReturnStrategy, StorageDeposit, Unlocks},
            RemainderValueStrategy, TransactionOptions,
        },
    },
    types::OutputDataDto,
};
use crate::{
    types::{
        api::core::response::OutputWithMetadataResponse,
        block::{
            output::{AccountId, FoundryId, NftId},
            payload::{transaction::TransactionId, TransactionPayload},
        },
    },
    wallet::account::types::InclusionState,
};

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
    tx_payload: TransactionPayload,
    inputs: Vec<OutputWithMetadataResponse>,
) -> crate::wallet::Result<Transaction> {
    Ok(Transaction {
        payload: tx_payload.clone(),
        block_id: inputs.first().map(|i| *i.metadata.block_id()),
        inclusion_state: InclusionState::Confirmed,
        timestamp: 0,
        // TODO check if we keep a timestamp in Transaction since milestone_timestamp_spent is gone
        // inputs
        //     .first()
        //     .and_then(|i| i.metadata.milestone_timestamp_spent.map(|t| t as u128 * 1000))
        //     .unwrap_or_else(|| crate::utils::unix_timestamp_now().as_millis()),
        transaction_id: tx_id,
        network_id: tx_payload.essence().network_id(),
        incoming: true,
        note: None,
        inputs,
    })
}
