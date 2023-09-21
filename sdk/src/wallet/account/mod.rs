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

use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::Arc,
};

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};

#[cfg(feature = "participation")]
pub use self::operations::participation::{AccountParticipationOverview, ParticipationEventWithNodes};
use self::types::{
    address::{AddressWithUnspentOutputs, Bip44Address},
    Balance, OutputData, Transaction, TransactionDto,
};
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
use super::{core::WalletInner, Wallet};
use crate::{
    client::{
        secret::{SecretManage, SecretManager},
        Client,
    },
    types::{
        api::core::response::OutputWithMetadataResponse,
        block::{
            output::{dto::FoundryOutputDto, AccountId, FoundryId, FoundryOutput, NftId, Output, OutputId, TokenId},
            payload::{transaction::TransactionId, TransactionPayload},
        },
        TryFromDto,
    },
    wallet::{account::types::InclusionState, Result},
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

// TODO: remove this down below!
// /// A thread guard over an account, so we can lock the account during operations.
// #[derive(Debug)]
// pub struct Account<S: SecretManage = SecretManager> {
//     inner: Arc<AccountInner>,
//     pub(crate) wallet: Arc<WalletInner<S>>,
// }

// impl<S: SecretManage> Clone for Account<S> {
//     fn clone(&self) -> Self {
//         Self {
//             inner: self.inner.clone(),
//             wallet: self.wallet.clone(),
//         }
//     }
// }

// impl<S: SecretManage> Account<S> {
//     pub fn get_secret_manager(&self) -> &Arc<RwLock<S>> {
//         self.wallet.get_secret_manager()
//     }
// }

// #[derive(Debug)]
// pub struct AccountInner {
//     details: RwLock<WalletData>,
//     // mutex to prevent multiple sync calls at the same or almost the same time, the u128 is a timestamp
//     // if the last synced time was < `MIN_SYNC_INTERVAL` second ago, we don't sync, but only calculate the balance
//     // again, because sending transactions can change that
//     pub(crate) last_synced: Mutex<u128>,
//     pub(crate) default_sync_options: Mutex<SyncOptions>,
// }

// // impl Deref so we can use `account.details()` instead of `account.details.read()`
// impl<S: SecretManage> Deref for Account<S> {
//     type Target = AccountInner;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

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
