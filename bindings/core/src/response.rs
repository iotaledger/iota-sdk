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
        NetworkInfoDto, NodeInfoWrapper,
    },
    types::{
        api::{
            core::{
                dto::{PeerDto, ReceiptDto},
                response::{
                    BlockMetadataResponse, InfoResponse as NodeInfo, OutputWithMetadataResponse, TreasuryResponse,
                    UtxoChangesResponse as MilestoneUTXOChanges,
                },
            },
            plugins::indexer::OutputIdsResponse,
        },
        block::{
            address::{dto::AddressDto, Bech32Address, Hrp},
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
    wallet::{
        account::{
            types::{AccountAddress, AddressWithUnspentOutputs, BalanceDto, OutputDataDto, TransactionDto},
            PreparedMintTokenTransactionDto,
        },
        message_interface::dtos::AccountDetailsDto,
    },
};
use serde::Serialize;
#[cfg(feature = "participation")]
use {
    iota_sdk::types::api::plugins::participation::types::{ParticipationEventId, ParticipationEventStatus},
    iota_sdk::wallet::account::{AccountParticipationOverview, ParticipationEventWithNodes},
    std::collections::HashMap,
};

use crate::{error::Error, OmittedDebug};

/// The response message.
#[derive(Serialize, Derivative)]
#[derivative(Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Response {
    // Client responses
    /// Response for:
    /// - [`GenerateAddresses`](crate::method::ClientMethod::GenerateAddresses)
    GeneratedAddresses(Vec<Bech32Address>),
    /// Response for:
    /// - [`GetNode`](crate::method::ClientMethod::GetNode)
    Node(Node),
    /// Response for:
    /// - [`GetNetworkInfo`](crate::method::ClientMethod::GetNetworkInfo)
    NetworkInfo(NetworkInfoDto),
    /// Response for:
    /// - [`GetNetworkId`](crate::method::ClientMethod::GetNetworkId)
    NetworkId(u64),
    /// Response for:
    /// - [`GetBech32Hrp`](crate::method::ClientMethod::GetBech32Hrp)
    Bech32Hrp(Hrp),
    /// Response for:
    /// - [`GetMinPowScore`](crate::method::ClientMethod::GetMinPowScore)
    MinPowScore(u32),
    /// Response for:
    /// - [`GetTipsInterval`](crate::method::ClientMethod::GetTipsInterval)
    TipsInterval(u64),
    /// Response for:
    /// - [`GetProtocolParameters`](crate::method::ClientMethod::GetProtocolParameters)
    ProtocolParameters(ProtocolParametersDto),
    /// Response for:
    /// - [`PrepareTransaction`](crate::method::ClientMethod::PrepareTransaction)
    PreparedTransactionData(PreparedTransactionDataDto),
    /// Response for:
    /// - [`SignTransaction`](crate::method::ClientMethod::SignTransaction)
    SignedTransaction(PayloadDto),
    /// Response for:
    /// - [`SignatureUnlock`](crate::method::SecretManagerMethod::SignatureUnlock)
    SignatureUnlock(UnlockDto),
    /// Response for:
    /// - [`SignEd25519`](crate::method::SecretManagerMethod::SignEd25519)
    Ed25519Signature(Ed25519SignatureDto),
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
    Peers(Vec<PeerDto>),
    /// Response for:
    /// - [`GetTips`](crate::method::ClientMethod::GetTips)
    Tips(Vec<BlockId>),
    /// Response for:
    /// - [`GetBlock`](crate::method::ClientMethod::GetBlock)
    /// - [`GetIncludedBlock`](crate::method::ClientMethod::GetIncludedBlock)
    Block(BlockDto),
    /// Response for:
    /// - [`BuildAndPostBlock`](crate::method::ClientMethod::BuildAndPostBlock)
    /// - [`PostBlockPayload`](crate::method::ClientMethod::PostBlockPayload)
    /// - [`Retry`](crate::method::ClientMethod::Retry)
    BlockIdWithBlock(BlockId, BlockDto),
    /// Response for:
    /// - [`GetBlockMetadata`](crate::method::ClientMethod::GetBlockMetadata)
    BlockMetadata(BlockMetadataResponse),
    /// Response for:
    /// - [`GetBlockRaw`](crate::method::ClientMethod::GetBlockRaw)
    BlockRaw(Vec<u8>),
    /// Response for:
    /// - [`GetOutput`](crate::method::ClientMethod::GetOutput)
    OutputWithMetadataResponse(OutputWithMetadataResponse),
    /// Response for:
    /// - [`GetOutputMetadata`](crate::method::ClientMethod::GetOutputMetadata)
    OutputMetadata(OutputMetadataDto),
    /// Response for:
    /// - [`GetOutputs`](crate::method::ClientMethod::GetOutputs)
    /// - [`GetOutputsIgnoreErrors`](crate::method::ClientMethod::GetOutputsIgnoreErrors)
    /// - [`FindOutputs`](crate::method::ClientMethod::FindOutputs)
    Outputs(Vec<OutputWithMetadataResponse>),
    /// Response for:
    /// - [`GetMilestoneById`](crate::method::ClientMethod::GetMilestoneById)
    /// - [`GetMilestoneByIndex`](crate::method::ClientMethod::GetMilestoneByIndex)
    Milestone(MilestonePayloadDto),
    /// Response for:
    /// - [`GetMilestoneByIdRaw`](crate::method::ClientMethod::GetMilestoneByIdRaw)
    /// - [`GetMilestoneByIndexRaw`](crate::method::ClientMethod::GetMilestoneByIndexRaw)
    MilestoneRaw(Vec<u8>),
    /// Response for:
    /// - [`GetUtxoChangesById`](crate::method::ClientMethod::GetUtxoChangesById)
    /// - [`GetUtxoChangesByIndex`](crate::method::ClientMethod::GetUtxoChangesByIndex)
    MilestoneUtxoChanges(MilestoneUTXOChanges),
    /// Response for:
    /// - [`GetReceipts`](crate::method::ClientMethod::GetReceipts)
    /// - [`GetReceiptsMigratedAt`](crate::method::ClientMethod::GetReceiptsMigratedAt)
    Receipts(Vec<ReceiptDto>),
    /// Response for:
    /// - [`GetTreasury`](crate::method::ClientMethod::GetTreasury)
    Treasury(TreasuryResponse),
    /// Response for:
    /// - [`AliasOutputId`](crate::method::ClientMethod::AliasOutputId)
    /// - [`NftOutputId`](crate::method::ClientMethod::NftOutputId)
    /// - [`FoundryOutputId`](crate::method::ClientMethod::FoundryOutputId)
    OutputId(OutputId),
    /// Response for:
    /// - [`BasicOutputIds`](crate::method::ClientMethod::BasicOutputIds)
    /// - [`AliasOutputIds`](crate::method::ClientMethod::AliasOutputIds)
    /// - [`NftOutputIds`](crate::method::ClientMethod::NftOutputIds)
    /// - [`FoundryOutputIds`](crate::method::ClientMethod::FoundryOutputIds)
    OutputIdsResponse(OutputIdsResponse),
    /// Response for:
    /// - [`FindBlocks`](crate::method::ClientMethod::FindBlocks)
    Blocks(Vec<BlockDto>),
    /// Response for:
    /// - [`RetryUntilIncluded`](crate::method::ClientMethod::RetryUntilIncluded)
    RetryUntilIncludedSuccessful(Vec<(BlockId, BlockDto)>),
    /// Response for:
    /// - [`ConsolidateFunds`](crate::method::ClientMethod::ConsolidateFunds)
    ConsolidatedFunds(Bech32Address),
    /// Response for:
    /// - [`FindInputs`](crate::method::ClientMethod::FindInputs)
    Inputs(Vec<UtxoInputDto>),
    /// Response for:
    /// - [`Reattach`](crate::method::ClientMethod::Reattach)
    /// - [`ReattachUnchecked`](crate::method::ClientMethod::ReattachUnchecked)
    Reattached((BlockId, BlockDto)),
    /// Response for:
    /// - [`Promote`](crate::method::ClientMethod::Promote)
    /// - [`PromoteUnchecked`](crate::method::ClientMethod::PromoteUnchecked)
    Promoted((BlockId, BlockDto)),
    /// Response for:
    /// - [`Bech32ToHex`](crate::method::UtilsMethod::Bech32ToHex)
    Bech32ToHex(String),
    /// Response for:
    /// - [`ParseBech32Address`](crate::method::UtilsMethod::ParseBech32Address)
    ParsedBech32Address(AddressDto),
    /// Response for:
    /// - [`MnemonicToHexSeed`](crate::method::UtilsMethod::MnemonicToHexSeed)
    MnemonicHexSeed(#[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))] String),
    /// Response for:
    /// - [`TransactionId`](crate::method::UtilsMethod::TransactionId)
    TransactionId(TransactionId),
    /// Response for:
    /// - [`ComputeAliasId`](crate::method::UtilsMethod::ComputeAliasId)
    AliasId(AliasId),
    /// Response for:
    /// - [`ComputeNftId`](crate::method::UtilsMethod::ComputeNftId)
    NftId(NftId),
    /// Response for:
    /// - [`ComputeFoundryId`](crate::method::UtilsMethod::ComputeFoundryId)
    FoundryId(FoundryId),
    /// Response for:
    /// - [`HashTransactionEssence`](crate::method::UtilsMethod::HashTransactionEssence)
    TransactionEssenceHash(String),
    /// Response for [`GetNodeInfo`](crate::method::ClientMethod::GetNodeInfo)
    NodeInfoWrapper(NodeInfoWrapper),
    /// Response for [`Bech32ToHex`](crate::method::UtilsMethod::Bech32ToHex)
    HexAddress(String),

    // Responses in client and wallet
    /// Response for:
    /// - [`BuildAliasOutput`](crate::method::ClientMethod::BuildAliasOutput)
    /// - [`BuildBasicOutput`](crate::method::ClientMethod::BuildBasicOutput)
    /// - [`BuildFoundryOutput`](crate::method::ClientMethod::BuildFoundryOutput)
    /// - [`BuildNftOutput`](crate::method::ClientMethod::BuildNftOutput)
    /// - [`GetFoundryOutput`](crate::method::AccountMethod::GetFoundryOutput)
    /// - [`PrepareOutput`](crate::method::AccountMethod::PrepareOutput)
    Output(OutputDto),
    /// Response for:
    /// - [`HexToBech32`](crate::method::ClientMethod::HexToBech32)
    /// - [`GenerateAddresses`](crate::method::ClientMethod::GenerateAddresses)
    /// - [`AliasIdToBech32`](crate::method::ClientMethod::AliasIdToBech32)
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
    /// - [`RetryTransactionUntilIncluded`](crate::method::AccountMethod::RetryTransactionUntilIncluded)
    BlockId(BlockId),
    /// Response for
    /// - [`GetLocalPow`](crate::method::ClientMethod::GetLocalPow)
    /// - [`GetFallbackToLocalPow`](crate::method::ClientMethod::GetFallbackToLocalPow)
    /// - [`VerifyEd25519Signature`](crate::method::UtilsMethod::VerifyEd25519Signature)
    /// - [`GetHealth`](crate::method::ClientMethod::GetHealth)
    /// - [`IsAddressValid`](crate::method::UtilsMethod::IsAddressValid)
    Bool(bool),
    /// Response for
    /// - [`Backup`](crate::method::WalletMethod::Backup),
    /// - [`ClearStrongholdPassword`](crate::method::WalletMethod::ClearStrongholdPassword),
    /// - [`DeregisterParticipationEvent`](crate::method::AccountMethod::DeregisterParticipationEvent),
    /// - [`SetAlias`](crate::method::AccountMethod::SetAlias),
    /// - [`SetDefaultSyncOptions`](crate::method::AccountMethod::SetDefaultSyncOptions),
    /// - [`RestoreBackup`](crate::method::WalletMethod::RestoreBackup),
    /// - [`SetClientOptions`](crate::method::WalletMethod::SetClientOptions),
    /// - [`SetStrongholdPassword`](crate::method::WalletMethod::SetStrongholdPassword),
    /// - [`SetStrongholdPasswordClearInterval`](crate::method::WalletMethod::SetStrongholdPasswordClearInterval),
    /// - [`StoreMnemonic`](crate::method::WalletMethod::StoreMnemonic),
    /// - [`StartBackgroundSync`](crate::method::WalletMethod::StartBackgroundSync),
    /// - [`StopBackgroundSync`](crate::method::WalletMethod::StopBackgroundSync),
    /// - [`EmitTestEvent`](crate::method::WalletMethod::EmitTestEvent),
    /// - [`ClearListeners`](crate::method::WalletMethod::ClearListeners)
    /// - [`StoreMnemonic`](crate::method::WalletMethod::StoreMnemonic)
    Ok,
    /// Response for any method that returns an error.
    Error(Error),
    /// Response for any method that panics.
    Panic(String),

    // wallet responses
    /// Response for
    /// - [`CreateAccount`](crate::method::WalletMethod::CreateAccount),
    /// - [`GetAccount`](crate::method::WalletMethod::GetAccount)
    Account(AccountDetailsDto),
    /// Response for [`GetAccountIndexes`](crate::method::WalletMethod::GetAccountIndexes)
    AccountIndexes(Vec<u32>),
    /// Response for [`GetAccounts`](crate::method::WalletMethod::GetAccounts)
    Accounts(Vec<AccountDetailsDto>),
    /// Response for [`Addresses`](crate::method::AccountMethod::Addresses)
    Addresses(Vec<AccountAddress>),
    /// Response for
    /// - [`AddressesWithUnspentOutputs`](crate::method::AccountMethod::AddressesWithUnspentOutputs)
    AddressesWithUnspentOutputs(Vec<AddressWithUnspentOutputs>),
    /// Response for
    /// - [`MinimumRequiredStorageDeposit`](crate::method::AccountMethod::MinimumRequiredStorageDeposit)
    MinimumRequiredStorageDeposit(String),
    /// Response for
    /// - [`GetOutputsWithAdditionalUnlockConditions`](crate::method::AccountMethod::GetOutputsWithAdditionalUnlockConditions)
    OutputIds(Vec<OutputId>),
    /// Response for [`GetOutput`](crate::method::AccountMethod::GetOutput)
    OutputData(Option<Box<OutputDataDto>>),
    /// Response for
    /// - [`Outputs`](crate::method::AccountMethod::Outputs),
    /// - [`UnspentOutputs`](crate::method::AccountMethod::UnspentOutputs)
    OutputsData(Vec<OutputDataDto>),
    /// Response for    
    /// [`PrepareBurn`](crate::method::AccountMethod::PrepareBurn),
    /// [`PrepareConsolidateOutputs`](crate::method::AccountMethod::PrepareConsolidateOutputs)
    /// [`PrepareCreateAliasOutput`](crate::method::AccountMethod::PrepareCreateAliasOutput)
    /// [`PrepareDecreaseNativeTokenSupply`](crate::method::AccountMethod::PrepareDecreaseNativeTokenSupply)
    /// [`PrepareDecreaseVotingPower`](crate::method::AccountMethod::PrepareDecreaseVotingPower)
    /// [`PrepareIncreaseVotingPower`](crate::method::AccountMethod::PrepareIncreaseVotingPower)
    /// [`PrepareMintNfts`](crate::method::AccountMethod::PrepareMintNfts),
    /// [`PrepareSendAmount`](crate::method::AccountMethod::PrepareSendAmount),
    /// [`PrepareSendNativeTokens`](crate::method::AccountMethod::PrepareSendNativeTokens),
    /// [`PrepareSendNft`](crate::method::AccountMethod::PrepareSendNft),
    /// [`PrepareStopParticipating`](crate::method::AccountMethod::PrepareStopParticipating)
    /// [`PrepareTransaction`](crate::method::AccountMethod::PrepareTransaction)
    /// [`PrepareVote`](crate::method::AccountMethod::PrepareVote)
    PreparedTransaction(PreparedTransactionDataDto),
    /// Response for
    /// - [`PrepareIncreaseNativeTokenSupply`](crate::method::AccountMethod::PrepareIncreaseNativeTokenSupply),
    /// - [`PrepareMintNativeToken`](crate::method::AccountMethod::PrepareMintNativeToken),
    PreparedMintTokenTransaction(PreparedMintTokenTransactionDto),
    /// Response for
    /// - [`GetTransaction`](crate::method::AccountMethod::GetTransaction),
    /// - [`GetIncomingTransaction`](crate::method::AccountMethod::GetIncomingTransaction)
    Transaction(Option<Box<TransactionDto>>),
    /// Response for
    /// - [`Transactions`](crate::method::AccountMethod::Transactions),
    /// - [`PendingTransactions`](crate::method::AccountMethod::PendingTransactions),
    /// - [`IncomingTransactions`](crate::method::AccountMethod::IncomingTransactions)
    Transactions(Vec<TransactionDto>),
    /// Response for
    /// - [`SignTransactionEssence`](crate::method::AccountMethod::SignTransactionEssence)
    SignedTransactionData(SignedTransactionDataDto),
    /// GenerateAddress response.
    /// Response for [`GenerateAddresses`](crate::method::AccountMethod::GenerateAddresses)
    GeneratedAddress(Vec<AccountAddress>),
    /// Response for
    /// - [`GetBalance`](crate::method::AccountMethod::GetBalance),
    /// - [`Sync`](crate::method::AccountMethod::Sync)
    Balance(BalanceDto),
    /// Response for
    /// - [`ClaimOutputs`](crate::method::AccountMethod::ClaimOutputs)
    /// - [`SendAmount`](crate::method::AccountMethod::SendAmount)
    /// - [`SendOutputs`](crate::method::AccountMethod::SendOutputs)
    /// - [`SignAndSubmitTransaction`](crate::method::AccountMethod::SignAndSubmitTransaction)
    /// - [`SubmitAndStoreTransaction`](crate::method::AccountMethod::SubmitAndStoreTransaction)
    SentTransaction(TransactionDto),
    /// Response for
    /// - [`GetParticipationEvent`](crate::method::AccountMethod::GetParticipationEvent)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEvent(Option<ParticipationEventWithNodes>),
    /// Response for
    /// - [`GetParticipationEventIds`](crate::method::AccountMethod::GetParticipationEventIds)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEventIds(Vec<ParticipationEventId>),
    /// Response for
    /// - [`GetParticipationEventStatus`](crate::method::AccountMethod::GetParticipationEventStatus)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEventStatus(ParticipationEventStatus),
    /// Response for
    /// - [`GetParticipationEvents`](crate::method::AccountMethod::GetParticipationEvents)
    /// - [`RegisterParticipationEvents`](crate::method::AccountMethod::RegisterParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEvents(HashMap<ParticipationEventId, ParticipationEventWithNodes>),
    /// Response for
    /// - [`GetVotingPower`](crate::method::AccountMethod::GetVotingPower)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    VotingPower(String),
    /// Response for
    /// - [`GetParticipationOverview`](crate::method::AccountMethod::GetParticipationOverview)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    AccountParticipationOverview(AccountParticipationOverview),
}
