// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "participation")]
use iota_sdk::wallet::account::types::participation::ParticipationEventRegistrationOptions;
use iota_sdk::{
    client::api::{PreparedTransactionDataDto, SignedTransactionDataDto},
    types::block::{
        dto::U256Dto,
        output::{
            dto::{AliasIdDto, NftIdDto, OutputDto, TokenIdDto},
            FoundryId, OutputId,
        },
        payload::transaction::TransactionId,
    },
    wallet::{
        account::{
            AddressGenerationOptions, AliasOutputOptionsDto, FilterOptions, IncreaseNativeTokenSupplyOptionsDto,
            NativeTokenOptionsDto, NftOptionsDto, OutputOptionsDto, OutputsToClaim, SyncOptions, TransactionOptionsDto,
        },
        message_interface::dtos::AddressWithAmountDto,
        AddressAndNftId, AddressNativeTokens,
    },
};
#[cfg(feature = "participation")]
use iota_sdk::{
    client::node_manager::node::Node,
    types::api::plugins::participation::types::{ParticipationEventId, ParticipationEventType},
};
use serde::{Deserialize, Serialize};

/// Each public account method.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
pub enum AccountMethod {
    /// Burn native tokens. This doesn't require the foundry output which minted them, but will not increase
    /// the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
    /// recommended to use melting, if the foundry output is available.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    BurnNativeToken {
        /// Native token id
        #[serde(rename = "tokenId")]
        token_id: TokenIdDto,
        /// To be burned amount
        #[serde(rename = "burnAmount")]
        burn_amount: U256Dto,
        options: Option<TransactionOptionsDto>,
    },
    /// Burn an nft output. Outputs controlled by it will be swept before if they don't have a storage
    /// deposit return, timelock or expiration unlock condition. This should be preferred over burning, because after
    /// burning, the foundry can never be destroyed anymore.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    BurnNft {
        #[serde(rename = "nftId")]
        nft_id: NftIdDto,
        options: Option<TransactionOptionsDto>,
    },
    /// Consolidate outputs.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    ConsolidateOutputs {
        force: bool,
        #[serde(rename = "outputConsolidationThreshold")]
        output_consolidation_threshold: Option<usize>,
    },
    /// Create an alias output.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    CreateAliasOutput {
        #[serde(rename = "aliasOutputOptions")]
        alias_output_options: Option<AliasOutputOptionsDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Destroy an alias output. Outputs controlled by it will be swept before if they don't have a
    /// storage deposit return, timelock or expiration unlock condition. The amount and possible native tokens will be
    /// sent to the governor address.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    DestroyAlias {
        #[serde(rename = "aliasId")]
        alias_id: AliasIdDto,
        options: Option<TransactionOptionsDto>,
    },
    /// Function to destroy a foundry output with a circulating supply of 0.
    /// Native tokens in the foundry (minted by other foundries) will be transacted to the controlling alias
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    DestroyFoundry {
        #[serde(rename = "foundryId")]
        foundry_id: FoundryId,
        options: Option<TransactionOptionsDto>,
    },
    /// Generate new unused addresses.
    /// Expected response: [`GeneratedAddress`](crate::Response::GeneratedAddress)
    GenerateAddresses {
        amount: u32,
        options: Option<AddressGenerationOptions>,
    },
    /// Get the [`OutputData`](iota_sdk::account::types::OutputData) of an output stored in the account
    /// Expected response: [`OutputData`](crate::Response::OutputData)
    GetOutput {
        #[serde(rename = "outputId")]
        output_id: OutputId,
    },
    /// Get the [`Output`](iota_sdk::iota_sdk::client::block::output::Output) that minted a native token by its TokenId
    /// Expected response: [`Output`](crate::Response::Output)
    GetFoundryOutput {
        #[serde(rename = "tokenId")]
        token_id: TokenIdDto,
    },
    /// Get outputs with additional unlock conditions
    /// Expected response: [`OutputIds`](crate::Response::OutputIds)
    GetOutputsWithAdditionalUnlockConditions {
        #[serde(rename = "outputsToClaim")]
        outputs_to_claim: OutputsToClaim,
    },
    /// Get the [`Transaction`](iota_sdk::account::types::Transaction) of a transaction stored in the account
    /// Expected response: [`Transaction`](crate::Response::Transaction)
    GetTransaction {
        #[serde(rename = "transactionId")]
        transaction_id: TransactionId,
    },
    /// Get the transaction with inputs of an incoming transaction stored in the account
    /// List might not be complete, if the node pruned the data already
    /// Expected response: [`IncomingTransactionData`](crate::Response::IncomingTransactionData)
    GetIncomingTransactionData {
        #[serde(rename = "transactionId")]
        transaction_id: TransactionId,
    },
    /// Expected response: [`Addresses`](crate::Response::Addresses)
    /// List addresses.
    Addresses,
    /// Returns only addresses of the account with unspent outputs
    /// Expected response:
    /// [`AddressesWithUnspentOutputs`](crate::Response::AddressesWithUnspentOutputs)
    AddressesWithUnspentOutputs,
    /// Returns all outputs of the account
    /// Expected response: [`OutputsData`](crate::Response::OutputsData)
    Outputs {
        #[serde(rename = "filterOptions")]
        filter_options: Option<FilterOptions>,
    },
    /// Returns all unspent outputs of the account
    /// Expected response: [`OutputsData`](crate::Response::OutputsData)
    UnspentOutputs {
        #[serde(rename = "filterOptions")]
        filter_options: Option<FilterOptions>,
    },
    /// Returns all incoming transactions of the account
    /// Expected response:
    /// [`IncomingTransactionsData`](crate::Response::IncomingTransactionsData)
    IncomingTransactions,
    /// Returns all transaction of the account
    /// Expected response: [`Transactions`](crate::Response::Transactions)
    Transactions,
    /// Returns all pending transactions of the account
    /// Expected response: [`Transactions`](crate::Response::Transactions)
    PendingTransactions,
    /// Melt native tokens. This happens with the foundry output which minted them, by increasing it's
    /// `melted_tokens` field.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    DecreaseNativeTokenSupply {
        /// Native token id
        #[serde(rename = "tokenId")]
        token_id: TokenIdDto,
        /// To be melted amount
        #[serde(rename = "meltAmount")]
        melt_amount: U256Dto,
        options: Option<TransactionOptionsDto>,
    },
    /// Calculate the minimum required storage deposit for an output.
    /// Expected response:
    /// [`MinimumRequiredStorageDeposit`](crate::Response::MinimumRequiredStorageDeposit)
    MinimumRequiredStorageDeposit { output: OutputDto },
    /// Mint more native token.
    /// Expected response: [`MintTokenTransaction`](crate::Response::MintTokenTransaction)
    IncreaseNativeTokenSupply {
        /// Native token id
        #[serde(rename = "tokenId")]
        token_id: TokenIdDto,
        /// To be minted amount
        #[serde(rename = "mintAmount")]
        mint_amount: U256Dto,
        #[serde(rename = "increaseNativeTokenSupplyOptions")]
        increase_native_token_supply_options: Option<IncreaseNativeTokenSupplyOptionsDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Mint native token.
    /// Expected response: [`MintTokenTransaction`](crate::Response::MintTokenTransaction)
    MintNativeToken {
        #[serde(rename = "nativeTokenOptions")]
        native_token_options: NativeTokenOptionsDto,
        options: Option<TransactionOptionsDto>,
    },
    /// Mint nft.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    MintNfts {
        #[serde(rename = "nftsOptions")]
        nfts_options: Vec<NftOptionsDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Get account balance information.
    /// Expected response: [`Balance`](crate::Response::Balance)
    GetBalance,
    /// Prepare an output.
    /// Expected response: [`Output`](crate::Response::Output)
    PrepareOutput {
        options: OutputOptionsDto,
        #[serde(rename = "transactionOptions")]
        transaction_options: Option<TransactionOptionsDto>,
    },
    /// Prepare transaction.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareTransaction {
        outputs: Vec<OutputDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Prepare send amount.
    /// Expected response: [`PreparedTransaction`](crate::Response::PreparedTransaction)
    PrepareSendAmount {
        #[serde(rename = "addressesWithAmount")]
        addresses_with_amount: Vec<AddressWithAmountDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Retries (promotes or reattaches) a transaction sent from the account for a provided transaction id until it's
    /// included (referenced by a milestone). Returns the included block id.
    /// Expected response: [`BlockId`](crate::Response::BlockId)
    RetryTransactionUntilIncluded {
        /// Transaction id
        #[serde(rename = "transactionId")]
        transaction_id: TransactionId,
        /// Interval
        interval: Option<u64>,
        /// Maximum attempts
        #[serde(rename = "maxAttempts")]
        max_attempts: Option<u64>,
    },
    /// Sync the account by fetching new information from the nodes. Will also retry pending transactions
    /// if necessary.
    /// Expected response: [`Balance`](crate::Response::Balance)
    SyncAccount {
        /// Sync options
        options: Option<SyncOptions>,
    },
    /// Send amount.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    SendAmount {
        #[serde(rename = "addressesWithAmount")]
        addresses_with_amount: Vec<AddressWithAmountDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Send native tokens.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    SendNativeTokens {
        #[serde(rename = "addressesNativeTokens")]
        addresses_native_tokens: Vec<AddressNativeTokens>,
        options: Option<TransactionOptionsDto>,
    },
    /// Send nft.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    SendNft {
        #[serde(rename = "addressesAndNftIds")]
        addresses_nft_ids: Vec<AddressAndNftId>,
        options: Option<TransactionOptionsDto>,
    },
    /// Set the alias of the account.
    /// Expected response: [`Ok`](crate::Response::Ok)
    SetAlias { alias: String },
    /// Send outputs in a transaction.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    SendOutputs {
        outputs: Vec<OutputDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Sign a prepared transaction.
    /// Expected response: [`SignedTransactionData`](crate::Response::SignedTransactionData)
    SignTransactionEssence {
        #[serde(rename = "preparedTransactionData")]
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Validate the transaction, submit it to a node and store it in the account.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    SubmitAndStoreTransaction {
        #[serde(rename = "signedTransactionData")]
        signed_transaction_data: SignedTransactionDataDto,
    },
    /// Claim outputs.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    ClaimOutputs {
        #[serde(rename = "outputIdsToClaim")]
        output_ids_to_claim: Vec<OutputId>,
    },
    /// Vote for a participation event.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    Vote {
        #[serde(rename = "eventId")]
        event_id: Option<ParticipationEventId>,
        answers: Option<Vec<u8>>,
    },
    /// Stop participating for an event.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    StopParticipating {
        #[serde(rename = "eventId")]
        event_id: ParticipationEventId,
    },
    /// Get the account's total voting power (voting or NOT voting).
    /// Expected response: [`VotingPower`](crate::Response::VotingPower)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    GetVotingPower,
    /// Calculates a participation overview for an account. If event_ids are provided, only return outputs and tracked
    /// participations for them.
    /// Expected response:
    /// [`AccountParticipationOverview`](crate::Response::AccountParticipationOverview)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    GetParticipationOverview {
        #[serde(rename = "eventIds")]
        event_ids: Option<Vec<ParticipationEventId>>,
    },
    /// Designates a given amount of tokens towards an account's "voting power" by creating a
    /// special output, which is really a basic one with some metadata.
    /// This will stop voting in most cases (if there is a remainder output), but the voting data isn't lost and
    /// calling `Vote` without parameters will revote. Expected response:
    /// [`SentTransaction`](crate::Response::SentTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    IncreaseVotingPower { amount: String },
    /// Reduces an account's "voting power" by a given amount.
    /// This will stop voting, but the voting data isn't lost and calling `Vote` without parameters will revote.
    /// Expected response: [`SentTransaction`](crate::Response::SentTransaction)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    DecreaseVotingPower { amount: String },
    /// Stores participation information locally and returns the event.
    ///
    /// This will NOT store the node url and auth inside the client options.
    /// Expected response: [`ParticipationEvents`](crate::Response::ParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    RegisterParticipationEvents {
        options: ParticipationEventRegistrationOptions,
    },
    /// Removes a previously registered participation event from local storage.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    DeregisterParticipationEvent {
        #[serde(rename = "eventId")]
        event_id: ParticipationEventId,
    },
    /// Expected response: [`ParticipationEvent`](crate::Response::ParticipationEvent)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    GetParticipationEvent {
        #[serde(rename = "eventId")]
        event_id: ParticipationEventId,
    },
    /// Expected response: [`ParticipationEventIds`](crate::Response::ParticipationEventIds)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    GetParticipationEventIds {
        node: Node,
        #[serde(rename = "eventType")]
        event_type: Option<ParticipationEventType>,
    },
    /// Expected response:
    /// [`ParticipationEventStatus`](crate::Response::ParticipationEventStatus)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    GetParticipationEventStatus {
        #[serde(rename = "eventId")]
        event_id: ParticipationEventId,
    },
    /// Expected response: [`ParticipationEvents`](crate::Response::ParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    GetParticipationEvents,
}
