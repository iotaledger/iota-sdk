// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derivative::Derivative;
#[cfg(feature = "mqtt")]
use iota_sdk::client::mqtt::Topic;
use iota_sdk::{
    client::{
        node_api::indexer::query_parameters::{
            AccountOutputQueryParameters, AnchorOutputQueryParameters, BasicOutputQueryParameters,
            DelegationOutputQueryParameters, FoundryOutputQueryParameters, NftOutputQueryParameters,
            OutputQueryParameters,
        },
        node_manager::node::NodeAuth,
    },
    types::block::{
        address::{Address, Bech32Address, Hrp},
        output::{
            feature::Feature, unlock_condition::UnlockCondition, AccountId, AnchorId, DelegationId, FoundryId, NftId,
            Output, OutputId, TokenScheme,
        },
        payload::{dto::PayloadDto, signed_transaction::TransactionId},
        slot::{EpochIndex, SlotCommitmentId, SlotIndex},
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
        // If not provided, minimum amount will be used
        #[serde(default, with = "option_string")]
        amount: Option<u64>,
        #[serde(default, with = "string")]
        mana: u64,
        account_id: AccountId,
        foundry_counter: Option<u32>,
        unlock_conditions: Vec<UnlockCondition>,
        features: Option<Vec<Feature>>,
        immutable_features: Option<Vec<Feature>>,
    },
    /// Build a BasicOutput.
    /// Expected response: [`Output`](crate::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildBasicOutput {
        // If not provided, minimum amount will be used
        #[serde(default, with = "option_string")]
        amount: Option<u64>,
        #[serde(default, with = "string")]
        mana: u64,
        unlock_conditions: Vec<UnlockCondition>,
        features: Option<Vec<Feature>>,
    },
    /// Build a FoundryOutput.
    /// Expected response: [`Output`](crate::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildFoundryOutput {
        // If not provided, minimum amount will be used
        #[serde(default, with = "option_string")]
        amount: Option<u64>,
        serial_number: u32,
        token_scheme: TokenScheme,
        unlock_conditions: Vec<UnlockCondition>,
        features: Option<Vec<Feature>>,
        immutable_features: Option<Vec<Feature>>,
    },
    /// Build an NftOutput.
    /// Expected response: [`Output`](crate::Response::Output)
    #[allow(missing_docs)]
    #[serde(rename_all = "camelCase")]
    BuildNftOutput {
        // If not provided, minimum amount will be used
        #[serde(default, with = "option_string")]
        amount: Option<u64>,
        #[serde(default, with = "string")]
        mana: u64,
        nft_id: NftId,
        unlock_conditions: Vec<UnlockCondition>,
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
    #[serde(rename_all = "camelCase")]
    BuildBasicBlock {
        /// The issuer's ID.
        issuer_id: AccountId,
        /// The block payload.
        #[serde(default)]
        payload: Option<PayloadDto>,
    },
    //////////////////////////////////////////////////////////////////////
    // Node core API
    //////////////////////////////////////////////////////////////////////
    /// Returns the health of the node.
    GetHealth {
        /// Url
        url: String,
    },
    /// Returns the available API route groups of the node.
    GetRoutes,
    /// Returns general information about a node given its URL and - if required - the authentication data.
    GetInfo {
        /// Url
        url: String,
        /// Node authentication
        auth: Option<NodeAuth>,
    },
    /// Returns general information about the node together with its URL.
    GetNodeInfo,
    /// Returns network metrics.
    GetNetworkMetrics,
    /// Check the readiness of the node to issue a new block, the reference mana cost based on the rate setter and
    /// current network congestion, and the block issuance credits of the requested account.
    #[serde(rename_all = "camelCase")]
    GetAccountCongestion {
        /// The Account ID of the account.
        account_id: AccountId,
        /// Work score to check readiness for, max work score is assumed if not provided.
        work_score: Option<u32>,
    },
    /// Returns all the available Mana rewards of an account or delegation output in the returned range of epochs.
    #[serde(rename_all = "camelCase")]
    GetOutputManaRewards {
        /// Output ID of an account or delegation output.
        output_id: OutputId,
        /// A client can specify a slot index explicitly, which should be equal to the slot it uses as the commitment
        /// input for the claiming transaction to ensure the node calculates the rewards identically as during
        /// transaction execution. Rewards are decayed up to the epoch corresponding to the given slotIndex +
        /// MinCommittableAge. For a Delegation Output in delegating state (i.e. when Delegation ID is zeroed), that
        /// epoch - 1 is also used as the last epoch for which rewards are fetched. Callers that do not build
        /// transactions with the returned values may omit this value in which case it defaults to the latest committed
        /// slot, which is good enough to, e.g. display estimated rewards to users.
        slot: Option<SlotIndex>,
    },
    /// Returns information of all registered validators and if they are active, ordered by their holding stake.
    #[serde(rename_all = "camelCase")]
    GetValidators {
        /// The page number of validators.
        page_size: Option<u32>,
        /// Starts the search from the cursor (requested slot index+start index).
        cursor: Option<String>,
    },
    /// Return information about a validator.
    #[serde(rename_all = "camelCase")]
    GetValidator {
        /// The Account ID of the account.
        account_id: AccountId,
    },
    /// Return the information of committee members at the given epoch index. If epoch index is not provided, the
    /// current committee members are returned.
    GetCommittee {
        /// The epoch index to query.
        epoch: Option<EpochIndex>,
    },
    /// Get issuance
    GetIssuance,
    /// Post block (JSON)
    PostBlock {
        /// Block
        block: BlockDto,
    },
    /// Post block (raw)
    #[serde(rename_all = "camelCase")]
    PostBlockRaw {
        /// Block as raw bytes
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
    /// Get a block with its metadata
    #[serde(rename_all = "camelCase")]
    GetBlockWithMetadata {
        /// Block ID
        block_id: BlockId,
    },
    /// Get block as raw bytes.
    #[serde(rename_all = "camelCase")]
    GetBlockRaw {
        /// Block ID
        block_id: BlockId,
    },
    /// Get output with its output ID proof.
    #[serde(rename_all = "camelCase")]
    GetOutput {
        /// Output ID
        output_id: OutputId,
    },
    /// Get output as raw bytes.
    #[serde(rename_all = "camelCase")]
    GetOutputRaw {
        /// Output ID
        output_id: OutputId,
    },
    /// Get output metadata.
    #[serde(rename_all = "camelCase")]
    GetOutputMetadata {
        /// Output ID
        output_id: OutputId,
    },
    /// Get output with its metadata including the output ID proof.
    #[serde(rename_all = "camelCase")]
    GetOutputWithMetadata {
        /// Output ID
        output_id: OutputId,
    },
    /// Returns the included block of the transaction.
    #[serde(rename_all = "camelCase")]
    GetIncludedBlock {
        /// Transaction ID
        transaction_id: TransactionId,
    },
    /// Returns the raw bytes of the included block of a transaction.
    #[serde(rename_all = "camelCase")]
    GetIncludedBlockRaw {
        /// Transaction ID
        transaction_id: TransactionId,
    },
    /// Returns the included block metadata of the transaction.
    #[serde(rename_all = "camelCase")]
    GetIncludedBlockMetadata {
        /// Transaction ID
        transaction_id: TransactionId,
    },
    /// Find the metadata of a transaction.
    #[serde(rename_all = "camelCase")]
    GetTransactionMetadata {
        /// Transaction ID
        transaction_id: TransactionId,
    },
    /// Look up a commitment by a given commitment ID.
    #[serde(rename_all = "camelCase")]
    GetCommitment {
        /// Commitment ID of the commitment to look up.
        commitment_id: SlotCommitmentId,
    },
    /// Look up a commitment by a given commitment ID and return its raw bytes.
    #[serde(rename_all = "camelCase")]
    GetCommitmentRaw {
        /// Commitment ID of the commitment to look up.
        commitment_id: SlotCommitmentId,
    },
    /// Get all UTXO changes of a given slot by Commitment ID.
    #[serde(rename_all = "camelCase")]
    GetUtxoChanges {
        /// Commitment ID of the commitment to look up.
        commitment_id: SlotCommitmentId,
    },
    /// Get all full UTXO changes of a given slot by Commitment ID.
    #[serde(rename_all = "camelCase")]
    GetUtxoChangesFull {
        /// Commitment ID of the commitment to look up.
        commitment_id: SlotCommitmentId,
    },
    /// Look up a commitment by a given commitment index.
    GetCommitmentBySlot {
        /// Index of the commitment to look up.
        slot: SlotIndex,
    },
    /// Look up a commitment by a given commitment index and return its raw bytes.
    GetCommitmentBySlotRaw {
        /// Index of the commitment to look up.
        slot: SlotIndex,
    },
    /// Get all UTXO changes of a given slot by commitment index.
    GetUtxoChangesBySlot {
        /// Index of the commitment to look up.
        slot: SlotIndex,
    },
    /// Get all full UTXO changes of a given slot by commitment index.
    GetUtxoChangesFullBySlot {
        /// Index of the commitment to look up.
        slot: SlotIndex,
    },

    //////////////////////////////////////////////////////////////////////
    // Node indexer API
    //////////////////////////////////////////////////////////////////////
    /// Fetch account/anchor/basic/delegation/NFT/foundry output IDs
    #[serde(rename_all = "camelCase")]
    OutputIds {
        /// Query parameters for output requests
        query_parameters: OutputQueryParameters,
    },
    /// Fetch basic output IDs
    #[serde(rename_all = "camelCase")]
    BasicOutputIds {
        /// Query parameters for output requests
        query_parameters: BasicOutputQueryParameters,
    },
    /// Fetch account output IDs
    #[serde(rename_all = "camelCase")]
    AccountOutputIds {
        /// Query parameters for output requests
        query_parameters: AccountOutputQueryParameters,
    },
    /// Fetch account output ID
    #[serde(rename_all = "camelCase")]
    AccountOutputId {
        /// Account id
        account_id: AccountId,
    },
    /// Fetch anchor output IDs
    #[serde(rename_all = "camelCase")]
    AnchorOutputIds {
        /// Query parameters for output requests
        query_parameters: AnchorOutputQueryParameters,
    },
    /// Fetch anchor output ID
    #[serde(rename_all = "camelCase")]
    AnchorOutputId {
        /// Anchor id
        anchor_id: AnchorId,
    },
    /// Fetch delegation output IDs
    #[serde(rename_all = "camelCase")]
    DelegationOutputIds {
        /// Query parameters for output requests
        query_parameters: DelegationOutputQueryParameters,
    },
    /// Fetch delegation output ID
    #[serde(rename_all = "camelCase")]
    DelegationOutputId {
        /// Delegation id
        delegation_id: DelegationId,
    },
    /// Fetch foundry Output IDs
    #[serde(rename_all = "camelCase")]
    FoundryOutputIds {
        /// Query parameters for output requests
        query_parameters: FoundryOutputQueryParameters,
    },
    /// Fetch foundry Output ID
    #[serde(rename_all = "camelCase")]
    FoundryOutputId {
        /// Foundry ID
        foundry_id: FoundryId,
    },
    /// Fetch NFT output IDs
    #[serde(rename_all = "camelCase")]
    NftOutputIds {
        /// Query parameters for output requests
        query_parameters: NftOutputQueryParameters,
    },
    /// Fetch NFT output ID
    #[serde(rename_all = "camelCase")]
    NftOutputId {
        /// NFT ID
        nft_id: NftId,
    },

    //////////////////////////////////////////////////////////////////////
    // High level API
    //////////////////////////////////////////////////////////////////////
    /// Fetch outputs with associated output ID proofs from provided OutputIds (requests are sent in parallel)
    #[serde(rename_all = "camelCase")]
    GetOutputs {
        /// Output IDs
        output_ids: Vec<OutputId>,
    },
    /// Try to get outputs with associated output ID proofs from provided OutputIds (requests are sent in parallel and
    /// errors are ignored, can be useful for spent outputs)
    #[serde(rename_all = "camelCase")]
    GetOutputsIgnoreNotFound {
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
    /// Converts an address to its bech32 representation
    #[serde(rename_all = "camelCase")]
    AddressToBech32 { address: Address, bech32_hrp: Option<Hrp> },
    /// Calculate the minimum required amount for an output.
    /// Expected response:
    /// [`Amount`](crate::Response::Amount)
    ComputeMinimumOutputAmount { output: Output },
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
