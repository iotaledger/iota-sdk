// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
#[cfg(feature = "participation")]
use iota_sdk::wallet::account::types::participation::ParticipationEventRegistrationOptions;
#[cfg(feature = "participation")]
use iota_sdk::{
    client::node_manager::node::Node,
    types::api::plugins::participation::types::{ParticipationEventId, ParticipationEventType},
};
use iota_sdk::{
    client::{
        api::{GetAddressesOptions, PreparedTransactionDataDto, SignedTransactionDataDto},
        secret::GenerateAddressOptions,
    },
    types::block::{
        address::Bech32Address,
        output::{
            dto::{OutputDto, TokenSchemeDto},
            feature::dto::FeatureDto,
            unlock_condition::dto::UnlockConditionDto,
            AliasId, FoundryId, NativeToken, NftId, OutputId, TokenId,
        },
        payload::transaction::TransactionId,
        signature::dto::Ed25519SignatureDto,
    },
    utils::serde::bip44::Bip44Def,
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
    /// Build an AliasOutput.
    /// Expected response: [`Output`](crate::wallet::message_interface::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildAliasOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        native_tokens: Option<Vec<NativeToken>>,
        alias_id: AliasId,
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
        native_tokens: Option<Vec<NativeToken>>,
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
        native_tokens: Option<Vec<NativeToken>>,
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
        native_tokens: Option<Vec<NativeToken>>,
        nft_id: NftId,
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
        token_id: TokenId,
        /// To be burned amount
        burn_amount: U256,
        options: Option<TransactionOptionsDto>,
    },
    /// Burn an nft output. Outputs controlled by it will be swept before if they don't have a storage
    /// deposit return, timelock or expiration unlock condition. This should be preferred over burning, because after
    /// burning, the foundry can never be destroyed anymore.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    BurnNft {
        nft_id: NftId,
        options: Option<TransactionOptionsDto>,
    },
    /// Get outputs with additional unlock conditions
    /// Expected response: [`OutputIds`](crate::wallet::message_interface::Response::OutputIds)
    #[serde(rename_all = "camelCase")]
    ClaimableOutputs { outputs_to_claim: OutputsToClaim },
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
        params: Option<CreateAliasParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Destroy an alias output. Outputs controlled by it will be swept before if they don't have a
    /// storage deposit return, timelock or expiration unlock condition. The amount and possible native tokens will be
    /// sent to the governor address.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    DestroyAlias {
        alias_id: AliasId,
        options: Option<TransactionOptionsDto>,
    },
    /// Destroy a foundry output with a circulating supply of 0.
    /// Native tokens in the foundry (minted by other foundries) will be transacted to the controlling alias
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    DestroyFoundry {
        foundry_id: FoundryId,
        options: Option<TransactionOptionsDto>,
    },
    /// Generate new unused ed25519 addresses.
    /// Expected response:
    /// [`GeneratedEd25519Addresses`](crate::wallet::message_interface::Response::GeneratedEd25519Addresses)
    GenerateEd25519Addresses {
        amount: u32,
        options: Option<GenerateAddressOptions>,
    },
    /// Generate EVM addresses.
    /// Expected response:
    /// [`GeneratedEvmAddresses`](crate::wallet::message_interface::Response::GeneratedEvmAddresses)
    GenerateEvmAddresses { options: GetAddressesOptions },
    /// Verify an ed25519 signature against a message.
    /// Expected response:
    /// [`Bool`](crate::wallet::message_interface::Response::Bool)
    VerifyEd25519Signature {
        signature: Ed25519SignatureDto,
        message: String,
    },
    /// Verify a Secp256k1Ecdsa signature against a message.
    /// Expected response:
    /// [`Bool`](crate::wallet::message_interface::Response::Bool)
    #[serde(rename_all = "camelCase")]
    VerifySecp256k1EcdsaSignature {
        public_key: String,
        signature: String,
        message: String,
    },
    /// Signs a message with an Secp256k1Ecdsa private key.
    SignSecp256k1Ecdsa {
        /// The message to sign, hex encoded String
        message: String,
        /// Chain to sign the message with
        #[serde(with = "Bip44Def")]
        chain: Bip44,
    },
    /// Get the [`OutputData`](crate::wallet::account::types::OutputData) of an output stored in the account
    /// Expected response: [`OutputData`](crate::wallet::message_interface::Response::OutputData)
    #[serde(rename_all = "camelCase")]
    GetOutput { output_id: OutputId },
    /// Get the [`Output`](crate::types::block::output::Output) that minted a native token by its TokenId
    /// Expected response: [`Output`](crate::wallet::message_interface::Response::Output)
    #[serde(rename_all = "camelCase")]
    GetFoundryOutput { token_id: TokenId },
    /// Get the [`Transaction`](crate::wallet::account::types::Transaction) of a transaction stored in the account
    /// Expected response: [`Transaction`](crate::wallet::message_interface::Response::Transaction)
    #[serde(rename_all = "camelCase")]
    GetTransaction { transaction_id: TransactionId },
    /// Get the transaction with inputs of an incoming transaction stored in the account
    /// List might not be complete, if the node pruned the data already
    /// Expected response:
    /// [`Transaction`](crate::wallet::message_interface::Response::Transaction)
    #[serde(rename_all = "camelCase")]
    GetIncomingTransaction { transaction_id: TransactionId },
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
    /// [`Transactions`](crate::wallet::message_interface::Response::Transactions)
    IncomingTransactions,
    /// Returns all transaction of the account
    /// Expected response: [`Transactions`](crate::wallet::message_interface::Response::Transactions)
    Transactions,
    /// Returns all pending transactions of the account
    /// Expected response: [`Transactions`](crate::wallet::message_interface::Response::Transactions)
    PendingTransactions,
    /// Melt native tokens. This happens with the foundry output which minted them, by increasing its
    /// `melted_tokens` field.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    MeltNativeToken {
        /// Native token id
        token_id: TokenId,
        /// To be melted amount
        melt_amount: U256,
        options: Option<TransactionOptionsDto>,
    },
    /// Calculate the minimum required storage deposit for an output.
    /// Expected response:
    /// [`MinimumRequiredStorageDeposit`](crate::wallet::message_interface::Response::MinimumRequiredStorageDeposit)
    MinimumRequiredStorageDeposit { output: OutputDto },
    /// Mint additional native tokens.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    MintNativeToken {
        /// Native token id
        token_id: TokenId,
        /// To be minted amount
        mint_amount: U256,
        options: Option<TransactionOptionsDto>,
    },
    /// Create a native token.
    /// Expected response:
    /// [`CreateNativeTokenTransaction`](crate::wallet::message_interface::Response::CreateNativeTokenTransaction)
    #[serde(rename_all = "camelCase")]
    CreateNativeToken {
        params: CreateNativeTokenParams,
        options: Option<TransactionOptionsDto>,
    },
    /// Mint nft.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    MintNfts {
        params: Vec<MintNftParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Get account balance information.
    /// Expected response: [`Balance`](crate::wallet::message_interface::Response::Balance)
    GetBalance,
    /// Prepare an output.
    /// Expected response: [`Output`](crate::wallet::message_interface::Response::Output)
    #[serde(rename_all = "camelCase")]
    PrepareOutput {
        params: Box<OutputParams>,
        transaction_options: Option<TransactionOptionsDto>,
    },
    /// Prepare transaction.
    /// Expected response: [`PreparedTransaction`](crate::wallet::message_interface::Response::PreparedTransaction)
    PrepareTransaction {
        outputs: Vec<OutputDto>,
        options: Option<TransactionOptionsDto>,
    },
    /// Prepare to send base coins.
    /// Expected response: [`PreparedTransaction`](crate::wallet::message_interface::Response::PreparedTransaction)
    #[serde(rename_all = "camelCase")]
    PrepareSend {
        params: Vec<SendParams>,
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
    /// if necessary. A custom default can be set using SetDefaultSyncOptions.
    /// Expected response: [`Balance`](crate::wallet::message_interface::Response::Balance)
    SyncAccount {
        /// Sync options
        options: Option<SyncOptions>,
    },
    /// Send base coins.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    Send {
        params: Vec<SendParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Send native tokens.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    SendNativeTokens {
        params: Vec<SendNativeTokensParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Send nft.
    /// Expected response: [`SentTransaction`](crate::wallet::message_interface::Response::SentTransaction)
    #[serde(rename_all = "camelCase")]
    SendNft {
        params: Vec<SendNftParams>,
        options: Option<TransactionOptionsDto>,
    },
    /// Set the alias of the account.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
    SetAlias { alias: String },
    /// Set the fallback SyncOptions for account syncing.
    /// If storage is enabled, will persist during restarts.
    /// Expected response: [`Ok`](crate::wallet::message_interface::Response::Ok)
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
    RequestFundsFromFaucet { url: String, address: Bech32Address },
}

#[cfg(test)]
mod test {
    #[test]
    fn bip44_deserialization() {
        let sign_secp256k1_ecdsa_method: super::AccountMethod = serde_json::from_str(
            r#"{"name": "signSecp256k1Ecdsa", "data": {"message": "0xFFFFFFFF", "chain": {"account": 2, "addressIndex": 1}}}"#,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_value(&sign_secp256k1_ecdsa_method).unwrap(),
            serde_json::json!({
                "name": "signSecp256k1Ecdsa",
                "data": {
                    "message": "0xFFFFFFFF",
                    "chain": {
                        "coinType": 4218,
                        "account": 2,
                        "change": 0,
                        "addressIndex": 1
                    }
                }
            })
        );
    }
}
