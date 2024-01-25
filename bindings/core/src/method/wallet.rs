// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "stronghold")]
use std::path::PathBuf;

use crypto::keys::bip44::Bip44;
use derivative::Derivative;
#[cfg(feature = "events")]
use iota_sdk::wallet::events::types::{WalletEvent, WalletEventType};
// #[cfg(feature = "participation")]
// use iota_sdk::{
//     client::node_manager::node::Node,
//     types::api::plugins::participation::types::{ParticipationEventId, ParticipationEventType},
//     wallet::types::participation::ParticipationEventRegistrationOptions,
// };
use iota_sdk::{
    client::{
        api::{input_selection::Burn, PreparedTransactionDataDto, SignedTransactionDataDto},
        node_manager::node::NodeAuth,
        secret::GenerateAddressOptions,
    },
    types::block::{
        address::{Bech32Address, Hrp},
        output::{Output, OutputId, TokenId},
        payload::signed_transaction::TransactionId,
    },
    wallet::{
        ClientOptions, ConsolidationParams, CreateAccountParams, CreateNativeTokenParams, FilterOptions, MintNftParams,
        OutputParams, OutputsToClaim, SendNativeTokenParams, SendNftParams, SendParams, SyncOptions,
        TransactionOptions,
    },
    U256,
};
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(feature = "stronghold")]
use crate::OmittedDebug;

/// The methods that can be sent to the actor.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
#[non_exhaustive]
pub enum WalletMethod {
    /// Returns the accounts of the wallet.
    /// Expected response: [`OutputsData`](crate::Response::OutputsData)
    Accounts,
    /// Backup storage. Password must be the current one, when Stronghold is used as SecretManager.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    Backup {
        /// The backup destination.
        destination: PathBuf,
        /// Stronghold file password.
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        password: String,
    },
    /// Restore a backup from a Stronghold file
    /// Replaces client_options, coin_type, secret_manager and wallet. Returns an error if the wallet was already
    /// created If Stronghold is used as secret_manager, the existing Stronghold file will be overwritten. If a
    /// mnemonic was stored, it will be gone.
    /// if ignore_if_coin_type_mismatch.is_some(), client options will not be restored
    /// if ignore_if_coin_type_mismatch == Some(true), client options coin type and the wallet will not be restored if
    /// the cointype doesn't match
    /// If a bech32 hrp is provided to ignore_if_bech32_hrp_mismatch, that doesn't match the one of the current
    /// address, the wallet will not be restored. Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[serde(rename_all = "camelCase")]
    RestoreBackup {
        /// The path to the backed up Stronghold.
        source: PathBuf,
        /// Stronghold file password.
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        password: String,
        /// If ignore_if_coin_type_mismatch.is_some(), client options will not be restored.
        /// If ignore_if_coin_type_mismatch == Some(true), client options coin type and wallet will not be restored
        /// if the cointype doesn't match.
        ignore_if_coin_type_mismatch: Option<bool>,
        /// If a bech32 hrp is provided to ignore_if_bech32_hrp_mismatch, that doesn't match the one of the current
        /// address, the wallet will not be restored.
        ignore_if_bech32_mismatch: Option<Hrp>,
    },
    /// Updates the client options for the wallet.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[serde(rename_all = "camelCase")]
    SetClientOptions { client_options: Box<ClientOptions> },
    /// Start background syncing.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[serde(rename_all = "camelCase")]
    StartBackgroundSync {
        /// Sync options
        options: Option<SyncOptions>,
        /// Interval in milliseconds
        interval_in_milliseconds: Option<u64>,
    },
    /// Stop background syncing.
    /// Expected response: [`Ok`](crate::Response::Ok)
    StopBackgroundSync,
    // Remove all listeners of this type. Empty vec clears all listeners
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    #[serde(rename_all = "camelCase")]
    ClearListeners { event_types: Vec<WalletEventType> },
    /// Update the authentication for the provided node.
    /// Expected response: [`Ok`](crate::Response::Ok)
    UpdateNodeAuth {
        /// Node url
        url: Url,
        /// Authentication options
        auth: Option<NodeAuth>,
    },
    /// Get outputs with additional unlock conditions.
    /// Expected response: [`OutputIds`](crate::Response::OutputIds)
    #[serde(rename_all = "camelCase")]
    ClaimableOutputs { outputs_to_claim: OutputsToClaim },
    /// Claim outputs.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    ClaimOutputs { output_ids_to_claim: Vec<OutputId> },
    // /// Removes a previously registered participation event from local storage.
    // /// Expected response: [`Ok`](crate::Response::Ok)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // #[serde(rename_all = "camelCase")]
    // DeregisterParticipationEvent { event_id: ParticipationEventId },
    /// Get the wallet address.
    /// Expected response: [`Address`](crate::Response::Address)
    GetAddress,
    /// Get wallet balance information.
    /// Expected response: [`Balance`](crate::Response::Balance)
    GetBalance,
    /// Get the [`Output`] that minted a native token by its TokenId.
    /// Expected response: [`Output`](crate::Response::Output)
    #[serde(rename_all = "camelCase")]
    GetFoundryOutput { token_id: TokenId },
    /// Get the transaction with inputs of an incoming transaction stored in the wallet.
    /// List might not be complete, if the node pruned the data already.
    /// Expected response: [`Transaction`](crate::Response::Transaction)
    #[serde(rename_all = "camelCase")]
    GetIncomingTransaction { transaction_id: TransactionId },
    /// Get the [`OutputData`](iota_sdk::wallet::types::OutputData) of an output stored in the wallet.
    /// Expected response: [`OutputData`](crate::Response::OutputData)
    #[serde(rename_all = "camelCase")]
    GetOutput { output_id: OutputId },
    // /// Expected response: [`ParticipationEvent`](crate::Response::ParticipationEvent)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // #[serde(rename_all = "camelCase")]
    // GetParticipationEvent { event_id: ParticipationEventId },
    // /// Expected response: [`ParticipationEventIds`](crate::Response::ParticipationEventIds)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // #[serde(rename_all = "camelCase")]
    // GetParticipationEventIds {
    //     node: Node,
    //     event_type: Option<ParticipationEventType>,
    // },
    // /// Expected response:
    // /// [`ParticipationEventStatus`](crate::Response::ParticipationEventStatus)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // #[serde(rename_all = "camelCase")]
    // GetParticipationEventStatus { event_id: ParticipationEventId },
    // /// Expected response: [`ParticipationEvents`](crate::Response::ParticipationEvents)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // GetParticipationEvents,
    // /// Calculates a participation overview for the wallet. If event_ids are provided, only return outputs and
    // tracked /// participations for them.
    // /// Expected response: [`ParticipationOverview`](crate::Response::ParticipationOverview)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // #[serde(rename_all = "camelCase")]
    // GetParticipationOverview {
    //     event_ids: Option<Vec<ParticipationEventId>>,
    // },
    /// Get the [`TransactionWithMetadata`](iota_sdk::wallet::types::TransactionWithMetadata) of a transaction stored
    /// in the wallet.
    /// Expected response: [`Transaction`](crate::Response::Transaction)
    #[serde(rename_all = "camelCase")]
    GetTransaction { transaction_id: TransactionId },
    // /// Get the wallet's total voting power (voting or NOT voting).
    // /// Expected response: [`VotingPower`](crate::Response::VotingPower)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // GetVotingPower,
    /// Returns the implicit account creation address of the wallet if it is Ed25519 based.
    /// Expected response: [`Bech32Address`](crate::Response::Bech32Address)
    ImplicitAccountCreationAddress,
    /// Prepares to transition an implicit account to an account.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[serde(rename_all = "camelCase")]
    PrepareImplicitAccountTransition {
        output_id: OutputId,
        #[serde(default)]
        public_key: Option<String>,
        #[serde(default)]
        bip_path: Option<Bip44>,
    },
    /// Returns the implicit accounts of the wallet.
    /// Expected response: [`OutputsData`](crate::Response::OutputsData)
    ImplicitAccounts,
    /// Returns all incoming transactions of the wallet.
    /// Expected response: [`Transactions`](crate::Response::Transactions)
    IncomingTransactions,
    /// Returns all outputs of the wallet.
    /// Expected response: [`OutputsData`](crate::Response::OutputsData)
    #[serde(rename_all = "camelCase")]
    Outputs { filter_options: Option<FilterOptions> },
    /// Returns all pending transactions of the wallet.
    /// Expected response: [`Transactions`](crate::Response::Transactions)
    PendingTransactions,
    /// A generic function that can be used to burn native tokens, nfts, foundries and accounts.
    ///
    /// Note that burning **native tokens** doesn't require the foundry output which minted them, but will not
    /// increase the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output.
    /// Therefore it's recommended to use melting, if the foundry output is available.
    ///
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareBurn {
        burn: Burn,
        options: Option<TransactionOptions>,
    },
    /// Claim outputs.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[serde(rename_all = "camelCase")]
    PrepareClaimOutputs { output_ids_to_claim: Vec<OutputId> },
    /// Consolidate outputs.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareConsolidateOutputs { params: ConsolidationParams },
    /// Create an alias output.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareCreateAccountOutput {
        params: Option<CreateAccountParams>,
        options: Option<TransactionOptions>,
    },
    /// Prepare to create a native token.
    /// Expected response:
    /// [`PreparedCreateNativeTokenTransaction`](crate::Response::PreparedCreateNativeTokenTransaction)
    PrepareCreateNativeToken {
        params: CreateNativeTokenParams,
        options: Option<TransactionOptions>,
    },
    // /// Reduces a wallet's "voting power" by a given amount.
    // /// This will stop voting, but the voting data isn't lost and calling `Vote` without parameters will revote.
    // /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // PrepareDecreaseVotingPower {
    //     #[serde(with = "iota_sdk::utils::serde::string")]
    //     amount: u64,
    // },
    // /// Designates a given amount of tokens towards a wallet's "voting power" by creating a
    // /// special output, which is really a basic one with some metadata.
    // /// This will stop voting in most cases (if there is a remainder output), but the voting data isn't lost and
    // /// calling `Vote` without parameters will revote. Expected response:
    // /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // PrepareIncreaseVotingPower {
    //     #[serde(with = "iota_sdk::utils::serde::string")]
    //     amount: u64,
    // },
    /// Prepare to melt native tokens. This happens with the foundry output which minted them, by increasing it's
    /// `melted_tokens` field.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[serde(rename_all = "camelCase")]
    PrepareMeltNativeToken {
        /// Native token id
        token_id: TokenId,
        /// To be melted amount
        melt_amount: U256,
        options: Option<TransactionOptions>,
    },
    /// Prepare to mint additional native tokens.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[serde(rename_all = "camelCase")]
    PrepareMintNativeToken {
        /// Native token id
        token_id: TokenId,
        /// To be minted amount
        mint_amount: U256,
        options: Option<TransactionOptions>,
    },
    /// Prepare to mint NFTs.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareMintNfts {
        params: Vec<MintNftParams>,
        options: Option<TransactionOptions>,
    },
    /// Prepare an output.
    /// Expected response: [`Output`](crate::Response::Output)
    #[serde(rename_all = "camelCase")]
    PrepareOutput {
        params: Box<OutputParams>,
        transaction_options: Option<TransactionOptions>,
    },
    /// Prepare to send base coins.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareSend {
        params: Vec<SendParams>,
        options: Option<TransactionOptions>,
    },
    /// Prepare to send native tokens.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareSendNativeTokens {
        params: Vec<SendNativeTokenParams>,
        options: Option<TransactionOptions>,
    },
    /// Prepare to Send nft.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareSendNft {
        params: Vec<SendNftParams>,
        options: Option<TransactionOptions>,
    },
    // /// Stop participating for an event.
    // /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // #[serde(rename_all = "camelCase")]
    // PrepareStopParticipating { event_id: ParticipationEventId },
    /// Prepare transaction.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareTransaction {
        outputs: Vec<Output>,
        options: Option<TransactionOptions>,
    },
    // /// Vote for a participation event.
    // /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // #[serde(rename_all = "camelCase")]
    // PrepareVote {
    //     event_id: Option<ParticipationEventId>,
    //     answers: Option<Vec<u8>>,
    // },
    // /// Stores participation information locally and returns the event.
    // ///
    // /// This will NOT store the node url and auth inside the client options.
    // /// Expected response: [`ParticipationEvents`](crate::Response::ParticipationEvents)
    // #[cfg(feature = "participation")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    // RegisterParticipationEvents {
    //     options: ParticipationEventRegistrationOptions,
    // },
    /// Reissues a transaction sent from the wallet for a provided transaction id until it's
    /// included (referenced by a milestone). Returns the included block id.
    /// Expected response: [`BlockId`](crate::Response::BlockId)
    #[serde(rename_all = "camelCase")]
    ReissueTransactionUntilIncluded {
        /// Transaction id
        transaction_id: TransactionId,
        /// Interval
        interval: Option<u64>,
        /// Maximum attempts
        max_attempts: Option<u64>,
    },
    /// Send base coins.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    Send {
        #[serde(with = "iota_sdk::utils::serde::string")]
        amount: u64,
        address: Bech32Address,
        options: Option<TransactionOptions>,
    },
    /// Send base coins to multiple addresses, or with additional parameters.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    SendWithParams {
        params: Vec<SendParams>,
        options: Option<TransactionOptions>,
    },
    /// Send outputs in a transaction.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    SendOutputs {
        outputs: Vec<Output>,
        options: Option<TransactionOptions>,
    },
    /// Set the alias of the wallet.
    /// Expected response: [`Ok`](crate::Response::Ok)
    SetAlias { alias: String },
    /// Set the fallback SyncOptions for wallet syncing.
    /// If storage is enabled, will persist during restarts.
    /// Expected response: [`Ok`](crate::Response::Ok)
    SetDefaultSyncOptions { options: SyncOptions },
    /// Validate the transaction, sign it, submit it to a node and store it in the wallet.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    SignAndSubmitTransaction {
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Sign a prepared transaction.
    /// Expected response: [`SignedTransactionData`](crate::Response::SignedTransactionData)
    #[serde(rename_all = "camelCase")]
    SignTransaction {
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Validate the transaction, submit it to a node and store it in the wallet.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    SubmitAndStoreTransaction {
        signed_transaction_data: SignedTransactionDataDto,
    },
    /// Sync the wallet by fetching new information from the nodes. Will also reissue pending transactions
    /// if necessary. A custom default can be set using SetDefaultSyncOptions.
    /// Expected response: [`Balance`](crate::Response::Balance)
    Sync {
        /// Sync options
        options: Option<SyncOptions>,
    },
    /// Returns all transactions of the wallet
    /// Expected response: [`Transactions`](crate::Response::Transactions)
    Transactions,
    /// Returns all unspent outputs of the wallet
    /// Expected response: [`OutputsData`](crate::Response::OutputsData)
    #[serde(rename_all = "camelCase")]
    UnspentOutputs { filter_options: Option<FilterOptions> },

    /// Emits an event for testing if the event system is working
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    EmitTestEvent { event: WalletEvent },

    // TODO: reconsider whether to have the following methods on the wallet
    /// Generate an address without storing it
    /// Expected response: [`Bech32Address`](crate::Response::Bech32Address)
    #[serde(rename_all = "camelCase")]
    GenerateEd25519Address {
        /// Account index
        account_index: u32,
        /// Account index
        address_index: u32,
        /// Options
        options: Option<GenerateAddressOptions>,
        /// Bech32 HRP
        bech32_hrp: Option<Hrp>,
    },
    /// Get the ledger nano status
    /// Expected response: [`LedgerNanoStatus`](crate::Response::LedgerNanoStatus)
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    GetLedgerNanoStatus,
    /// Set the stronghold password.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    SetStrongholdPassword {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        password: String,
    },
    /// Set the stronghold password clear interval.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[serde(rename_all = "camelCase")]
    SetStrongholdPasswordClearInterval { interval_in_milliseconds: Option<u64> },
    /// Store a mnemonic into the Stronghold vault.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    StoreMnemonic {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        mnemonic: String,
    },
    /// Change the Stronghold password to another one and also re-encrypt the values in the loaded snapshot with it.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[serde(rename_all = "camelCase")]
    ChangeStrongholdPassword {
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        current_password: String,
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        new_password: String,
    },
    /// Clears the Stronghold password from memory.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    ClearStrongholdPassword,
    /// Checks if the Stronghold password is available.
    /// Expected response:
    /// [`Bool`](crate::Response::Bool)
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    IsStrongholdPasswordAvailable,
}
