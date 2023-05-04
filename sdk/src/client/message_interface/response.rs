// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(not(target_family = "wasm"))]
use std::collections::HashSet;

use serde::Serialize;

#[cfg(feature = "ledger_nano")]
use crate::client::secret::LedgerNanoStatus;
use crate::{
    client::{api::PreparedTransactionDataDto, node_manager::node::Node, Error, NetworkInfoDto, NodeInfoWrapper},
    types::{
        api::{
            core::{
                dto::PeerDto,
                response::{
                    BlockMetadataResponse, InfoResponse as NodeInfo, OutputWithMetadataResponse,
                    UtxoChangesResponse as MilestoneUTXOChanges,
                },
            },
            plugins::indexer::OutputIdsResponse,
        },
        block::{
            address::dto::AddressDto,
            input::dto::UtxoInputDto,
            output::{
                dto::{OutputDto, OutputMetadataDto},
                AliasId, FoundryId, NftId, OutputId,
            },
            payload::{
                dto::{MilestonePayloadDto, PayloadDto},
                transaction::TransactionId,
            },
            protocol::dto::ProtocolParametersDto,
            signature::dto::Ed25519SignatureDto,
            unlock::dto::UnlockDto,
            BlockDto, BlockId,
        },
    },
};

/// The response message.
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Response {
    /// Response for:
    /// - [`BuildAliasOutput`](crate::client::message_interface::Message::BuildAliasOutput)
    /// - [`BuildBasicOutput`](crate::client::message_interface::Message::BuildBasicOutput)
    /// - [`BuildFoundryOutput`](crate::client::message_interface::Message::BuildFoundryOutput)
    /// - [`BuildNftOutput`](crate::client::message_interface::Message::BuildNftOutput)
    BuiltOutput(OutputDto),
    /// Response for:
    /// - [`GetLocalPow`](crate::client::message_interface::Message::GetLocalPow)
    /// - [`GetFallbackToLocalPow`](crate::client::message_interface::Message::GetFallbackToLocalPow)
    /// - [`VerifyEd25519Signature`](crate::client::message_interface::Message::VerifyEd25519Signature)
    /// - [`GetHealth`](crate::client::message_interface::Message::GetHealth)
    /// - [`IsAddressValid`](crate::client::message_interface::Message::IsAddressValid)
    Bool(bool),
    /// Response for:
    /// - [`GenerateAddresses`](crate::client::message_interface::Message::GenerateAddresses)
    GeneratedAddresses(Vec<String>),
    /// Response for:
    /// - [`GetNode`](crate::client::message_interface::Message::GetNode)
    Node(Node),
    /// Response for:
    /// - [`GetNetworkInfo`](crate::client::message_interface::Message::GetNetworkInfo)
    NetworkInfo(NetworkInfoDto),
    /// Response for:
    /// - [`GetNetworkId`](crate::client::message_interface::Message::GetNetworkId)
    NetworkId(u64),
    /// Response for:
    /// - [`GetBech32Hrp`](crate::client::message_interface::Message::GetBech32Hrp)
    Bech32Hrp(String),
    /// Response for:
    /// - [`GetMinPowScore`](crate::client::message_interface::Message::GetMinPowScore)
    MinPowScore(u32),
    /// Response for:
    /// - [`GetTipsInterval`](crate::client::message_interface::Message::GetTipsInterval)
    TipsInterval(u64),
    /// Response for:
    /// - [`GetProtocolParameters`](crate::client::message_interface::Message::GetProtocolParameters)
    ProtocolParameters(ProtocolParametersDto),
    /// Response for:
    /// - [`GetLedgerNanoStatus`](crate::client::message_interface::Message::GetLedgerNanoStatus)
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerNanoStatus(LedgerNanoStatus),
    /// Response for:
    /// - [`PrepareTransaction`](crate::client::message_interface::Message::PrepareTransaction)
    PreparedTransactionData(PreparedTransactionDataDto),
    /// Response for:
    /// - [`SignTransaction`](crate::client::message_interface::Message::SignTransaction)
    SignedTransaction(PayloadDto),
    /// Response for:
    /// - [`SignatureUnlock`](crate::client::message_interface::Message::SignatureUnlock)
    SignatureUnlock(UnlockDto),
    /// Response for:
    /// - [`SignEd25519`](crate::client::message_interface::Message::SignEd25519)
    Ed25519Signature(Ed25519SignatureDto),
    /// Response for:
    /// - [`UnhealthyNodes`](crate::client::message_interface::Message::UnhealthyNodes)
    #[cfg(not(target_family = "wasm"))]
    UnhealthyNodes(HashSet<Node>),
    /// Response for:
    /// - [`GetNodeInfo`](crate::client::message_interface::Message::GetNodeInfo)
    NodeInfo(NodeInfo),
    /// Response for:
    /// - [`GetInfo`](crate::client::message_interface::Message::GetInfo)
    Info(NodeInfoWrapper),
    /// Response for:
    /// - [`GetPeers`](crate::client::message_interface::Message::GetPeers)
    Peers(Vec<PeerDto>),
    /// Response for:
    /// - [`GetTips`](crate::client::message_interface::Message::GetTips)
    Tips(Vec<BlockId>),
    /// Response for:
    /// - [`GetBlock`](crate::client::message_interface::Message::GetBlock)
    /// - [`GetIncludedBlock`](crate::client::message_interface::Message::GetIncludedBlock)
    Block(BlockDto),
    /// Response for:
    /// - [`BuildAndPostBlock`](crate::client::message_interface::Message::BuildAndPostBlock)
    /// - [`PostBlockPayload`](crate::client::message_interface::Message::PostBlockPayload)
    /// - [`Retry`](crate::client::message_interface::Message::Retry)
    BlockIdWithBlock(BlockId, BlockDto),
    /// Response for:
    /// - [`GetBlockMetadata`](crate::client::message_interface::Message::GetBlockMetadata)
    BlockMetadata(BlockMetadataResponse),
    /// Response for:
    /// - [`GetBlockRaw`](crate::client::message_interface::Message::GetBlockRaw)
    BlockRaw(Vec<u8>),
    /// Response for:
    /// - [`GetOutput`](crate::client::message_interface::Message::GetOutput)
    Output(OutputWithMetadataResponse),
    /// Response for:
    /// - [`GetOutputMetadata`](crate::client::message_interface::Message::GetOutputMetadata)
    OutputMetadata(OutputMetadataDto),
    /// Response for:
    /// - [`GetOutputs`](crate::client::message_interface::Message::GetOutputs)
    /// - [`TryGetOutputs`](crate::client::message_interface::Message::TryGetOutputs)
    /// - [`FindOutputs`](crate::client::message_interface::Message::FindOutputs)
    Outputs(Vec<OutputWithMetadataResponse>),
    /// Response for:
    /// - [`GetMilestoneById`](crate::client::message_interface::Message::GetMilestoneById)
    /// - [`GetMilestoneByIndex`](crate::client::message_interface::Message::GetMilestoneByIndex)
    Milestone(MilestonePayloadDto),
    /// Response for:
    /// - [`GetMilestoneByIdRaw`](crate::client::message_interface::Message::GetMilestoneByIdRaw)
    /// - [`GetMilestoneByIndexRaw`](crate::client::message_interface::Message::GetMilestoneByIndexRaw)
    MilestoneRaw(Vec<u8>),
    /// Response for:
    /// - [`GetUtxoChangesById`](crate::client::message_interface::Message::GetUtxoChangesById)
    /// - [`GetUtxoChangesByIndex`](crate::client::message_interface::Message::GetUtxoChangesByIndex)
    MilestoneUtxoChanges(MilestoneUTXOChanges),
    /// Response for:
    /// - [`AliasOutputId`](crate::client::message_interface::Message::AliasOutputId)
    /// - [`NftOutputId`](crate::client::message_interface::Message::NftOutputId)
    /// - [`FoundryOutputId`](crate::client::message_interface::Message::FoundryOutputId)
    OutputId(OutputId),
    /// Response for:
    /// - [`BasicOutputIds`](crate::client::message_interface::Message::BasicOutputIds)
    /// - [`AliasOutputIds`](crate::client::message_interface::Message::AliasOutputIds)
    /// - [`NftOutputIds`](crate::client::message_interface::Message::NftOutputIds)
    /// - [`FoundryOutputIds`](crate::client::message_interface::Message::FoundryOutputIds)
    OutputIdsResponse(OutputIdsResponse),
    /// Response for:
    /// - [`FindBlocks`](crate::client::message_interface::Message::FindBlocks)
    Blocks(Vec<BlockDto>),
    /// Response for:
    /// - [`RetryUntilIncluded`](crate::client::message_interface::Message::RetryUntilIncluded)
    RetryUntilIncludedSuccessful(Vec<(BlockId, BlockDto)>),
    /// Response for:
    /// - [`ConsolidateFunds`](crate::client::message_interface::Message::ConsolidateFunds)
    ConsolidatedFunds(String),
    /// Response for:
    /// - [`FindInputs`](crate::client::message_interface::Message::FindInputs)
    Inputs(Vec<UtxoInputDto>),
    /// Response for:
    /// - [`Reattach`](crate::client::message_interface::Message::Reattach)
    /// - [`ReattachUnchecked`](crate::client::message_interface::Message::ReattachUnchecked)
    Reattached((BlockId, BlockDto)),
    /// Response for:
    /// - [`Promote`](crate::client::message_interface::Message::Promote)
    /// - [`PromoteUnchecked`](crate::client::message_interface::Message::PromoteUnchecked)
    Promoted((BlockId, BlockDto)),
    /// Response for:
    /// - [`Bech32ToHex`](crate::client::message_interface::Message::Bech32ToHex)
    Bech32ToHex(String),
    /// Response for:
    /// - [`AliasIdToBech32`](crate::client::message_interface::Message::AliasIdToBech32)
    /// - [`HexPublicKeyToBech32Address`](crate::client::message_interface::Message::HexPublicKeyToBech32Address)
    /// - [`HexToBech32`](crate::client::message_interface::Message::HexToBech32)
    /// - [`NftIdToBech32`](crate::client::message_interface::Message::NftIdToBech32)
    Bech32Address(String),
    /// Response for:
    /// - [`ParseBech32Address`](crate::client::message_interface::Message::ParseBech32Address)
    ParsedBech32Address(AddressDto),
    /// Response for:
    /// - [`GenerateMnemonic`](crate::client::message_interface::Message::GenerateMnemonic)
    GeneratedMnemonic(String),
    /// Response for:
    /// - [`MnemonicToHexSeed`](crate::client::message_interface::Message::MnemonicToHexSeed)
    MnemonicHexSeed(String),
    /// Response for:
    /// - [`BlockId`](crate::client::message_interface::Message::BlockId)
    /// - [`PostBlock`](crate::client::message_interface::Message::PostBlock)
    /// - [`PostBlockRaw`](crate::client::message_interface::Message::PostBlockRaw)
    BlockId(BlockId),
    /// Response for:
    /// - [`TransactionId`](crate::client::message_interface::Message::TransactionId)
    TransactionId(TransactionId),
    /// Response for:
    /// - [`ComputeAliasId`](crate::client::message_interface::Message::ComputeAliasId)
    AliasId(AliasId),
    /// Response for:
    /// - [`ComputeNftId`](crate::client::message_interface::Message::ComputeNftId)
    NftId(NftId),
    /// Response for:
    /// - [`ComputeFoundryId`](crate::client::message_interface::Message::ComputeFoundryId)
    FoundryId(FoundryId),
    /// Response for:
    /// - [`Faucet`](crate::client::message_interface::Message::Faucet)
    Faucet(String),
    /// Response for:
    /// - [`HashTransactionEssence`](crate::client::message_interface::Message::HashTransactionEssence)
    TransactionEssenceHash(String),
    /// Response for:
    /// - [`ClearListeners`](crate::client::message_interface::Message::ClearListeners)
    /// - [`StoreMnemonic`](crate::client::message_interface::Message::StoreMnemonic)
    Ok,
    /// Response for any method that returns an error.
    Error(Error),
    /// Response for any method that panics.
    Panic(String),
}
