// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::{Debug, Formatter, Result};

use serde::Serialize;
#[cfg(feature = "participation")]
use {
    crate::types::api::plugins::participation::types::{ParticipationEventId, ParticipationEventStatus},
    crate::wallet::account::operations::participation::{AccountParticipationOverview, ParticipationEventWithNodes},
    std::collections::HashMap,
};

#[cfg(feature = "ledger_nano")]
use crate::client::secret::LedgerNanoStatus;
use crate::{
    client::{
        api::{PreparedTransactionDataDto, SignedTransactionDataDto},
        NodeInfoWrapper,
    },
    types::block::{
        output::{dto::OutputDto, OutputId},
        BlockId,
    },
    wallet::{
        account::{
            operations::transaction::high_level::minting::mint_native_token::MintTokenTransactionDto,
            types::{address::AccountAddress, AccountBalanceDto, AddressWithUnspentOutputs, TransactionDto},
            OutputDataDto,
        },
        message_interface::dtos::AccountDetailsDto,
        Error,
    },
};

/// The response message.
#[derive(Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Response {
    /// Response for
    /// [`CreateAccount`](crate::wallet::message_interface::Message::CreateAccount),
    /// [`GetAccount`](crate::wallet::message_interface::Message::GetAccount)
    Account(AccountDetailsDto),
    /// Response for [`GetAccountIndexes`](crate::wallet::message_interface::Message::GetAccountIndexes)
    AccountIndexes(Vec<u32>),
    /// Response for [`GetAccounts`](crate::wallet::message_interface::Message::GetAccounts)
    Accounts(Vec<AccountDetailsDto>),
    /// Response for [`Addresses`](crate::wallet::message_interface::AccountMethod::Addresses)
    Addresses(Vec<AccountAddress>),
    /// Response for
    /// [`AddressesWithUnspentOutputs`](crate::wallet::message_interface::AccountMethod::AddressesWithUnspentOutputs)
    AddressesWithUnspentOutputs(Vec<AddressWithUnspentOutputs>),
    /// Response for
    /// [`RetryTransactionUntilIncluded`](crate::wallet::message_interface::AccountMethod::RetryTransactionUntilIncluded)
    BlockId(BlockId),
    /// Response for
    /// [`BuildAliasOutput`](crate::wallet::message_interface::AccountMethod::BuildAliasOutput)
    /// [`BuildBasicOutput`](crate::wallet::message_interface::AccountMethod::BuildBasicOutput)
    /// [`BuildFoundryOutput`](crate::wallet::message_interface::AccountMethod::BuildFoundryOutput)
    /// [`BuildNftOutput`](crate::wallet::message_interface::AccountMethod::BuildNftOutput)
    /// [`GetFoundryOutput`](crate::wallet::message_interface::AccountMethod::GetFoundryOutput)
    /// [`PrepareOutput`](crate::wallet::message_interface::AccountMethod::PrepareOutput)
    Output(OutputDto),
    /// Response for
    /// [`MinimumRequiredStorageDeposit`](crate::wallet::message_interface::AccountMethod::MinimumRequiredStorageDeposit)
    MinimumRequiredStorageDeposit(String),
    /// Response for
    /// [`GetOutputsWithAdditionalUnlockConditions`](crate::wallet::message_interface::AccountMethod::GetOutputsWithAdditionalUnlockConditions)
    OutputIds(Vec<OutputId>),
    /// Response for [`GetOutput`](crate::wallet::message_interface::AccountMethod::GetOutput)
    OutputData(Option<Box<OutputDataDto>>),
    /// Response for
    /// [`Outputs`](crate::wallet::message_interface::AccountMethod::Outputs),
    /// [`UnspentOutputs`](crate::wallet::message_interface::AccountMethod::UnspentOutputs)
    OutputsData(Vec<OutputDataDto>),
    /// Response for
    /// [`PrepareSendAmount`](crate::wallet::message_interface::AccountMethod::PrepareSendAmount),
    /// [`PrepareTransaction`](crate::wallet::message_interface::AccountMethod::PrepareTransaction)
    PreparedTransaction(PreparedTransactionDataDto),
    /// Response for
    /// [`GetTransaction`](crate::wallet::message_interface::AccountMethod::GetTransaction),
    Transaction(Option<Box<TransactionDto>>),
    /// Response for
    /// [`Transactions`](crate::wallet::message_interface::AccountMethod::Transactions),
    /// [`PendingTransactions`](crate::wallet::message_interface::AccountMethod::PendingTransactions)
    Transactions(Vec<TransactionDto>),
    /// Response for
    /// [`SignTransactionEssence`](crate::wallet::message_interface::AccountMethod::SignTransactionEssence)
    /// [`SubmitAndStoreTransaction`](crate::wallet::message_interface::AccountMethod::SubmitAndStoreTransaction)
    SignedTransactionData(SignedTransactionDataDto),
    /// GenerateAddress response.
    /// Response for [`GenerateAddresses`](crate::wallet::message_interface::AccountMethod::GenerateAddresses)
    GeneratedAddress(Vec<AccountAddress>),
    /// Response for
    /// [`GetBalance`](crate::wallet::message_interface::AccountMethod::GetBalance),
    /// [`SyncAccount`](crate::wallet::message_interface::AccountMethod::SyncAccount)
    Balance(AccountBalanceDto),
    /// Response for
    /// [`GetLedgerNanoStatus`](crate::wallet::message_interface::Message::GetLedgerNanoStatus),
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerNanoStatus(LedgerNanoStatus),
    /// Response for
    /// [`GetIncomingTransaction`](crate::wallet::message_interface::AccountMethod::GetIncomingTransaction),
    IncomingTransaction(Option<Box<TransactionDto>>),
    /// Response for
    /// [`IncomingTransactions`](crate::wallet::message_interface::AccountMethod::IncomingTransactions),
    IncomingTransactions(Vec<TransactionDto>),
    /// Response for
    /// [`ConsolidateOutputs`](crate::wallet::message_interface::AccountMethod::ConsolidateOutputs)
    /// [`ClaimOutputs`](crate::wallet::message_interface::AccountMethod::ClaimOutputs)
    /// [`CreateAliasOutput`](crate::wallet::message_interface::AccountMethod::CreateAliasOutput)
    /// [`SendAmount`](crate::wallet::message_interface::AccountMethod::SendAmount),
    /// [`MintNfts`](crate::wallet::message_interface::AccountMethod::MintNfts),
    /// [`SendAmount`](crate::wallet::message_interface::AccountMethod::SendAmount),
    /// [`SendNativeTokens`](crate::wallet::message_interface::AccountMethod::SendNativeTokens),
    /// [`SendNft`](crate::wallet::message_interface::AccountMethod::SendNft),
    /// [`SendOutputs`](crate::wallet::message_interface::AccountMethod::SendOutputs)
    /// [`SubmitAndStoreTransaction`](crate::wallet::message_interface::AccountMethod::SubmitAndStoreTransaction)
    /// [`Vote`](crate::wallet::message_interface::AccountMethod::Vote)
    /// [`StopParticipating`](crate::wallet::message_interface::AccountMethod::StopParticipating)
    /// [`IncreaseVotingPower`](crate::wallet::message_interface::AccountMethod::IncreaseVotingPower)
    /// [`DecreaseVotingPower`](crate::wallet::message_interface::AccountMethod::DecreaseVotingPower)
    SentTransaction(TransactionDto),
    /// Response for
    /// [`MintNativeToken`](crate::wallet::message_interface::AccountMethod::MintNativeToken),
    MintTokenTransaction(MintTokenTransactionDto),
    /// Response for
    /// [`IsStrongholdPasswordAvailable`](crate::wallet::message_interface::Message::IsStrongholdPasswordAvailable)
    StrongholdPasswordIsAvailable(bool),
    /// An error occurred.
    Error(Error),
    /// A panic occurred.
    Panic(String),
    /// Response for [`GenerateMnemonic`](crate::wallet::message_interface::Message::GenerateMnemonic)
    GeneratedMnemonic(String),
    /// Response for [`GetNodeInfo`](crate::wallet::message_interface::Message::GetNodeInfo)
    NodeInfo(NodeInfoWrapper),
    /// Response for
    /// [`GetParticipationEvent`](crate::wallet::message_interface::AccountMethod::GetParticipationEvent)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEvent(Option<ParticipationEventWithNodes>),
    /// Response for
    /// [`GetParticipationEventIds`](crate::wallet::message_interface::AccountMethod::GetParticipationEventIds)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEventIds(Vec<ParticipationEventId>),
    /// Response for
    /// [`GetParticipationEventStatus`](crate::wallet::message_interface::AccountMethod::GetParticipationEventStatus)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEventStatus(ParticipationEventStatus),
    /// Response for
    /// [`GetParticipationEvents`](crate::wallet::message_interface::AccountMethod::GetParticipationEvents)
    /// [`RegisterParticipationEvent`](crate::wallet::message_interface::AccountMethod::RegisterParticipationEvents)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    ParticipationEvents(HashMap<ParticipationEventId, ParticipationEventWithNodes>),
    /// Response for
    /// [`GetParticipationOverview`](crate::wallet::message_interface::AccountMethod::GetParticipationOverview)
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    AccountParticipationOverview(AccountParticipationOverview),
    /// Response for [`Bech32ToHex`](crate::wallet::message_interface::Message::Bech32ToHex)
    HexAddress(String),
    /// Response for [`HexToBech32`](crate::wallet::message_interface::Message::HexToBech32)
    /// Response for [`GenerateAddress`](crate::wallet::message_interface::Message::GenerateAddress)
    Bech32Address(String),
    /// Response for
    /// [`RequestFundsFromFaucet`](crate::wallet::message_interface::AccountMethod::RequestFundsFromFaucet)
    Faucet(String),
    /// Response for
    /// [`Backup`](crate::wallet::message_interface::Message::Backup),
    /// [`ClearStrongholdPassword`](crate::wallet::message_interface::Message::ClearStrongholdPassword),
    /// [`DeregisterParticipationEvent`](crate::wallet::message_interface::AccountMethod::DeregisterParticipationEvent),
    /// [`RestoreBackup`](crate::wallet::message_interface::Message::RestoreBackup),
    /// [`VerifyMnemonic`](crate::wallet::message_interface::Message::VerifyMnemonic),
    /// [`SetClientOptions`](crate::wallet::message_interface::Message::SetClientOptions),
    /// [`SetStrongholdPassword`](crate::wallet::message_interface::Message::SetStrongholdPassword),
    /// [`SetStrongholdPasswordClearInterval`](crate::wallet::message_interface::Message::SetStrongholdPasswordClearInterval),
    /// [`StoreMnemonic`](crate::wallet::message_interface::Message::StoreMnemonic),
    /// [`StartBackgroundSync`](crate::wallet::message_interface::Message::StartBackgroundSync),
    /// [`StopBackgroundSync`](crate::wallet::message_interface::Message::StopBackgroundSync),
    /// [`EmitTestEvent`](crate::wallet::message_interface::Message::EmitTestEvent),
    Ok(()),
}

// Custom Debug implementation to not log secrets
impl Debug for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Account(account) => write!(f, "Account({account:?})"),
            Self::AccountIndexes(account_indexes) => write!(f, "AccountIndexes({account_indexes:?})"),
            Self::Accounts(accounts) => write!(f, "Accounts({accounts:?})"),
            Self::Addresses(addresses) => write!(f, "Addresses({addresses:?})"),
            Self::AddressesWithUnspentOutputs(addresses) => {
                write!(f, "AddressesWithUnspentOutputs({addresses:?})")
            }
            Self::BlockId(block_id) => write!(f, "BlockId({block_id:?})"),
            Self::Output(output) => write!(f, "Output({output:?})"),
            Self::MinimumRequiredStorageDeposit(amount) => write!(f, "MinimumRequiredStorageDeposit({amount:?})"),
            Self::OutputIds(output_ids) => write!(f, "OutputIds({output_ids:?})"),
            Self::OutputData(output) => write!(f, "OutputData({output:?})"),
            Self::OutputsData(outputs) => write!(f, "OutputsData{outputs:?}"),
            Self::PreparedTransaction(transaction_data) => {
                write!(f, "PreparedTransaction({transaction_data:?})")
            }
            Self::Transaction(transaction) => write!(f, "Transaction({transaction:?})"),
            Self::Transactions(transactions) => write!(f, "Transactions({transactions:?})"),
            Self::SignedTransactionData(signed_transaction_data) => {
                write!(f, "SignedTransactionData({signed_transaction_data:?})")
            }
            Self::GeneratedAddress(addresses) => write!(f, "GeneratedAddress({addresses:?})"),
            Self::Balance(balance) => write!(f, "Balance({balance:?})"),
            Self::IncomingTransaction(transaction) => {
                write!(f, "IncomingTransaction({transaction:?})")
            }
            Self::IncomingTransactions(transactions_data) => {
                write!(f, "IncomingTransactions({transactions_data:?})")
            }
            Self::SentTransaction(transaction) => write!(f, "SentTransaction({transaction:?})"),
            Self::MintTokenTransaction(mint_transaction) => {
                write!(f, "MintTokenTransaction({mint_transaction:?})")
            }
            Self::StrongholdPasswordIsAvailable(is_available) => {
                write!(f, "StrongholdPasswordIsAvailable({is_available:?})")
            }
            Self::Error(error) => write!(f, "Error({error:?})"),
            Self::Panic(panic_msg) => write!(f, "Panic({panic_msg:?})"),
            Self::GeneratedMnemonic(_) => write!(f, "GeneratedMnemonic(<omitted>)"),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNanoStatus(ledger_nano_status) => write!(f, "LedgerNanoStatus({ledger_nano_status:?})"),
            Self::NodeInfo(info) => write!(f, "NodeInfo({info:?})"),
            Self::HexAddress(hex_address) => write!(f, "Hex encoded address({hex_address:?})"),
            Self::Bech32Address(bech32_address) => write!(f, "Bech32 encoded address({bech32_address:?})"),
            Self::Ok(()) => write!(f, "Ok(())"),
            #[cfg(feature = "participation")]
            Self::ParticipationEvent(event) => write!(f, "ParticipationEvent({event:?})"),
            #[cfg(feature = "participation")]
            Self::ParticipationEventStatus(event_status) => write!(f, "ParticipationEventStatus({event_status:?})"),
            #[cfg(feature = "participation")]
            Self::ParticipationEvents(events) => write!(f, "ParticipationEvents({events:?})"),
            #[cfg(feature = "participation")]
            Self::ParticipationEventIds(event_ids) => write!(f, "ParticipationEventIds({event_ids:?})"),
            #[cfg(feature = "participation")]
            Self::AccountParticipationOverview(overview) => {
                write!(f, "AccountParticipationOverview({overview:?})")
            }
            Self::Faucet(response) => write!(f, "Faucet({response:?})"),
        }
    }
}
