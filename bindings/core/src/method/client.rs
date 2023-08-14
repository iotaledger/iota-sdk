// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
#[cfg(feature = "mqtt")]
use iota_sdk::client::mqtt::Topic;
use iota_sdk::{
    client::{node_api::indexer::query_parameters::QueryParameter, node_manager::node::NodeAuth},
    types::block::{
        address::{Bech32Address, Hrp},
        output::{
            dto::OutputDto, feature::Feature, unlock_condition::dto::UnlockConditionDto, AccountId, FoundryId,
            NativeToken, NftId, OutputId, TokenScheme,
        },
        payload::{dto::PayloadDto, transaction::TransactionId},
        BlockDto, BlockId,
    },
    utils::serde::{option_string, string},
};
use serde::{Deserialize, Serialize};

/// Each public client method.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ClientMethod {
    /// Build an AccountOutput.
    /// Expected response: [`Output`](crate::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildAccountOutput {
        // If not provided, minimum storage deposit will be used
        #[serde(default, with = "option_string")]
        amount: Option<u64>,
        // TODO: Determine if `default` is wanted here
        #[serde(default, with = "string")]
        mana: u64,
        native_tokens: Option<Vec<NativeToken>>,
        account_id: AccountId,
        state_index: Option<u32>,
        state_metadata: Option<String>,
        foundry_counter: Option<u32>,
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<Feature>>,
        immutable_features: Option<Vec<Feature>>,
    },
    /// Build a BasicOutput.
    /// Expected response: [`Output`](crate::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildBasicOutput {
        // If not provided, minimum storage deposit will be used
        #[serde(default, with = "option_string")]
        amount: Option<u64>,
        // TODO: Determine if `default` is wanted here
        #[serde(default, with = "string")]
        mana: u64,
        native_tokens: Option<Vec<NativeToken>>,
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<Feature>>,
    },
    /// Build a FoundryOutput.
    /// Expected response: [`Output`](crate::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildFoundryOutput {
        // If not provided, minimum storage deposit will be used
        #[serde(default, with = "option_string")]
        amount: Option<u64>,
        native_tokens: Option<Vec<NativeToken>>,
        serial_number: u32,
        token_scheme: TokenScheme,
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<Feature>>,
        immutable_features: Option<Vec<Feature>>,
    },
    /// Build an NftOutput.
    /// Expected response: [`Output`](crate::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildNftOutput {
        // If not provided, minimum storage deposit will be used
        #[serde(default, with = "option_string")]
        amount: Option<u64>,
        // TODO: Determine if `default` is wanted here
        #[serde(default, with = "string")]
        mana: u64,
        native_tokens: Option<Vec<NativeToken>>,
        nft_id: NftId,
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<Feature>>,
        immutable_features: Option<Vec<Feature>>,
    },
    /// Removes all listeners for the provided topics.
    /// Expected response: [`Ok`](crate::Response::Ok)
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    ClearListeners {
        /// Topics for which listeners should be removed.
        topics: Vec<Topic>,
    },
    /// Get a node candidate from the healthy node pool.
    GetNode,
    /// Gets the network related information such as network_id.
    GetNetworkInfo,
    /// Gets the network id of the node we're connecting to.
    GetNetworkId,
    /// Returns the bech32_hrp
    GetBech32Hrp,
    /// Returns the protocol parameters
    GetProtocolParameters,
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
    /// Fetch account output IDs
    #[serde(rename_all = "camelCase")]
    AccountOutputIds {
        /// Query parameters for output requests
        query_parameters: Vec<QueryParameter>,
    },
    /// Fetch account output ID
    #[serde(rename_all = "camelCase")]
    AccountOutputId {
        /// Account id
        account_id: AccountId,
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
    /// Function to find inputs from addresses for a provided amount (useful for offline signing)
    FindInputs {
        /// Addresses
        addresses: Vec<Bech32Address>,
        /// Amount
        amount: u64,
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
    /// Transforms an account id to a bech32 encoded address
    #[serde(rename_all = "camelCase")]
    AccountIdToBech32 {
        /// Account ID
        account_id: AccountId,
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
    /// Returns a block ID from a block
    BlockId {
        /// Block
        block: BlockDto,
    },
}
