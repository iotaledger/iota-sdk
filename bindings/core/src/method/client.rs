// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
#[cfg(feature = "mqtt")]
use iota_sdk::client::mqtt::Topic;
use iota_sdk::{
    client::{
        api::{
            ClientBlockBuilderOptions as BuildBlockOptions, GetAddressesOptions as GenerateAddressesOptions,
            PreparedTransactionDataDto,
        },
        node_api::indexer::query_parameters::QueryParameter,
        node_manager::node::NodeAuth,
        secret::SecretManagerDto,
    },
    types::block::{
        address::{Bech32Address, Hrp},
        output::{
            dto::{OutputDto, TokenSchemeDto},
            feature::dto::FeatureDto,
            unlock_condition::dto::UnlockConditionDto,
            AliasId, FoundryId, NativeToken, NftId, OutputId,
        },
        payload::{dto::PayloadDto, milestone::MilestoneId, transaction::TransactionId},
        BlockDto, BlockId,
    },
};
use serde::{Deserialize, Serialize};

use crate::OmittedDebug;

/// Each public client method.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
pub enum ClientMethod {
    /// Build an AliasOutput.
    /// Expected response: [`Output`](crate::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildAliasOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        native_tokens: Option<Vec<NativeToken>>,
        alias_id: AliasId,
        state_index: Option<u32>,
        state_metadata: Option<String>,
        foundry_counter: Option<u32>,
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
        immutable_features: Option<Vec<FeatureDto>>,
    },
    /// Build a BasicOutput.
    /// Expected response: [`Output`](crate::Response::Output)
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
    /// Expected response: [`Output`](crate::Response::Output)
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
    /// Expected response: [`Output`](crate::Response::Output)
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
    /// Removes all listeners for the provided topics.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    ClearListeners {
        /// Topics for which listeners should be removed.
        topics: Vec<Topic>,
    },
    /// Build and post a block
    #[serde(rename_all = "camelCase")]
    BuildAndPostBlock {
        /// Secret manager
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        secret_manager: Option<SecretManagerDto>,
        /// Options
        options: Option<BuildBlockOptions>,
    },
    /// Get a node candidate from the healthy node pool.
    GetNode,
    /// Gets the network related information such as network_id and min_pow_score
    GetNetworkInfo,
    /// Gets the network id of the node we're connecting to.
    GetNetworkId,
    /// Returns the bech32_hrp
    GetBech32Hrp,
    /// Returns the min pow score
    GetMinPowScore,
    /// Returns the tips interval
    GetTipsInterval,
    /// Returns the protocol parameters
    GetProtocolParameters,
    /// Returns if local pow should be used or not
    GetLocalPow,
    /// Get fallback to local proof of work timeout
    GetFallbackToLocalPow,
    /// Returns the unhealthy nodes.
    #[cfg(not(target_family = "wasm"))]
    UnhealthyNodes,
    /// Extension method which provides request methods for plugins.
    #[serde(rename_all = "camelCase")]
    CallPluginRoute {
        base_plugin_path: String,
        method: String,
        endpoint: String,
        query_params: Vec<String>,
        request_object: Option<String>,
    },
    /// Prepare a transaction for signing
    #[serde(rename_all = "camelCase")]
    PrepareTransaction {
        /// Secret manager
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        secret_manager: Option<SecretManagerDto>,
        /// Options
        options: Option<BuildBlockOptions>,
    },
    /// Sign a transaction
    #[serde(rename_all = "camelCase")]
    SignTransaction {
        /// Secret manager
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        secret_manager: SecretManagerDto,
        /// Prepared transaction data
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Build a block containing the specified payload and post it to the network.
    PostBlockPayload {
        /// The payload to send
        payload: PayloadDto,
    },
    //////////////////////////////////////////////////////////////////////
    // Node core API
    //////////////////////////////////////////////////////////////////////
    /// Get health
    GetHealth {
        /// Url
        url: String,
    },
    /// Get node info
    GetNodeInfo {
        /// Url
        url: String,
        /// Node authentication
        auth: Option<NodeAuth>,
    },
    /// Returns the node information together with the url of the used node
    GetInfo,
    /// Get peers
    GetPeers,
    /// Get tips
    GetTips,
    /// Post block (JSON)
    PostBlock {
        /// Block
        block: BlockDto,
    },
    /// Post block (raw)
    #[serde(rename_all = "camelCase")]
    PostBlockRaw {
        /// Block
        block_bytes: Vec<u8>,
    },
    /// Get block
    #[serde(rename_all = "camelCase")]
    GetBlock {
        /// Block ID
        block_id: BlockId,
    },
    /// Get block metadata with block_id
    #[serde(rename_all = "camelCase")]
    GetBlockMetadata {
        /// Block ID
        block_id: BlockId,
    },
    /// Get block raw
    #[serde(rename_all = "camelCase")]
    GetBlockRaw {
        /// Block ID
        block_id: BlockId,
    },
    /// Get output
    #[serde(rename_all = "camelCase")]
    GetOutput {
        /// Output ID
        output_id: OutputId,
    },
    /// Get output metadata
    #[serde(rename_all = "camelCase")]
    GetOutputMetadata {
        /// Output ID
        output_id: OutputId,
    },
    /// Get the milestone by the given milestone id.
    #[serde(rename_all = "camelCase")]
    GetMilestoneById {
        /// Milestone ID
        milestone_id: MilestoneId,
    },
    /// Get the raw milestone by the given milestone id.
    #[serde(rename_all = "camelCase")]
    GetMilestoneByIdRaw {
        /// Milestone ID
        milestone_id: MilestoneId,
    },
    /// Get the milestone by the given index.
    GetMilestoneByIndex {
        /// Milestone Index
        index: u32,
    },
    /// Get the raw milestone by the given index.
    GetMilestoneByIndexRaw {
        /// Milestone Index
        index: u32,
    },
    /// Get the UTXO changes by the given milestone id.
    #[serde(rename_all = "camelCase")]
    GetUtxoChangesById {
        /// Milestone ID
        milestone_id: MilestoneId,
    },
    /// Get the UTXO changes by the given milestone index.
    GetUtxoChangesByIndex {
        /// Milestone Index
        index: u32,
    },
    /// Get all receipts.
    GetReceipts,
    /// Get the receipts by the given milestone index.
    #[serde(rename_all = "camelCase")]
    GetReceiptsMigratedAt {
        /// Milestone index
        milestone_index: u32,
    },
    /// Get the treasury output.
    GetTreasury,
    /// Returns the included block of the transaction.
    #[serde(rename_all = "camelCase")]
    GetIncludedBlock {
        /// Transaction ID
        transaction_id: TransactionId,
    },
    /// Returns the included block metadata of the transaction.
    #[serde(rename_all = "camelCase")]
    GetIncludedBlockMetadata {
        /// Transaction ID
        transaction_id: TransactionId,
    },

    //////////////////////////////////////////////////////////////////////
    // Node indexer API
    //////////////////////////////////////////////////////////////////////
    /// Fetch basic output IDs
    #[serde(rename_all = "camelCase")]
    BasicOutputIds {
        /// Query parameters for output requests
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch alias output IDs
    #[serde(rename_all = "camelCase")]
    AliasOutputIds {
        /// Query parameters for output requests
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch alias output ID
    #[serde(rename_all = "camelCase")]
    AliasOutputId {
        /// Alias id
        alias_id: AliasId,
    },
    /// Fetch NFT output IDs
    #[serde(rename_all = "camelCase")]
    NftOutputIds {
        /// Query parameters for output requests
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch NFT output ID
    #[serde(rename_all = "camelCase")]
    NftOutputId {
        /// NFT ID
        nft_id: NftId,
    },
    /// Fetch foundry Output IDs
    #[serde(rename_all = "camelCase")]
    FoundryOutputIds {
        /// Query parameters for output requests
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch foundry Output ID
    #[serde(rename_all = "camelCase")]
    FoundryOutputId {
        /// Foundry ID
        foundry_id: FoundryId,
    },

    //////////////////////////////////////////////////////////////////////
    // High level API
    //////////////////////////////////////////////////////////////////////
    /// Fetch OutputWithMetadataResponse from provided OutputIds (requests are sent in parallel)
    #[serde(rename_all = "camelCase")]
    GetOutputs {
        /// Output IDs
        output_ids: Vec<OutputId>,
    },
    /// Try to get OutputWithMetadataResponse from provided OutputIds (requests are sent in parallel and errors are
    /// ignored, can be useful for spent outputs)
    #[serde(rename_all = "camelCase")]
    GetOutputsIgnoreErrors {
        /// Output IDs
        output_ids: Vec<OutputId>,
    },
    /// Find all blocks by provided block IDs.
    #[serde(rename_all = "camelCase")]
    FindBlocks {
        /// BlockIDs
        block_ids: Vec<BlockId>,
    },
    /// Retries (promotes or reattaches) a block for provided block id. Block should only be
    /// retried only if they are valid and haven't been confirmed for a while.
    #[serde(rename_all = "camelCase")]
    Retry {
        /// Block ID
        block_id: BlockId,
    },
    /// Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
    /// milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
    /// position and additional reattached blocks
    #[serde(rename_all = "camelCase")]
    RetryUntilIncluded {
        /// Block ID
        block_id: BlockId,
        /// Interval
        interval: Option<u64>,
        /// Maximum attempts
        max_attempts: Option<u64>,
    },
    /// Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
    /// Returns the address to which the funds got consolidated, if any were available
    #[serde(rename_all = "camelCase")]
    ConsolidateFunds {
        /// Secret manager
        #[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))]
        secret_manager: SecretManagerDto,
        /// Addresses generation options
        generate_addresses_options: GenerateAddressesOptions,
    },
    /// Function to find inputs from addresses for a provided amount (useful for offline signing)
    FindInputs {
        /// Addresses
        addresses: Vec<Bech32Address>,
        /// Amount
        amount: u64,
    },
    /// Find all outputs based on the requests criteria. This method will try to query multiple nodes if
    /// the request amount exceeds individual node limit.
    #[serde(rename_all = "camelCase")]
    FindOutputs {
        /// Output IDs
        output_ids: Vec<OutputId>,
        /// Addresses
        addresses: Vec<Bech32Address>,
    },
    /// Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    #[serde(rename_all = "camelCase")]
    Reattach {
        /// Block ID
        block_id: BlockId,
    },
    /// Reattach a block without checking if it should be reattached
    #[serde(rename_all = "camelCase")]
    ReattachUnchecked {
        /// Block ID
        block_id: BlockId,
    },
    /// Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
    /// method should error out and should not allow unnecessary promotions.
    #[serde(rename_all = "camelCase")]
    Promote {
        /// Block ID
        block_id: BlockId,
    },
    /// Promote a block without checking if it should be promoted
    #[serde(rename_all = "camelCase")]
    PromoteUnchecked {
        /// Block ID
        block_id: BlockId,
    },

    //////////////////////////////////////////////////////////////////////
    // Utils
    //////////////////////////////////////////////////////////////////////
    /// Transforms a hex encoded address to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    HexToBech32 {
        /// Hex encoded bech32 address
        hex: String,
        /// Human readable part
        bech32_hrp: Option<Hrp>,
    },
    /// Transforms an alias id to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    AliasIdToBech32 {
        /// Alias ID
        alias_id: AliasId,
        /// Human readable part
        bech32_hrp: Option<Hrp>,
    },
    /// Transforms an nft id to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    NftIdToBech32 {
        /// Nft ID
        nft_id: NftId,
        /// Human readable part
        bech32_hrp: Option<Hrp>,
    },
    /// Transforms a hex encoded public key to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    HexPublicKeyToBech32Address {
        /// Hex encoded public key
        hex: String,
        /// Human readable part
        bech32_hrp: Option<Hrp>,
    },
    /// Calculate the minimum required storage deposit for an output.
    /// Expected response:
    /// [`MinimumRequiredStorageDeposit`](crate::Response::MinimumRequiredStorageDeposit)
    MinimumRequiredStorageDeposit { output: OutputDto },
    /// Requests funds for a given address from the faucet, for example `https://faucet.testnet.shimmer.network/api/enqueue` or `http://localhost:8091/api/enqueue`.
    RequestFundsFromFaucet {
        /// Faucet URL
        url: String,
        /// The address for request funds
        address: Bech32Address,
    },
}
