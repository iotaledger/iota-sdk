// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(not(target_family = "wasm"))]
use std::collections::HashSet;

use derivative::Derivative;
#[cfg(feature = "ledger_nano")]
use iota_sdk::client::secret::LedgerNanoStatus;
use iota_sdk::{
    client::{
        api::{PreparedTransactionDataDto, SignedTransactionDataDto},
        node_manager::node::Node,
        NetworkInfo, NodeInfoWrapper,
    },
    types::{
        api::{
            core::{
                BlockMetadataResponse, BlockWithMetadataResponse, CommitteeResponse, CongestionResponse,
                InfoResponse as NodeInfo, IssuanceBlockHeaderResponse, ManaRewardsResponse, OutputWithMetadataResponse,
                TransactionMetadataResponse, UtxoChangesFullResponse, UtxoChangesResponse, ValidatorResponse,
                ValidatorsResponse,
            },
            plugins::indexer::OutputIdsResponse,
        },
        block::{
            address::{Address, Bech32Address, Hrp},
            input::UtxoInput,
            output::{AccountId, FoundryId, NftId, Output, OutputId, OutputMetadata, OutputWithMetadata, TokenId},
            payload::{dto::SignedTransactionPayloadDto, signed_transaction::TransactionId},
            protocol::ProtocolParameters,
            semantic::TransactionFailureReason,
            signature::Ed25519Signature,
            slot::{SlotCommitment, SlotCommitmentId},
            unlock::Unlock,
            BlockDto, BlockId, UnsignedBlockDto,
        },
    },
    utils::serde::string,
    wallet::{
        types::{Balance, OutputData, TransactionWithMetadataDto},
        PreparedCreateNativeTokenTransactionDto,
    },
};
use serde::Serialize;
#[cfg(feature = "participation")]
use {
    iota_sdk::types::api::plugins::participation::types::{ParticipationEventId, ParticipationEventStatus},
    iota_sdk::wallet::{ParticipationEventWithNodes, ParticipationOverview},
    std::collections::HashMap,
};

use crate::{error::Error, OmittedDebug};

/// The response message.
#[derive(Serialize, Derivative)]
#[derivative(Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
#[non_exhaustive]
pub enum Response {
    /// Response for:
    /// - [`GenerateEd25519Addresses`](crate::method::SecretManagerMethod::GenerateEd25519Addresses)
    GeneratedEd25519Addresses(Vec<Bech32Address>),
    /// Response for:
    /// - [`GenerateEvmAddresses`](crate::method::SecretManagerMethod::GenerateEvmAddresses)
    GeneratedEvmAddresses(Vec<String>),
    /// Response for:
    /// - [`GetNode`](crate::method::ClientMethod::GetNode)
    Node(Node),
    /// Response for:
    /// - [`GetNetworkInfo`](crate::method::ClientMethod::GetNetworkInfo)
    NetworkInfo(NetworkInfo),
    /// Response for:
    /// - [`GetNetworkId`](crate::method::ClientMethod::GetNetworkId)
    NetworkId(String),
    /// Response for:
    /// - [`GetBech32Hrp`](crate::method::ClientMethod::GetBech32Hrp)
    Bech32Hrp(Hrp),
    /// Response for:
    /// - [`GetProtocolParameters`](crate::method::ClientMethod::GetProtocolParameters)
    ProtocolParameters(ProtocolParameters),
    /// Response for:
    /// - [`SignTransaction`](crate::method::SecretManagerMethod::SignTransaction)
    SignedTransaction(SignedTransactionPayloadDto),
    /// Response for:
    /// - [`SignatureUnlock`](crate::method::SecretManagerMethod::SignatureUnlock)
    SignatureUnlock(Unlock),
    /// Response for:
    /// - [`SignEd25519`](crate::method::SecretManagerMethod::SignEd25519)
    Ed25519Signature(Ed25519Signature),
    /// Response for:
    /// - [`SignSecp256k1Ecdsa`](crate::method::SecretManagerMethod::SignSecp256k1Ecdsa)
    #[serde(rename_all = "camelCase")]
    Secp256k1EcdsaSignature { public_key: String, signature: String },
    /// Response for:
    /// - [`UnhealthyNodes`](crate::method::ClientMethod::UnhealthyNodes)
    #[cfg(not(target_family = "wasm"))]
    UnhealthyNodes(HashSet<Node>),
    /// Response for:
    /// - [`GetNodeInfo`](crate::method::ClientMethod::GetNodeInfo)
    NodeInfo(NodeInfo),
    /// Response for:
    /// - [`GetInfo`](crate::method::ClientMethod::GetInfo)
    Info(NodeInfoWrapper),
    /// Response for:
    /// - [`GetAccountCongestion`](crate::method::ClientMethod::GetAccountCongestion)
    Congestion(CongestionResponse),
    /// Response for:
    /// - [`GetRewards`](crate::method::ClientMethod::GetRewards)
    ManaRewards(ManaRewardsResponse),
    /// Response for:
    /// - [`GetValidators`](crate::method::ClientMethod::GetValidators)
    Validators(ValidatorsResponse),
    /// Response for:
    /// - [`GetValidator`](crate::method::ClientMethod::GetValidator)
    Validator(ValidatorResponse),
    /// Response for:
    /// - [`GetCommittee`](crate::method::ClientMethod::GetCommittee)
    Committee(CommitteeResponse),
    /// Response for:
    /// - [`GetIssuance`](crate::method::ClientMethod::GetIssuance)
    Issuance(IssuanceBlockHeaderResponse),
    /// Response for:
    /// - [`BuildBasicBlock`](crate::method::ClientMethod::BuildBasicBlock)
    UnsignedBlock(UnsignedBlockDto),
    /// Response for:
    /// - [`GetBlock`](crate::method::ClientMethod::GetBlock)
    /// - [`GetIncludedBlock`](crate::method::ClientMethod::GetIncludedBlock)
    /// - [`SignBlock`](crate::method::SecretManagerMethod::SignBlock)
    Block(BlockDto),
    /// Response for:
    /// - [`GetBlockMetadata`](crate::method::ClientMethod::GetBlockMetadata)
    BlockMetadata(BlockMetadataResponse),
    /// Response for:
    /// - [`GetTransactionMetadata`](crate::method::ClientMethod::GetTransactionMetadata)
    TransactionMetadata(TransactionMetadataResponse),
    /// Response for:
    /// - [`GetCommitment`](crate::method::ClientMethod::GetCommitment)
    /// - [`GetCommitmentByIndex`](crate::method::ClientMethod::GetCommitmentByIndex)
    SlotCommitment(SlotCommitment),
    /// Response for:
    /// - [`GetUtxoChanges`](crate::method::ClientMethod::GetUtxoChanges)
    /// - [`GetUtxoChangesByIndex`](crate::method::ClientMethod::GetUtxoChangesByIndex)
    UtxoChanges(UtxoChangesResponse),
    /// Response for:
    /// - [`GetUtxoChangesFull`](crate::method::ClientMethod::GetUtxoChangesFull)
    /// - [`GetUtxoChangesFullByIndex`](crate::method::ClientMethod::GetUtxoChangesFullByIndex)
    UtxoChangesFull(UtxoChangesFullResponse),
    /// Response for:
    /// - [`GetBlockWithMetadata`](crate::method::ClientMethod::GetBlockWithMetadata)
    BlockWithMetadata(BlockWithMetadataResponse),
    /// Response for:
    /// - [`GetBlockRaw`](crate::method::ClientMethod::GetBlockRaw)
    Raw(Vec<u8>),
    /// Response for:
    /// - [`GetOutput`](crate::method::ClientMethod::GetOutput)
    OutputWithMetadataResponse(OutputWithMetadataResponse),
    /// Response for:
    /// - [`GetOutputMetadata`](crate::method::ClientMethod::GetOutputMetadata)
    OutputMetadata(OutputMetadata),
    /// Response for:
    /// - [`GetOutputWithMetadata`](crate::method::ClientMethod::GetOutputWithMetadata)
    OutputWithMetadata(OutputWithMetadata),
    /// Response for:
    /// - [`GetOutputs`](crate::method::ClientMethod::GetOutputs)
    /// - [`GetOutputsIgnoreErrors`](crate::method::ClientMethod::GetOutputsIgnoreErrors)
    Outputs(Vec<OutputWithMetadataResponse>),
    /// Response for:
    /// - [`AccountOutputId`](crate::method::ClientMethod::AccountOutputId)
    /// - [`AnchorOutputId`](crate::method::ClientMethod::AnchorOutputId)
    /// - [`DelegationOutputId`](crate::method::ClientMethod::DelegationOutputId)
    /// - [`FoundryOutputId`](crate::method::ClientMethod::FoundryOutputId)
    /// - [`NftOutputId`](crate::method::ClientMethod::NftOutputId)
    OutputId(OutputId),
    /// Response for:
    /// - [`OutputIds`](crate::method::ClientMethod::OutputIds)
    /// - [`BasicOutputIds`](crate::method::ClientMethod::BasicOutputIds)
    /// - [`AccountOutputIds`](crate::method::ClientMethod::AccountOutputIds)
    /// - [`AnchorOutputIds`](crate::method::ClientMethod::AnchorOutputIds)
    /// - [`DelegationOutputIds`](crate::method::ClientMethod::DelegationOutputIds)
    /// - [`FoundryOutputIds`](crate::method::ClientMethod::FoundryOutputIds)
    /// - [`NftOutputIds`](crate::method::ClientMethod::NftOutputIds)
    OutputIdsResponse(OutputIdsResponse),
    /// Response for:
    /// - [`FindBlocks`](crate::method::ClientMethod::FindBlocks)
    Blocks(Vec<BlockDto>),
    /// Response for:
    /// - [`FindInputs`](crate::method::ClientMethod::FindInputs)
    Inputs(Vec<UtxoInput>),
    /// Response for:
    /// [`OutputIdToUtxoInput`](crate::method::UtilsMethod::OutputIdToUtxoInput)
    Input(UtxoInput),
    /// Response for:
    /// - [`Bech32ToHex`](crate::method::UtilsMethod::Bech32ToHex)
    Bech32ToHex(String),
    /// Response for:
    /// - [`ParseBech32Address`](crate::method::UtilsMethod::ParseBech32Address)
    ParsedBech32Address(Address),
    /// Response for:
    /// - [`MnemonicToHexSeed`](crate::method::UtilsMethod::MnemonicToHexSeed)
    MnemonicHexSeed(#[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))] String),
    /// Response for:
    /// - [`ComputeTokenId`](crate::method::UtilsMethod::ComputeTokenId)
    TokenId(TokenId),
    /// Response for:
    /// - [`TransactionId`](crate::method::UtilsMethod::TransactionId)
    TransactionId(TransactionId),
    /// Response for:
    /// - [`ComputeAccountId`](crate::method::UtilsMethod::ComputeAccountId)
    AccountId(AccountId),
    /// Response for:
    /// - [`ComputeNftId`](crate::method::UtilsMethod::ComputeNftId)
    NftId(NftId),
    /// Response for:
    /// - [`ComputeFoundryId`](crate::method::UtilsMethod::ComputeFoundryId)
    FoundryId(FoundryId),
    /// Response for:
    /// - [`TransactionSigningHash`](crate::method::UtilsMethod::TransactionSigningHash)
    Hash(String),
    /// Response for [`GetNodeInfo`](crate::method::ClientMethod::GetNodeInfo)
    NodeInfoWrapper(NodeInfoWrapper),
    /// Response for [`Bech32ToHex`](crate::method::UtilsMethod::Bech32ToHex)
    HexAddress(String),
    /// Response for [`OutputHexBytes`](crate::method::UtilsMethod::OutputHexBytes)
    HexBytes(String),
    /// Response for [`CallPluginRoute`](crate::method::ClientMethod::CallPluginRoute)
    CustomJson(serde_json::Value),
    /// Response for [`ComputeSlotCommitmentId`](crate::method::UtilsMethod::ComputeSlotCommitmentId)
    SlotCommitmentId(SlotCommitmentId),
    /// Response for [`VerifyTransactionSemantic`](crate::method::UtilsMethod::VerifyTransactionSemantic).
    TransactionFailureReason(Option<TransactionFailureReason>),

    // Responses in client and wallet
    /// Response for:
    /// - [`BuildAccountOutput`](crate::method::ClientMethod::BuildAccountOutput)
    /// - [`BuildBasicOutput`](crate::method::ClientMethod::BuildBasicOutput)
    /// - [`BuildFoundryOutput`](crate::method::ClientMethod::BuildFoundryOutput)
    /// - [`BuildNftOutput`](crate::method::ClientMethod::BuildNftOutput)
    /// - [`GetFoundryOutput`](crate::method::WalletMethod::GetFoundryOutput)
    /// - [`PrepareOutput`](crate::method::WalletMethod::PrepareOutput)
    Output(Output),
    /// Response for:
    /// - [`AccountIdToBech32`](crate::method::ClientMethod::AccountIdToBech32)
    /// - [`HexPublicKeyToBech32Address`](crate::method::ClientMethod::HexPublicKeyToBech32Address)
    /// - [`HexToBech32`](crate::method::ClientMethod::HexToBech32)
    /// - [`NftIdToBech32`](crate::method::ClientMethod::NftIdToBech32)
    /// - [`ImplicitAccountCreationAddress`](crate::method::WalletMethod::ImplicitAccountCreationAddress)
    Bech32Address(Bech32Address),
    /// - [`Faucet`](crate::method::ClientMethod::RequestFundsFromFaucet)
    Faucet(String),
    /// Response for:
    /// - [`GenerateMnemonic`](crate::method::UtilsMethod::GenerateMnemonic)
    GeneratedMnemonic(#[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))] String),
    /// Response for
    /// - [`GetLedgerNanoStatus`](crate::method::SecretManagerMethod::GetLedgerNanoStatus)
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerNanoStatus(LedgerNanoStatus),
    /// Response for:
    /// - [`BlockId`](crate::method::UtilsMethod::BlockId)
    /// - [`PostBlock`](crate::method::ClientMethod::PostBlock)
    /// - [`PostBlockRaw`](crate::method::ClientMethod::PostBlockRaw)
    /// - [`ReissueTransactionUntilIncluded`](crate::method::WalletMethod::ReissueTransactionUntilIncluded)
    BlockId(BlockId),
    /// Response for:
    /// - [`GetHealth`](crate::method::ClientMethod::GetHealth)
    /// - [`IsAddressValid`](crate::method::UtilsMethod::IsAddressValid)
    /// - [`VerifyEd25519Signature`](crate::method::UtilsMethod::VerifyEd25519Signature)
    /// - [`VerifySecp256k1EcdsaSignature`](crate::method::UtilsMethod::VerifySecp256k1EcdsaSignature)
    Bool(bool),
    /// Response for:
    /// - [`Backup`](crate::method::WalletMethod::Backup),
    /// - [`ClearListeners`](crate::method::WalletMethod::ClearListeners)
    /// - [`ClearStrongholdPassword`](crate::method::WalletMethod::ClearStrongholdPassword),
    /// - [`DeregisterParticipationEvent`](crate::method::WalletMethod::DeregisterParticipationEvent),
    /// - [`EmitTestEvent`](crate::method::WalletMethod::EmitTestEvent),
    /// - [`RestoreBackup`](crate::method::WalletMethod::RestoreBackup),
    /// - [`SetAlias`](crate::method::WalletMethod::SetAlias),
    /// - [`SetClientOptions`](crate::method::WalletMethod::SetClientOptions),
    /// - [`SetDefaultSyncOptions`](crate::method::WalletMethod::SetDefaultSyncOptions),
    /// - [`SetStrongholdPassword`](crate::method::WalletMethod::SetStrongholdPassword),
    /// - [`SetStrongholdPasswordClearInterval`](crate::method::WalletMethod::SetStrongholdPasswordClearInterval),
    /// - [`StartBackgroundSync`](crate::method::WalletMethod::StartBackgroundSync),
    /// - [`StoreMnemonic`](crate::method::WalletMethod::StoreMnemonic),
    /// - [`StopBackgroundSync`](crate::method::WalletMethod::StopBackgroundSync),
    Ok,
    /// Response for any method that returns an error.
    Error(Error),
    /// Response for any method that panics.
    Panic(String),

    // wallet responses
    /// Response for:
    /// - [`GetAddress`](crate::method::WalletMethod::GetAddress)
    Address(Bech32Address),
    /// Response for:
    /// - [`ClientMethod::ComputeMinimumOutputAmount`](crate::method::ClientMethod::ComputeMinimumOutputAmount)
    /// - [`UtilsMethod::ComputeMinimumOutputAmount`](crate::method::UtilsMethod::ComputeMinimumOutputAmount)
    OutputAmount(#[serde(with = "string")] u64),
    /// Response for:
    /// - [`ClaimableOutputs`](crate::method::WalletMethod::ClaimableOutputs)
    OutputIds(Vec<OutputId>),
    /// Response for:
    /// - [`GetOutput`](crate::method::WalletMethod::GetOutput)
    OutputData(Option<Box<OutputData>>),
    /// Response for:
    /// - [`Outputs`](crate::method::WalletMethod::Outputs),
    /// - [`UnspentOutputs`](crate::method::WalletMethod::UnspentOutputs)
    OutputsData(Vec<OutputData>),
    /// Response for:
    /// - [`PrepareBurn`](crate::method::WalletMethod::PrepareBurn),
    /// - [`PrepareClaimOutputs`](crate::method::WalletMethod::PrepareClaimOutputs)
    /// - [`PrepareConsolidateOutputs`](crate::method::WalletMethod::PrepareConsolidateOutputs)
    /// - [`PrepareCreateAccountOutput`](crate::method::WalletMethod::PrepareCreateAccountOutput)
    /// - [`PrepareDecreaseVotingPower`](crate::method::WalletMethod::PrepareDecreaseVotingPower)
    /// - [`PrepareIncreaseVotingPower`](crate::method::WalletMethod::PrepareIncreaseVotingPower)
    /// - [`PrepareMeltNativeToken`](crate::method::WalletMethod::PrepareMeltNativeToken)
    /// - [`PrepareMintNativeToken`](crate::method::WalletMethod::PrepareMintNativeToken),
    /// - [`PrepareMintNfts`](crate::method::WalletMethod::PrepareMintNfts),
    /// - [`PrepareSend`](crate::method::WalletMethod::PrepareSend),
    /// - [`PrepareSendNativeTokens`](crate::method::WalletMethod::PrepareSendNativeTokens),
    /// - [`PrepareSendNft`](crate::method::WalletMethod::PrepareSendNft),
    /// - [`PrepareStopParticipating`](crate::method::WalletMethod::PrepareStopParticipating)
    /// - [`PrepareTransaction`](crate::method::WalletMethod::PrepareTransaction)
    /// - [`PrepareVote`](crate::method::WalletMethod::PrepareVote)
    /// - [`PrepareImplicitAccountTransition`](crate::method::WalletMethod::PrepareImplicitAccountTransition)
    PreparedTransaction(PreparedTransactionDataDto),
    /// Response for:
    /// - [`PrepareCreateNativeToken`](crate::method::WalletMethod::PrepareCreateNativeToken),
    PreparedCreateNativeTokenTransaction(PreparedCreateNativeTokenTransactionDto),
    /// Response for:
    /// - [`GetIncomingTransaction`](crate::method::WalletMethod::GetIncomingTransaction)
    /// - [`GetTransaction`](crate::method::WalletMethod::GetTransaction),
    Transaction(Option<Box<TransactionWithMetadataDto>>),
    /// Response for:
    /// - [`IncomingTransactions`](crate::method::WalletMethod::IncomingTransactions)
    /// - [`PendingTransactions`](crate::method::WalletMethod::PendingTransactions),
    /// - [`Transactions`](crate::method::WalletMethod::Transactions),
    Transactions(Vec<TransactionWithMetadataDto>),
    /// Response for:
    /// - [`SignTransaction`](crate::method::WalletMethod::SignTransaction)
    SignedTransactionData(SignedTransactionDataDto),
    /// Response for:
    /// - [`GetBalance`](crate::method::WalletMethod::GetBalance),
    /// - [`Sync`](crate::method::WalletMethod::Sync)
    Balance(Balance),
    /// Response for:
    /// - [`ClaimOutputs`](crate::method::WalletMethod::ClaimOutputs)
    /// - [`Send`](crate::method::WalletMethod::Send)
    /// - [`SendOutputs`](crate::method::WalletMethod::SendOutputs)
    /// - [`SignAndSubmitTransaction`](crate::method::WalletMethod::SignAndSubmitTransaction)
    /// - [`SubmitAndStoreTransaction`](crate::method::WalletMethod::SubmitAndStoreTransaction)
    SentTransaction(TransactionWithMetadataDto),
    /// Response for:
    /// - [`GetParticipationEvent`](crate::method::WalletMethod::GetParticipationEvent)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEvent(Option<ParticipationEventWithNodes>),
    /// Response for:
    /// - [`GetParticipationEventIds`](crate::method::WalletMethod::GetParticipationEventIds)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEventIds(Vec<ParticipationEventId>),
    /// Response for:
    /// - [`GetParticipationEventStatus`](crate::method::WalletMethod::GetParticipationEventStatus)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEventStatus(ParticipationEventStatus),
    /// Response for:
    /// - [`GetParticipationEvents`](crate::method::WalletMethod::GetParticipationEvents)
    /// - [`RegisterParticipationEvents`](crate::method::WalletMethod::RegisterParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEvents(HashMap<ParticipationEventId, ParticipationEventWithNodes>),
    /// Response for:
    /// - [`GetVotingPower`](crate::method::WalletMethod::GetVotingPower)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    VotingPower(String),
    /// Response for:
    /// - [`GetParticipationOverview`](crate::method::WalletMethod::GetParticipationOverview)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationOverview(ParticipationOverview),
}
