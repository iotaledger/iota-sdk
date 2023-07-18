// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "participation")]
use iota_sdk::{
    client::node_manager::node::Node,
    types::api::plugins::participation::types::{ParticipationEventId, ParticipationEventType},
    wallet::account::types::participation::ParticipationEventRegistrationOptions,
};
use iota_sdk::{
    client::{
        api::{input_selection::BurnDto, PreparedTransactionDataDto, SignedTransactionDataDto},
        secret::GenerateAddressOptions,
    },
    types::block::{
        address::Bech32Address,
        output::{dto::OutputDto, OutputId, TokenId},
        payload::transaction::TransactionId,
    },
    wallet::{
        account::{
            CreateAliasParams, CreateNativeTokenParams, FilterOptions, MintNftParams, OutputParams, OutputsToClaim,
            SyncOptions, TransactionOptionsDto,
        },
        SendNativeTokensParams, SendNftParams, SendParams,
    },
    U256,
};
use serde::{Deserialize, Serialize};

/// Each public account method.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
pub enum AccountMethod {
    /// List addresses.
    /// Expected response: [`Addresses`](crate::Response::Addresses)
    Addresses,
    /// Returns only addresses of the account with unspent outputs
    /// Expected response:
    /// [`AddressesWithUnspentOutputs`](crate::Response::AddressesWithUnspentOutputs)
    AddressesWithUnspentOutputs,
    /// Get outputs with additional unlock conditions
    /// Expected response: [`OutputIds`](crate::Response::OutputIds)
    #[serde(rename_all = "camelCase")]
    ClaimableOutputs { outputs_to_claim: OutputsToClaim },
    /// Claim outputs.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    ClaimOutputs { output_ids_to_claim: Vec<OutputId> },
    /// Removes a previously registered participation event from local storage.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    DeregisterParticipationEvent { event_id: ParticipationEventId },
    /// Generate new Ed25519 addresses.
    /// Expected response: [`GeneratedEd25519Addresses`](crate::Response::GeneratedEd25519Addresses)
    GenerateEd25519Addresses {
        amount: u32,
        options: Option<GenerateAddressOptions>,
    },
    /// Get account balance information.
    /// Expected response: [`Balance`](crate::Response::Balance)
    GetBalance,
    /// Get the [`Output`](iota_sdk::types::block::output::Output) that minted a native token by its TokenId
    /// Expected response: [`Output`](crate::Response::Output)
    #[serde(rename_all = "camelCase")]
    GetFoundryOutput { token_id: TokenId },
    /// Get the transaction with inputs of an incoming transaction stored in the account
    /// List might not be complete, if the node pruned the data already
    /// Expected response: [`Transaction`](crate::Response::Transaction)
    #[serde(rename_all = "camelCase")]
    GetIncomingTransaction { transaction_id: TransactionId },
    /// Get the [`OutputData`](iota_sdk::wallet::account::types::OutputData) of an output stored in the account
    /// Expected response: [`OutputData`](crate::Response::OutputData)
    #[serde(rename_all = "camelCase")]
    GetOutput { output_id: OutputId },
    /// Expected response: [`ParticipationEvent`](crate::Response::ParticipationEvent)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    GetParticipationEvent { event_id: ParticipationEventId },
    /// Expected response: [`ParticipationEventIds`](crate::Response::ParticipationEventIds)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    GetParticipationEventIds {
        node: Node,
        event_type: Option<ParticipationEventType>,
    },
    /// Expected response:
    /// [`ParticipationEventStatus`](crate::Response::ParticipationEventStatus)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    GetParticipationEventStatus { event_id: ParticipationEventId },
    /// Expected response: [`ParticipationEvents`](crate::Response::ParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    GetParticipationEvents,
    /// Calculates a participation overview for an account. If event_ids are provided, only return outputs and tracked
    /// participations for them.
    /// Expected response:
    /// [`AccountParticipationOverview`](crate::Response::AccountParticipationOverview)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    GetParticipationOverview {
        event_ids: Option<Vec<ParticipationEventId>>,
    },
    /// Get the [`Transaction`](iota_sdk::wallet::account::types::Transaction) of a transaction stored in the account
    /// Expected response: [`Transaction`](crate::Response::Transaction)
    #[serde(rename_all = "camelCase")]
    GetTransaction { transaction_id: TransactionId },
    /// Get the account's total voting power (voting or NOT voting).
    /// Expected response: [`VotingPower`](crate::Response::VotingPower)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    GetVotingPower,
    /// Returns all incoming transactions of the account
    /// Expected response:
    /// [`Transactions`](crate::Response::Transactions)
    IncomingTransactions,
    /// Returns all outputs of the account
    /// Expected response: [`OutputsData`](crate::Response::OutputsData)
    #[serde(rename_all = "camelCase")]
    Outputs { filter_options: Option<FilterOptions> },
    /// Returns all pending transactions of the account
    /// Expected response: [`Transactions`](crate::Response::Transactions)
    PendingTransactions,
    /// A generic `burn()` function that can be used to burn native tokens, nfts, foundries and aliases.
    ///
    /// Note that burning **native tokens** doesn't require the foundry output which minted them, but will not
    /// increase the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output.
    /// Therefore it's recommended to use melting, if the foundry output is available.
    ///
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareBurn {
        burn: BurnDto,
        options: Option<TransactionOptionsDto>,
    },
    /// Consolidate outputs.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[serde(rename_all = "camelCase")]
    PrepareConsolidateOutputs {
        force: bool,
        output_consolidation_threshold: Option<usize>,
    },
    /// Create an alias output.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareCreateAliasOutput {
        params: Option<CreateAliasParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Prepare to create a native token.
    /// Expected response:
    /// [`PreparedCreateNativeTokenTransaction`](crate::Response::PreparedCreateNativeTokenTransaction)
    PrepareCreateNativeToken {
        params: CreateNativeTokenParams,
        options: Option<TransactionOptionsDto>,
    },
    /// Reduces an account's "voting power" by a given amount.
    /// This will stop voting, but the voting data isn't lost and calling `Vote` without parameters will revote.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    PrepareDecreaseVotingPower { amount: String },
    /// Designates a given amount of tokens towards an account's "voting power" by creating a
    /// special output, which is really a basic one with some metadata.
    /// This will stop voting in most cases (if there is a remainder output), but the voting data isn't lost and
    /// calling `Vote` without parameters will revote. Expected response:
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    PrepareIncreaseVotingPower { amount: String },
    /// Prepare to melt native tokens. This happens with the foundry output which minted them, by increasing it's
    /// `melted_tokens` field.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[serde(rename_all = "camelCase")]
    PrepareMeltNativeToken {
        /// Native token id
        token_id: TokenId,
        /// To be melted amount
        melt_amount: U256,
        options: Option<TransactionOptionsDto>,
    },
    /// Prepare to mint additional native tokens.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[serde(rename_all = "camelCase")]
    PrepareMintNativeToken {
        /// Native token id
        token_id: TokenId,
        /// To be minted amount
        mint_amount: U256,
        options: Option<TransactionOptionsDto>,
    },
    /// Prepare to mint NFTs.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareMintNfts {
        params: Vec<MintNftParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Prepare an output.
    /// Expected response: [`Output`](crate::Response::Output)
    #[serde(rename_all = "camelCase")]
    PrepareOutput {
        params: Box<OutputParams>,
        transaction_options: Option<TransactionOptionsDto>,
    },
    /// Prepare to send base coins.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareSend {
        params: Vec<SendParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Prepare to send native tokens.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareSendNativeTokens {
        params: Vec<SendNativeTokensParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Prepare to Send nft.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareSendNft {
        params: Vec<SendNftParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Stop participating for an event.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    PrepareStopParticipating { event_id: ParticipationEventId },
    /// Prepare transaction.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareTransaction {
        outputs: Vec<OutputDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Vote for a participation event.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    PrepareVote {
        event_id: Option<ParticipationEventId>,
        answers: Option<Vec<u8>>,
    },
    /// Stores participation information locally and returns the event.
    ///
    /// This will NOT store the node url and auth inside the client options.
    /// Expected response: [`ParticipationEvents`](crate::Response::ParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    RegisterParticipationEvents {
        options: ParticipationEventRegistrationOptions,
    },
    /// Retries (promotes or reattaches) a transaction sent from the account for a provided transaction id until it's
    /// included (referenced by a milestone). Returns the included block id.
    /// Expected response: [`BlockId`](crate::Response::BlockId)
    #[serde(rename_all = "camelCase")]
    RetryTransactionUntilIncluded {
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
        amount: u64,
        address: Bech32Address,
        options: Option<TransactionOptionsDto>,
    },
    /// Send base coins to multiple addresses, or with additional parameters.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    SendWithParams {
        params: Vec<SendParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Send outputs in a transaction.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    SendOutputs {
        outputs: Vec<OutputDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Set the alias of the account.
    /// Expected response: [`Ok`](crate::Response::Ok)
    SetAlias { alias: String },
    /// Set the fallback SyncOptions for account syncing.
    /// If storage is enabled, will persist during restarts.
    /// Expected response: [`Ok`](crate::Response::Ok)
    SetDefaultSyncOptions { options: SyncOptions },
    /// Validate the transaction, sign it, submit it to a node and store it in the account.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    SignAndSubmitTransaction {
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Sign a prepared transaction.
    /// Expected response: [`SignedTransactionData`](crate::Response::SignedTransactionData)
    #[serde(rename_all = "camelCase")]
    SignTransactionEssence {
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Validate the transaction, submit it to a node and store it in the account.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    SubmitAndStoreTransaction {
        signed_transaction_data: SignedTransactionDataDto,
    },
    /// Sync the account by fetching new information from the nodes. Will also retry pending transactions
    /// if necessary. A custom default can be set using SetDefaultSyncOptions.
    /// Expected response: [`Balance`](crate::Response::Balance)
    Sync {
        /// Sync options
        options: Option<SyncOptions>,
    },
    /// Returns all transaction of the account
    /// Expected response: [`Transactions`](crate::Response::Transactions)
    Transactions,
    /// Returns all unspent outputs of the account
    /// Expected response: [`OutputsData`](crate::Response::OutputsData)
    #[serde(rename_all = "camelCase")]
    UnspentOutputs { filter_options: Option<FilterOptions> },
}
