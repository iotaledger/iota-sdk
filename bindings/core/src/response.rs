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
    wallet::{
        account::{
            types::{AccountAddress, AccountBalanceDto, TransactionDto},
            MintTokenTransactionDto, OutputDataDto,
        },
        message_interface::dtos::{AccountDto, AddressWithUnspentOutputsDto},
    },
};
use serde::Serialize;
#[cfg(feature = "participation")]
use {
    iota_sdk::types::api::plugins::participation::types::{ParticipationEventId, ParticipationEventStatus},
    iota_sdk::wallet::account::{AccountParticipationOverview, ParticipationEventWithNodes},
    std::collections::HashMap,
};

use crate::{error::BindingsError, OmittedDebug};

/// The response message.
#[derive(Serialize, Derivative)]
#[derivative(Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Response {
    // Client responses
    /// Response for:
    /// - [`BuildAliasOutput`](crate::message_interface::Message::BuildAliasOutput)
    /// - [`BuildBasicOutput`](crate::message_interface::Message::BuildBasicOutput)
    /// - [`BuildFoundryOutput`](crate::message_interface::Message::BuildFoundryOutput)
    /// - [`BuildNftOutput`](crate::message_interface::Message::BuildNftOutput)
    BuiltOutput(OutputDto),
    /// Response for:
    /// - [`GenerateAddresses`](crate::message_interface::Message::GenerateAddresses)
    GeneratedAddresses(Vec<String>),
    /// Response for:
    /// - [`GetNode`](crate::message_interface::Message::GetNode)
    Node(Node),
    /// Response for:
    /// - [`GetNetworkInfo`](crate::message_interface::Message::GetNetworkInfo)
    NetworkInfo(NetworkInfoDto),
    /// Response for:
    /// - [`GetNetworkId`](crate::message_interface::Message::GetNetworkId)
    NetworkId(u64),
    /// Response for:
    /// - [`GetBech32Hrp`](crate::message_interface::Message::GetBech32Hrp)
    Bech32Hrp(String),
    /// Response for:
    /// - [`GetMinPowScore`](crate::message_interface::Message::GetMinPowScore)
    MinPowScore(u32),
    /// Response for:
    /// - [`GetTipsInterval`](crate::message_interface::Message::GetTipsInterval)
    TipsInterval(u64),
    /// Response for:
    /// - [`GetProtocolParameters`](crate::message_interface::Message::GetProtocolParameters)
    ProtocolParameters(ProtocolParametersDto),
    /// Response for:
    /// - [`PrepareTransaction`](crate::message_interface::Message::PrepareTransaction)
    PreparedTransactionData(PreparedTransactionDataDto),
    /// Response for:
    /// - [`SignTransaction`](crate::message_interface::Message::SignTransaction)
    SignedTransaction(PayloadDto),
    /// Response for:
    /// - [`SignatureUnlock`](crate::message_interface::Message::SignatureUnlock)
    SignatureUnlock(UnlockDto),
    /// Response for:
    /// - [`SignEd25519`](crate::message_interface::Message::SignEd25519)
    Ed25519Signature(Ed25519SignatureDto),
    /// Response for:
    /// - [`UnhealthyNodes`](crate::message_interface::Message::UnhealthyNodes)
    #[cfg(not(target_family = "wasm"))]
    UnhealthyNodes(HashSet<Node>),
    /// Response for:
    /// - [`GetNodeInfo`](crate::message_interface::Message::GetNodeInfo)
    NodeInfo(NodeInfo),
    /// Response for:
    /// - [`GetInfo`](crate::message_interface::Message::GetInfo)
    Info(NodeInfoWrapper),
    /// Response for:
    /// - [`GetPeers`](crate::message_interface::Message::GetPeers)
    Peers(Vec<PeerDto>),
    /// Response for:
    /// - [`GetTips`](crate::message_interface::Message::GetTips)
    Tips(Vec<BlockId>),
    /// Response for:
    /// - [`GetBlock`](crate::message_interface::Message::GetBlock)
    /// - [`GetIncludedBlock`](crate::message_interface::Message::GetIncludedBlock)
    Block(BlockDto),
    /// Response for:
    /// - [`BuildAndPostBlock`](crate::message_interface::Message::BuildAndPostBlock)
    /// - [`PostBlockPayload`](crate::message_interface::Message::PostBlockPayload)
    /// - [`Retry`](crate::message_interface::Message::Retry)
    BlockIdWithBlock(BlockId, BlockDto),
    /// Response for:
    /// - [`GetBlockMetadata`](crate::message_interface::Message::GetBlockMetadata)
    BlockMetadata(BlockMetadataResponse),
    /// Response for:
    /// - [`GetBlockRaw`](crate::message_interface::Message::GetBlockRaw)
    BlockRaw(Vec<u8>),
    /// Response for:
    /// - [`GetOutput`](crate::message_interface::Message::GetOutput)
    OutputWithMetadataResponse(OutputWithMetadataResponse),
    /// Response for:
    /// - [`GetOutputMetadata`](crate::message_interface::Message::GetOutputMetadata)
    OutputMetadata(OutputMetadataDto),
    /// Response for:
    /// - [`GetOutputs`](crate::message_interface::Message::GetOutputs)
    /// - [`TryGetOutputs`](crate::message_interface::Message::TryGetOutputs)
    /// - [`FindOutputs`](crate::message_interface::Message::FindOutputs)
    Outputs(Vec<OutputWithMetadataResponse>),
    /// Response for:
    /// - [`GetMilestoneById`](crate::message_interface::Message::GetMilestoneById)
    /// - [`GetMilestoneByIndex`](crate::message_interface::Message::GetMilestoneByIndex)
    Milestone(MilestonePayloadDto),
    /// Response for:
    /// - [`GetMilestoneByIdRaw`](crate::message_interface::Message::GetMilestoneByIdRaw)
    /// - [`GetMilestoneByIndexRaw`](crate::message_interface::Message::GetMilestoneByIndexRaw)
    MilestoneRaw(Vec<u8>),
    /// Response for:
    /// - [`GetUtxoChangesById`](crate::message_interface::Message::GetUtxoChangesById)
    /// - [`GetUtxoChangesByIndex`](crate::message_interface::Message::GetUtxoChangesByIndex)
    MilestoneUtxoChanges(MilestoneUTXOChanges),
    /// Response for:
    /// - [`GetReceipts`](crate::message_interface::Message::GetReceipts)
    /// - [`GetReceiptsMigratedAt`](crate::message_interface::Message::GetReceiptsMigratedAt)
    Receipts(Vec<ReceiptDto>),
    /// Response for:
    /// - [`GetTreasury`](crate::message_interface::Message::GetTreasury)
    Treasury(TreasuryResponse),
    /// Response for:
    /// - [`AliasOutputId`](crate::message_interface::Message::AliasOutputId)
    /// - [`NftOutputId`](crate::message_interface::Message::NftOutputId)
    /// - [`FoundryOutputId`](crate::message_interface::Message::FoundryOutputId)
    OutputId(OutputId),
    /// Response for:
    /// - [`BasicOutputIds`](crate::message_interface::Message::BasicOutputIds)
    /// - [`AliasOutputIds`](crate::message_interface::Message::AliasOutputIds)
    /// - [`NftOutputIds`](crate::message_interface::Message::NftOutputIds)
    /// - [`FoundryOutputIds`](crate::message_interface::Message::FoundryOutputIds)
    OutputIdsResponse(OutputIdsResponse),
    /// Response for:
    /// - [`FindBlocks`](crate::message_interface::Message::FindBlocks)
    Blocks(Vec<BlockDto>),
    /// Response for:
    /// - [`RetryUntilIncluded`](crate::message_interface::Message::RetryUntilIncluded)
    RetryUntilIncludedSuccessful(Vec<(BlockId, BlockDto)>),
    /// Response for:
    /// - [`ConsolidateFunds`](crate::message_interface::Message::ConsolidateFunds)
    ConsolidatedFunds(String),
    /// Response for:
    /// - [`FindInputs`](crate::message_interface::Message::FindInputs)
    Inputs(Vec<UtxoInputDto>),
    /// Response for:
    /// - [`Reattach`](crate::message_interface::Message::Reattach)
    /// - [`ReattachUnchecked`](crate::message_interface::Message::ReattachUnchecked)
    Reattached((BlockId, BlockDto)),
    /// Response for:
    /// - [`Promote`](crate::message_interface::Message::Promote)
    /// - [`PromoteUnchecked`](crate::message_interface::Message::PromoteUnchecked)
    Promoted((BlockId, BlockDto)),
    /// Response for:
    /// - [`Bech32ToHex`](crate::message_interface::Message::Bech32ToHex)
    Bech32ToHex(String),
    /// Response for:
    /// - [`ParseBech32Address`](crate::message_interface::Message::ParseBech32Address)
    ParsedBech32Address(AddressDto),
    /// Response for:
    /// - [`MnemonicToHexSeed`](crate::message_interface::Message::MnemonicToHexSeed)
    MnemonicHexSeed(#[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))] String),
    /// Response for:
    /// - [`TransactionId`](crate::message_interface::Message::TransactionId)
    TransactionId(TransactionId),
    /// Response for:
    /// - [`ComputeAliasId`](crate::message_interface::Message::ComputeAliasId)
    AliasId(AliasId),
    /// Response for:
    /// - [`ComputeNftId`](crate::message_interface::Message::ComputeNftId)
    NftId(NftId),
    /// Response for:
    /// - [`ComputeFoundryId`](crate::message_interface::Message::ComputeFoundryId)
    FoundryId(FoundryId),
    /// Response for:
    /// - [`HashTransactionEssence`](crate::message_interface::Message::HashTransactionEssence)
    TransactionEssenceHash(String),

    // Responses in client and wallet
    /// Response for [`HexToBech32`](crate::message_interface::Message::HexToBech32)
    /// Response for [`GenerateAddress`](crate::message_interface::Message::GenerateAddress)
    /// Response for:
    /// - [`AliasIdToBech32`](crate::message_interface::Message::AliasIdToBech32)
    /// - [`HexPublicKeyToBech32Address`](crate::message_interface::Message::HexPublicKeyToBech32Address)
    /// - [`HexToBech32`](crate::message_interface::Message::HexToBech32)
    /// - [`NftIdToBech32`](crate::message_interface::Message::NftIdToBech32)
    Bech32Address(String),
    /// - [`Faucet`](crate::message_interface::Message::Faucet)
    /// Response for [`RequestFundsFromFaucet`](crate::message_interface::AccountMethod::RequestFundsFromFaucet)
    Faucet(String),
    /// Response for:
    /// - [`GenerateMnemonic`](crate::message_interface::Message::GenerateMnemonic)
    /// [`GenerateMnemonic`](crate::message_interface::Message::GenerateMnemonic)
    GeneratedMnemonic(#[derivative(Debug(format_with = "OmittedDebug::omitted_fmt"))] String),
    /// Response for
    /// - [`GetLedgerNanoStatus`](crate::message_interface::Message::GetLedgerNanoStatus)
    /// [`GetLedgerNanoStatus`](crate::message_interface::Message::GetLedgerNanoStatus),
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerNanoStatus(LedgerNanoStatus),
    /// Response for:
    /// - [`BlockId`](crate::message_interface::Message::BlockId)
    /// - [`PostBlock`](crate::message_interface::Message::PostBlock)
    /// - [`PostBlockRaw`](crate::message_interface::Message::PostBlockRaw)
    /// [`RetryTransactionUntilIncluded`](crate::message_interface::AccountMethod::RetryTransactionUntilIncluded)
    BlockId(BlockId),
    /// Response for
    /// - [`GetLocalPow`](crate::message_interface::Message::GetLocalPow)
    /// - [`GetFallbackToLocalPow`](crate::message_interface::Message::GetFallbackToLocalPow)
    /// - [`VerifyEd25519Signature`](crate::message_interface::Message::VerifyEd25519Signature)
    /// - [`GetHealth`](crate::message_interface::Message::GetHealth)
    /// - [`IsAddressValid`](crate::message_interface::Message::IsAddressValid)
    Bool(bool),
    /// Response for
    /// [`Backup`](crate::message_interface::Message::Backup),
    /// [`ClearStrongholdPassword`](crate::message_interface::Message::ClearStrongholdPassword),
    /// [`DeregisterParticipationEvent`](crate::message_interface::AccountMethod::DeregisterParticipationEvent),
    /// [`RestoreBackup`](crate::message_interface::Message::RestoreBackup),
    /// [`VerifyMnemonic`](crate::message_interface::Message::VerifyMnemonic),
    /// [`SetClientOptions`](crate::message_interface::Message::SetClientOptions),
    /// [`SetStrongholdPassword`](crate::message_interface::Message::SetStrongholdPassword),
    /// [`SetStrongholdPasswordClearInterval`](crate::message_interface::Message::SetStrongholdPasswordClearInterval),
    /// [`StoreMnemonic`](crate::message_interface::Message::StoreMnemonic),
    /// [`StartBackgroundSync`](crate::message_interface::Message::StartBackgroundSync),
    /// [`StopBackgroundSync`](crate::message_interface::Message::StopBackgroundSync),
    /// [`EmitTestEvent`](crate::message_interface::Message::EmitTestEvent),
    /// - [`ClearListeners`](crate::message_interface::Message::ClearListeners)
    /// - [`StoreMnemonic`](crate::message_interface::Message::StoreMnemonic)
    Ok,
    /// Response for any method that returns an error.
    Error(BindingsError),
    /// Response for any method that panics.
    Panic(String),

    // wallet responses
    /// Response for
    /// [`CreateAccount`](crate::message_interface::Message::CreateAccount),
    /// [`GetAccount`](crate::message_interface::Message::GetAccount)
    Account(AccountDto),
    /// Response for [`GetAccountIndexes`](crate::message_interface::Message::GetAccountIndexes)
    AccountIndexes(Vec<u32>),
    /// Response for [`GetAccounts`](crate::message_interface::Message::GetAccounts)
    Accounts(Vec<AccountDto>),
    /// Response for [`Addresses`](crate::message_interface::AccountMethod::Addresses)
    Addresses(Vec<AccountAddress>),
    /// Response for
    /// [`AddressesWithUnspentOutputs`](crate::message_interface::AccountMethod::AddressesWithUnspentOutputs)
    AddressesWithUnspentOutputs(Vec<AddressWithUnspentOutputsDto>),
    /// Response for
    /// [`BuildAliasOutput`](crate::message_interface::AccountMethod::BuildAliasOutput)
    /// [`BuildBasicOutput`](crate::message_interface::AccountMethod::BuildBasicOutput)
    /// [`BuildFoundryOutput`](crate::message_interface::AccountMethod::BuildFoundryOutput)
    /// [`BuildNftOutput`](crate::message_interface::AccountMethod::BuildNftOutput)
    /// [`GetFoundryOutput`](crate::message_interface::AccountMethod::GetFoundryOutput)
    /// [`PrepareOutput`](crate::message_interface::AccountMethod::PrepareOutput)
    Output(OutputDto),
    /// Response for
    /// [`MinimumRequiredStorageDeposit`](crate::message_interface::AccountMethod::MinimumRequiredStorageDeposit)
    MinimumRequiredStorageDeposit(String),
    /// Response for
    /// [`GetOutputsWithAdditionalUnlockConditions`](crate::message_interface::AccountMethod::GetOutputsWithAdditionalUnlockConditions)
    OutputIds(Vec<OutputId>),
    /// Response for [`GetOutput`](crate::message_interface::AccountMethod::GetOutput)
    OutputData(Option<Box<OutputDataDto>>),
    /// Response for
    /// [`Outputs`](crate::message_interface::AccountMethod::Outputs),
    /// [`UnspentOutputs`](crate::message_interface::AccountMethod::UnspentOutputs)
    OutputsData(Vec<OutputDataDto>),
    /// Response for
    /// [`PrepareSendAmount`](crate::message_interface::AccountMethod::PrepareSendAmount),
    /// [`PrepareTransaction`](crate::message_interface::AccountMethod::PrepareTransaction)
    PreparedTransaction(PreparedTransactionDataDto),
    /// Response for
    /// [`GetTransaction`](crate::message_interface::AccountMethod::GetTransaction),
    Transaction(Option<Box<TransactionDto>>),
    /// Response for
    /// [`Transactions`](crate::message_interface::AccountMethod::Transactions),
    /// [`PendingTransactions`](crate::message_interface::AccountMethod::PendingTransactions)
    Transactions(Vec<TransactionDto>),
    /// Response for
    /// [`SignTransactionEssence`](crate::message_interface::AccountMethod::SignTransactionEssence)
    /// [`SubmitAndStoreTransaction`](crate::message_interface::AccountMethod::SubmitAndStoreTransaction)
    SignedTransactionData(SignedTransactionDataDto),
    /// GenerateAddress response.
    /// Response for [`GenerateAddresses`](crate::message_interface::AccountMethod::GenerateAddresses)
    GeneratedAddress(Vec<AccountAddress>),
    /// Response for
    /// [`GetBalance`](crate::message_interface::AccountMethod::GetBalance),
    /// [`SyncAccount`](crate::message_interface::AccountMethod::SyncAccount)
    Balance(AccountBalanceDto),
    /// Response for
    /// [`GetIncomingTransactionData`](crate::message_interface::AccountMethod::GetIncomingTransactionData),
    IncomingTransactionData(Option<Box<(TransactionId, TransactionDto)>>),
    /// Response for
    /// [`IncomingTransactions`](crate::message_interface::AccountMethod::IncomingTransactions),
    IncomingTransactionsData(Vec<(TransactionId, TransactionDto)>),
    /// Response for
    /// [`ConsolidateOutputs`](crate::message_interface::AccountMethod::ConsolidateOutputs)
    /// [`ClaimOutputs`](crate::message_interface::AccountMethod::ClaimOutputs)
    /// [`CreateAliasOutput`](crate::message_interface::AccountMethod::CreateAliasOutput)
    /// [`SendAmount`](crate::message_interface::AccountMethod::SendAmount),
    /// [`MintNfts`](crate::message_interface::AccountMethod::MintNfts),
    /// [`SendAmount`](crate::message_interface::AccountMethod::SendAmount),
    /// [`SendNativeTokens`](crate::message_interface::AccountMethod::SendNativeTokens),
    /// [`SendNft`](crate::message_interface::AccountMethod::SendNft),
    /// [`SendOutputs`](crate::message_interface::AccountMethod::SendOutputs)
    /// [`SubmitAndStoreTransaction`](crate::message_interface::AccountMethod::SubmitAndStoreTransaction)
    /// [`Vote`](crate::message_interface::AccountMethod::Vote)
    /// [`StopParticipating`](crate::message_interface::AccountMethod::StopParticipating)
    /// [`IncreaseVotingPower`](crate::message_interface::AccountMethod::IncreaseVotingPower)
    /// [`DecreaseVotingPower`](crate::message_interface::AccountMethod::DecreaseVotingPower)
    SentTransaction(TransactionDto),
    /// Response for
    /// [`MintNativeToken`](crate::message_interface::AccountMethod::MintNativeToken),
    MintTokenTransaction(MintTokenTransactionDto),
    /// Response for [`GetNodeInfo`](crate::message_interface::Message::GetNodeInfo)
    NodeInfoWrapper(NodeInfoWrapper),
    /// Response for
    /// [`GetParticipationEvent`](crate::message_interface::AccountMethod::GetParticipationEvent)
    /// [`RegisterParticipationEvent`](crate::message_interface::AccountMethod::RegisterParticipationEvent)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEvent(Option<ParticipationEventWithNodes>),
    /// Response for
    /// [`GetParticipationEventIds`](crate::message_interface::AccountMethod::GetParticipationEventIds)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEventIds(Vec<ParticipationEventId>),
    /// Response for
    /// [`GetParticipationEventStatus`](crate::message_interface::AccountMethod::GetParticipationEventStatus)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEventStatus(ParticipationEventStatus),
    /// Response for
    /// [`GetParticipationEvents`](crate::message_interface::AccountMethod::GetParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEvents(HashMap<ParticipationEventId, ParticipationEventWithNodes>),
    /// Response for
    /// [`GetVotingPower`](crate::message_interface::AccountMethod::GetVotingPower)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    VotingPower(String),
    /// Response for
    /// [`GetParticipationOverview`](crate::message_interface::AccountMethod::GetParticipationOverview)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    AccountParticipationOverview(AccountParticipationOverview),
    /// Response for [`Bech32ToHex`](crate::message_interface::Message::Bech32ToHex)
    HexAddress(String),
}
