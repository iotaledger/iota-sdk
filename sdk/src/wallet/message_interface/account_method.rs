// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[cfg(feature = "participation")]
use crate::wallet::account::types::participation::ParticipationEventRegistrationOptions;
use crate::{
    client::api::{PreparedTransactionDataDto, SignedTransactionDataDto},
    types::block::{
        dto::U256Dto,
        output::{
            dto::{AliasIdDto, NativeTokenDto, NftIdDto, OutputDto, TokenIdDto, TokenSchemeDto},
            feature::dto::FeatureDto,
            unlock_condition::dto::UnlockConditionDto,
            FoundryId, OutputId,
        },
        payload::transaction::TransactionId,
    },
    wallet::{
        account::{
            operations::{
                address_generation::AddressGenerationOptions,
                output_claiming::OutputsToClaim,
                syncing::SyncOptions,
                transaction::{
                    high_level::{
                        create_alias::AliasOutputOptionsDto,
                        minting::{
                            increase_native_token_supply::IncreaseNativeTokenSupplyOptionsDto,
                            mint_native_token::NativeTokenOptionsDto, mint_nfts::NftOptionsDto,
                        },
                    },
                    prepare_output::OutputOptionsDto,
                    TransactionOptionsDto,
                },
            },
            FilterOptions,
        },
        message_interface::dtos::AddressWithAmountDto,
        AddressAndNftId, AddressNativeTokens,
    },
};
#[cfg(feature = "participation")]
use crate::{
    client::node_manager::node::Node,
    types::api::plugins::participation::types::{ParticipationEventId, ParticipationEventType},
};

/// Each public account method.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
pub enum AccountMethod {
    /// Build an AliasOutput.
    /// Expected response: [`Output`](crate::wallet::message_interface::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildAliasOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        native_tokens: Option<Vec<NativeTokenDto>>,
        alias_id: AliasIdDto,
        state_index: Option<u32>,
        state_metadata: Option<Vec<u8>>,
        foundry_counter: Option<u32>,
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
        immutable_features: Option<Vec<FeatureDto>>,
    },
    /// Build a BasicOutput.
    /// Expected response: [`Output`](crate::wallet::message_interface::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildBasicOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        native_tokens: Option<Vec<NativeTokenDto>>,
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
    },
    /// Build a FoundryOutput.
    /// Expected response: [`Output`](crate::wallet::message_interface::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildFoundryOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        native_tokens: Option<Vec<NativeTokenDto>>,
        serial_number: u32,
        token_scheme: TokenSchemeDto,
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
        immutable_features: Option<Vec<FeatureDto>>,
    },
    /// Build an NftOutput.
    /// Expected response: [`Output`](crate::wallet::message_interface::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildNftOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        native_tokens: Option<Vec<NativeTokenDto>>,
        nft_id: NftIdDto,
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
        immutable_features: Option<Vec<FeatureDto>>,
    },
    /// Burn native tokens. This doesn't require the foundry output which minted them, but will not increase
    /// the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
    /// recommended to use melting, if the foundry output is available.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    BurnNativeToken {
        /// Native token id
        token_id: TokenIdDto,
        /// To be burned amount
        burn_amount: U256Dto,
        options: Option<TransactionOptionsDto>,
    },
    /// Burn an nft output. Outputs controlled by it will be swept before if they don't have a storage
    /// deposit return, timelock or expiration unlock condition. This should be preferred over burning, because after
    /// burning, the foundry can never be destroyed anymore.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    BurnNft {
        nft_id: NftIdDto,
        options: Option<TransactionOptionsDto>,
    },
    /// Consolidate outputs.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    ConsolidateOutputs {
        force: bool,
        output_consolidation_threshold: Option<usize>,
    },
    /// Create an alias output.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    CreateAliasOutput {
        alias_output_options: Option<AliasOutputOptionsDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Destroy an alias output. Outputs controlled by it will be swept before if they don't have a
    /// storage deposit return, timelock or expiration unlock condition. The amount and possible native tokens will be
    /// sent to the governor address.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    DestroyAlias {
        alias_id: AliasIdDto,
        options: Option<TransactionOptionsDto>,
    },
    /// Function to destroy a foundry output with a circulating supply of 0.
    /// Native tokens in the foundry (minted by other foundries) will be transacted to the controlling alias
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    DestroyFoundry {
        foundry_id: FoundryId,
        options: Option<TransactionOptionsDto>,
    },
    /// Generate new unused addresses.
    /// Expected response: [`GeneratedAddress`](crate::wallet::message_interface::Response::GeneratedAddress)
    GenerateAddresses {
        amount: u32,
        options: Option<AddressGenerationOptions>,
    },
    /// Get the [`OutputData`](crate::wallet::account::types::OutputData) of an output stored in the account
    /// Expected response: [`OutputData`](crate::wallet::message_interface::Response::OutputData)
    #[serde(rename_all = "camelCase")]
    GetOutput { output_id: OutputId },
    /// Get the [`Output`](crate::types::block::output::Output) that minted a native token by its TokenId
    /// Expected response: [`Output`](crate::wallet::message_interface::Response::Output)
    #[serde(rename_all = "camelCase")]
    GetFoundryOutput { token_id: TokenIdDto },
    /// Get outputs with additional unlock conditions
    /// Expected response: [`OutputIds`](crate::wallet::message_interface::Response::OutputIds)
    #[serde(rename_all = "camelCase")]
    GetOutputsWithAdditionalUnlockConditions { outputs_to_claim: OutputsToClaim },
    /// Get the [`Transaction`](crate::wallet::account::types::Transaction) of a transaction stored in the account
    /// Expected response: [`Transaction`](crate::wallet::message_interface::Response::Transaction)
    #[serde(rename_all = "camelCase")]
    GetTransaction { transaction_id: TransactionId },
    /// Get the transaction with inputs of an incoming transaction stored in the account
    /// List might not be complete, if the node pruned the data already
    /// Expected response:
    /// [`IncomingTransactionData`](crate::wallet::message_interface::Response::IncomingTransactionData)
    #[serde(rename_all = "camelCase")]
    GetIncomingTransactionData { transaction_id: TransactionId },
    /// Expected response: [`Addresses`](crate::wallet::message_interface::Response::Addresses)
    /// List addresses.
    Addresses,
    /// Returns only addresses of the account with unspent outputs
    /// Expected response:
    /// [`AddressesWithUnspentOutputs`](crate::wallet::message_interface::Response::AddressesWithUnspentOutputs)
    AddressesWithUnspentOutputs,
    /// Returns all outputs of the account
    /// Expected response: [`OutputsData`](crate::wallet::message_interface::Response::OutputsData)
    #[serde(rename_all = "camelCase")]
    Outputs { filter_options: Option<FilterOptions> },
    /// Returns all unspent outputs of the account
    /// Expected response: [`OutputsData`](crate::wallet::message_interface::Response::OutputsData)
    #[serde(rename_all = "camelCase")]
    UnspentOutputs { filter_options: Option<FilterOptions> },
    /// Returns all incoming transactions of the account
    /// Expected response:
    /// [`IncomingTransactionsData`](crate::wallet::message_interface::Response::IncomingTransactionsData)
    IncomingTransactions,
    /// Returns all transaction of the account
    /// Expected response: [`Transactions`](crate::wallet::message_interface::Response::Transactions)
    Transactions,
    /// Returns all pending transactions of the account
    /// Expected response: [`Transactions`](crate::wallet::message_interface::Response::Transactions)
    PendingTransactions,
    /// Melt native tokens. This happens with the foundry output which minted them, by increasing it's
    /// `melted_tokens` field.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    DecreaseNativeTokenSupply {
        /// Native token id
        token_id: TokenIdDto,
        /// To be melted amount
        melt_amount: U256Dto,
        options: Option<TransactionOptionsDto>,
    },
    /// Calculate the minimum required storage deposit for an output.
    /// Expected response:
    /// [`MinimumRequiredStorageDeposit`](crate::wallet::message_interface::Response::MinimumRequiredStorageDeposit)
    MinimumRequiredStorageDeposit { output: OutputDto },
    /// Mint more native token.
    /// Expected response: [`MintTokenTransaction`](crate::wallet::message_interface::Response::MintTokenTransaction)
    #[serde(rename_all = "camelCase")]
    IncreaseNativeTokenSupply {
        /// Native token id
        token_id: TokenIdDto,
        /// To be minted amount
        mint_amount: U256Dto,
        increase_native_token_supply_options: Option<IncreaseNativeTokenSupplyOptionsDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Mint native token.
    /// Expected response: [`MintTokenTransaction`](crate::wallet::message_interface::Response::MintTokenTransaction)
    #[serde(rename_all = "camelCase")]
    MintNativeToken {
        native_token_options: NativeTokenOptionsDto,
        options: Option<TransactionOptionsDto>,
    },
    /// Mint nft.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    MintNfts {
        nfts_options: Vec<NftOptionsDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Get account balance information.
    /// Expected response: [`Balance`](crate::wallet::message_interface::Response::Balance)
    GetBalance,
    /// Prepare an output.
    /// Expected response: [`Output`](crate::wallet::message_interface::Response::Output)
    #[serde(rename_all = "camelCase")]
    PrepareOutput {
        options: OutputOptionsDto,
        transaction_options: Option<TransactionOptionsDto>,
    },
    /// Prepare transaction.
    /// Expected response: [`PreparedTransaction`](crate::wallet::message_interface::Response::PreparedTransaction)
    PrepareTransaction {
        outputs: Vec<OutputDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Prepare send amount.
    /// Expected response: [`PreparedTransaction`](crate::wallet::message_interface::Response::PreparedTransaction)
    #[serde(rename_all = "camelCase")]
    PrepareSendAmount {
        addresses_with_amount: Vec<AddressWithAmountDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Retries (promotes or reattaches) a transaction sent from the account for a provided transaction id until it's
    /// included (referenced by a milestone). Returns the included block id.
    /// Expected response: [`BlockId`](crate::wallet::message_interface::Response::BlockId)
    #[serde(rename_all = "camelCase")]
    RetryTransactionUntilIncluded {
        /// Transaction id
        transaction_id: TransactionId,
        /// Interval
        interval: Option<u64>,
        /// Maximum attempts
        max_attempts: Option<u64>,
    },
    /// Sync the account by fetching new information from the nodes. Will also retry pending transactions
    /// if necessary.
    /// Expected response: [`Balance`](crate::wallet::message_interface::Response::Balance)
    SyncAccount {
        /// Sync options
        options: Option<SyncOptions>,
    },
    /// Send amount.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    SendAmount {
        addresses_with_amount: Vec<AddressWithAmountDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Send native tokens.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    SendNativeTokens {
        addresses_and_native_tokens: Vec<AddressNativeTokens>,
        options: Option<TransactionOptionsDto>,
    },
    /// Send nft.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    SendNft {
        addresses_and_nft_ids: Vec<AddressAndNftId>,
        options: Option<TransactionOptionsDto>,
    },
    /// Set the alias of the account.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    SetAlias { alias: String },
    /// Set the fallback SyncOptions for account syncing.
    /// If storage is enabled, will persist during restarts.
    /// Expected response: [`Ok`](crate::Response::Ok)
    SetDefaultSyncOptions { options: SyncOptions },
    /// Send outputs in a transaction.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    SendOutputs {
        outputs: Vec<OutputDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Sign a prepared transaction.
    /// Expected response: [`SignedTransactionData`](crate::wallet::message_interface::Response::SignedTransactionData)
    #[serde(rename_all = "camelCase")]
    SignTransactionEssence {
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Validate the transaction, submit it to a node and store it in the account.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    SubmitAndStoreTransaction {
        signed_transaction_data: SignedTransactionDataDto,
    },
    /// Claim outputs.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    ClaimOutputs { output_ids_to_claim: Vec<OutputId> },
    /// Vote for a participation event.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    Vote {
        event_id: Option<ParticipationEventId>,
        answers: Option<Vec<u8>>,
    },
    /// Stop participating for an event.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    StopParticipating { event_id: ParticipationEventId },
    /// Calculates a participation overview for an account. If event_ids are provided, only return outputs and tracked
    /// participations for them.
    /// Expected response:
    /// [`AccountParticipationOverview`](crate::wallet::message_interface::Response::AccountParticipationOverview)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    GetParticipationOverview {
        event_ids: Option<Vec<ParticipationEventId>>,
    },
    /// Designates a given amount of tokens towards an account's "voting power" by creating a
    /// special output, which is really a basic one with some metadata.
    /// This will stop voting in most cases (if there is a remainder output), but the voting data isn't lost and
    /// calling `Vote` without parameters will revote. Expected response:
    /// [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    IncreaseVotingPower { amount: String },
    /// Reduces an account's "voting power" by a given amount.
    /// This will stop voting, but the voting data isn't lost and calling `Vote` without parameters will revote.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    DecreaseVotingPower { amount: String },
    /// Stores participation information locally and returns the event.
    ///
    /// This will NOT store the node url and auth inside the client options.
    /// Expected response: [`ParticipationEvents`](crate::wallet::message_interface::Response::ParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    RegisterParticipationEvents {
        options: ParticipationEventRegistrationOptions,
    },
    /// Removes a previously registered participation event from local storage.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    DeregisterParticipationEvent { event_id: ParticipationEventId },
    /// Expected response: [`ParticipationEvent`](crate::wallet::message_interface::Response::ParticipationEvent)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    GetParticipationEvent { event_id: ParticipationEventId },
    /// Expected response: [`ParticipationEventIds`](crate::wallet::message_interface::Response::ParticipationEventIds)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    GetParticipationEventIds {
        node: Node,
        event_type: Option<ParticipationEventType>,
    },
    /// Expected response:
    /// [`ParticipationEventStatus`](crate::wallet::message_interface::Response::ParticipationEventStatus)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[serde(rename_all = "camelCase")]
    GetParticipationEventStatus { event_id: ParticipationEventId },
    /// Expected response: [`ParticipationEvents`](crate::wallet::message_interface::Response::ParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    GetParticipationEvents,
    /// Expected response: [`Faucet`](crate::wallet::message_interface::Response::Faucet)
    RequestFundsFromFaucet { url: String, address: String },
}
