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
                BlockMetadataResponse, InfoResponse as NodeInfo, IssuanceBlockHeaderResponse,
                OutputWithMetadataResponse, PeerResponse,
            },
            plugins::indexer::OutputIdsResponse,
        },
        block::{
            address::{Address, Bech32Address, Hrp},
            input::UtxoInput,
            output::{dto::OutputDto, AccountId, FoundryId, NftId, OutputId, OutputMetadata, TokenId},
            payload::{dto::SignedTransactionPayloadDto, signed_transaction::TransactionId},
            protocol::ProtocolParameters,
            signature::Ed25519Signature,
            slot::SlotCommitmentId,
            unlock::Unlock,
            BlockId, SignedBlockDto, UnsignedBlockDto,
        },
    },
    wallet::{
        core::WalletDataDto,
        types::{AddressWithUnspentOutputs, Balance, Bip44Address, OutputDataDto, TransactionWithMetadataDto},
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
    /// - [`GenerateEd25519Address`](crate::method::SecretManagerMethod::GenerateEd25519Addresses)
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
    /// - [`GetPeers`](crate::method::ClientMethod::GetPeers)
    Peers(Vec<PeerResponse>),
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
    SignedBlock(SignedBlockDto),
    /// Response for:
    /// - [`GetBlockMetadata`](crate::method::ClientMethod::GetBlockMetadata)
    BlockMetadata(BlockMetadataResponse),
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
    Blocks(Vec<SignedBlockDto>),
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
    /// - [`ComputeInputsCommitment`](crate::method::UtilsMethod::ComputeInputsCommitment)
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

    // Responses in client and wallet
    /// Response for:
    /// - [`BuildAccountOutput`](crate::method::ClientMethod::BuildAccountOutput)
    /// - [`BuildBasicOutput`](crate::method::ClientMethod::BuildBasicOutput)
    /// - [`BuildFoundryOutput`](crate::method::ClientMethod::BuildFoundryOutput)
    /// - [`BuildNftOutput`](crate::method::ClientMethod::BuildNftOutput)
    /// - [`GetFoundryOutput`](crate::method::WalletCommandMethod::GetFoundryOutput)
    /// - [`PrepareOutput`](crate::method::WalletCommandMethod::PrepareOutput)
    Output(OutputDto),
    /// Response for:
    /// - [`AccountIdToBech32`](crate::method::ClientMethod::AccountIdToBech32)
    /// - [`HexPublicKeyToBech32Address`](crate::method::ClientMethod::HexPublicKeyToBech32Address)
    /// - [`HexToBech32`](crate::method::ClientMethod::HexToBech32)
    /// - [`NftIdToBech32`](crate::method::ClientMethod::NftIdToBech32)
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
    /// - [`ReissueTransactionUntilIncluded`](crate::method::WalletCommandMethod::ReissueTransactionUntilIncluded)
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
    /// - [`DeregisterParticipationEvent`](crate::method::WalletCommandMethod::DeregisterParticipationEvent),
    /// - [`EmitTestEvent`](crate::method::WalletMethod::EmitTestEvent),
    /// - [`RestoreBackup`](crate::method::WalletMethod::RestoreBackup),
    /// - [`SetAlias`](crate::method::WalletCommandMethod::SetAlias),
    /// - [`SetClientOptions`](crate::method::WalletMethod::SetClientOptions),
    /// - [`SetDefaultSyncOptions`](crate::method::WalletCommandMethod::SetDefaultSyncOptions),
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
    /// - [`Address`](crate::method::WalletCommandMethod::GetAddress)
    Address(Bech32Address),
    /// Response for:
    /// - [`MinimumRequiredStorageDeposit`](crate::method::ClientMethod::MinimumRequiredStorageDeposit)
    /// - [`ComputeStorageDeposit`](crate::method::UtilsMethod::ComputeStorageDeposit)
    MinimumRequiredStorageDeposit(String),
    /// Response for:
    /// - [`ClaimableOutputs`](crate::method::WalletCommandMethod::ClaimableOutputs)
    OutputIds(Vec<OutputId>),
    /// Response for:
    /// - [`GetOutput`](crate::method::WalletCommandMethod::GetOutput)
    OutputData(Option<Box<OutputDataDto>>),
    /// Response for:
    /// - [`Outputs`](crate::method::WalletCommandMethod::Outputs),
    /// - [`UnspentOutputs`](crate::method::WalletCommandMethod::UnspentOutputs)
    OutputsData(Vec<OutputDataDto>),
    /// Response for:
    /// - [`PrepareBurn`](crate::method::WalletCommandMethod::PrepareBurn),
    /// - [`PrepareConsolidateOutputs`](crate::method::WalletCommandMethod::PrepareConsolidateOutputs)
    /// - [`PrepareCreateAccountOutput`](crate::method::WalletCommandMethod::PrepareCreateAccountOutput)
    /// - [`PrepareDecreaseVotingPower`](crate::method::WalletCommandMethod::PrepareDecreaseVotingPower)
    /// - [`PrepareIncreaseVotingPower`](crate::method::WalletCommandMethod::PrepareIncreaseVotingPower)
    /// - [`PrepareMeltNativeToken`](crate::method::WalletCommandMethod::PrepareMeltNativeToken)
    /// - [`PrepareMintNativeToken`](crate::method::WalletCommandMethod::PrepareMintNativeToken),
    /// - [`PrepareMintNfts`](crate::method::WalletCommandMethod::PrepareMintNfts),
    /// - [`PrepareSend`](crate::method::WalletCommandMethod::PrepareSend),
    /// - [`PrepareSendNativeTokens`](crate::method::WalletCommandMethod::PrepareSendNativeTokens),
    /// - [`PrepareSendNft`](crate::method::WalletCommandMethod::PrepareSendNft),
    /// - [`PrepareStopParticipating`](crate::method::WalletCommandMethod::PrepareStopParticipating)
    /// - [`PrepareTransaction`](crate::method::WalletCommandMethod::PrepareTransaction)
    /// - [`PrepareVote`](crate::method::WalletCommandMethod::PrepareVote)
    PreparedTransaction(PreparedTransactionDataDto),
    /// Response for:
    /// - [`PrepareCreateNativeToken`](crate::method::WalletCommandMethod::PrepareCreateNativeToken),
    PreparedCreateNativeTokenTransaction(PreparedCreateNativeTokenTransactionDto),
    /// Response for:
    /// - [`GetIncomingTransaction`](crate::method::WalletCommandMethod::GetIncomingTransaction)
    /// - [`GetTransaction`](crate::method::WalletCommandMethod::GetTransaction),
    Transaction(Option<Box<TransactionWithMetadataDto>>),
    /// Response for:
    /// - [`IncomingTransactions`](crate::method::WalletCommandMethod::IncomingTransactions)
    /// - [`PendingTransactions`](crate::method::WalletCommandMethod::PendingTransactions),
    /// - [`Transactions`](crate::method::WalletCommandMethod::Transactions),
    Transactions(Vec<TransactionWithMetadataDto>),
    /// Response for:
    /// - [`SignTransaction`](crate::method::WalletCommandMethod::SignTransaction)
    SignedTransactionData(SignedTransactionDataDto),
    /// Response for:
    /// - [`GetBalance`](crate::method::WalletCommandMethod::GetBalance),
    /// - [`Sync`](crate::method::WalletCommandMethod::Sync)
    Balance(Balance),
    /// Response for:
    /// - [`ClaimOutputs`](crate::method::WalletCommandMethod::ClaimOutputs)
    /// - [`Send`](crate::method::WalletCommandMethod::Send)
    /// - [`SendOutputs`](crate::method::WalletCommandMethod::SendOutputs)
    /// - [`SignAndSubmitTransaction`](crate::method::WalletCommandMethod::SignAndSubmitTransaction)
    /// - [`SubmitAndStoreTransaction`](crate::method::WalletCommandMethod::SubmitAndStoreTransaction)
    SentTransaction(TransactionWithMetadataDto),
    /// Response for:
    /// - [`GetParticipationEvent`](crate::method::WalletCommandMethod::GetParticipationEvent)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEvent(Option<ParticipationEventWithNodes>),
    /// Response for:
    /// - [`GetParticipationEventIds`](crate::method::WalletCommandMethod::GetParticipationEventIds)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEventIds(Vec<ParticipationEventId>),
    /// Response for:
    /// - [`GetParticipationEventStatus`](crate::method::WalletCommandMethod::GetParticipationEventStatus)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEventStatus(ParticipationEventStatus),
    /// Response for:
    /// - [`GetParticipationEvents`](crate::method::WalletCommandMethod::GetParticipationEvents)
    /// - [`RegisterParticipationEvents`](crate::method::WalletCommandMethod::RegisterParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEvents(HashMap<ParticipationEventId, ParticipationEventWithNodes>),
    /// Response for:
    /// - [`GetVotingPower`](crate::method::WalletCommandMethod::GetVotingPower)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    VotingPower(String),
    /// Response for:
    /// - [`GetParticipationOverview`](crate::method::WalletCommandMethod::GetParticipationOverview)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationOverview(ParticipationOverview),
}
