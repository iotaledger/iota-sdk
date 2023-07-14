// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::{Debug, Formatter, Result};

#[cfg(feature = "ledger_nano")]
use iota_sdk::client::secret::LedgerNanoStatus;
use iota_sdk::{
    client::{
        api::{PreparedTransactionDataDto, SignedTransactionDataDto},
        NodeInfoWrapper,
    },
    types::block::{
        address::Bech32Address,
        output::{dto::OutputDto, OutputId},
        BlockId,
    },
    wallet::{
        account::{
            types::{AccountAddress, AddressWithUnspentOutputs, Balance, TransactionDto},
            AccountDetailsDto, CreateNativeTokenTransactionDto, OutputDataDto,
        },
        Error,
    },
};
use serde::Serialize;
#[cfg(feature = "participation")]
use {
    iota_sdk::types::api::plugins::participation::types::{ParticipationEventId, ParticipationEventStatus},
    iota_sdk::wallet::account::{AccountParticipationOverview, ParticipationEventWithNodes},
    std::collections::HashMap,
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
    /// [`ClaimableOutputs`](crate::wallet::message_interface::AccountMethod::ClaimableOutputs)
    OutputIds(Vec<OutputId>),
    /// Response for [`GetOutput`](crate::wallet::message_interface::AccountMethod::GetOutput)
    OutputData(Option<Box<OutputDataDto>>),
    /// Response for
    /// [`Outputs`](crate::wallet::message_interface::AccountMethod::Outputs),
    /// [`UnspentOutputs`](crate::wallet::message_interface::AccountMethod::UnspentOutputs)
    OutputsData(Vec<OutputDataDto>),
    /// Response for
    /// [`PrepareSend`](crate::wallet::message_interface::AccountMethod::PrepareSend),
    /// [`PrepareTransaction`](crate::wallet::message_interface::AccountMethod::PrepareTransaction)
    PreparedTransaction(PreparedTransactionDataDto),
    /// Response for
    /// [`GetTransaction`](crate::wallet::message_interface::AccountMethod::GetTransaction),
    /// [`GetIncomingTransaction`](crate::wallet::message_interface::AccountMethod::GetIncomingTransaction)
    Transaction(Option<Box<TransactionDto>>),
    /// Response for
    /// [`Transactions`](crate::wallet::message_interface::AccountMethod::Transactions),
    /// [`PendingTransactions`](crate::wallet::message_interface::AccountMethod::PendingTransactions),
    /// [`IncomingTransactions`](crate::wallet::message_interface::AccountMethod::IncomingTransactions)
    Transactions(Vec<TransactionDto>),
    /// Response for
    /// [`SignTransactionEssence`](crate::wallet::message_interface::AccountMethod::SignTransactionEssence)
    /// [`SubmitAndStoreTransaction`](crate::wallet::message_interface::AccountMethod::SubmitAndStoreTransaction)
    SignedTransactionData(SignedTransactionDataDto),
    /// GenerateAddress response.
    /// Response for
    /// [`GenerateEd25519Addresses`](crate::wallet::message_interface::AccountMethod::GenerateEd25519Addresses)
    GeneratedEd25519Addresses(Vec<AccountAddress>),
    /// GenerateAddress response.
    /// Response for
    /// [`GenerateEvmAddresses`](crate::wallet::message_interface::AccountMethod::GenerateEvmAddresses)
    GeneratedEvmAddresses(Vec<String>),
    /// Response for:
    /// - [`SignSecp256k1Ecdsa`](crate::wallet::message_interface::AccountMethod::SignSecp256k1Ecdsa)
    #[serde(rename_all = "camelCase")]
    Secp256k1EcdsaSignature { public_key: String, signature: String },
    /// Response for
    /// [`GetBalance`](crate::wallet::message_interface::AccountMethod::GetBalance),
    /// [`SyncAccount`](crate::wallet::message_interface::AccountMethod::SyncAccount)
    Balance(Balance),
    /// Response for
    /// [`GetLedgerNanoStatus`](crate::wallet::message_interface::Message::GetLedgerNanoStatus),
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerNanoStatus(LedgerNanoStatus),
    /// Response for
    /// [`ConsolidateOutputs`](crate::wallet::message_interface::AccountMethod::ConsolidateOutputs)
    /// [`ClaimOutputs`](crate::wallet::message_interface::AccountMethod::ClaimOutputs)
    /// [`CreateAliasOutput`](crate::wallet::message_interface::AccountMethod::CreateAliasOutput)
    /// [`MintNfts`](crate::wallet::message_interface::AccountMethod::MintNfts),
    /// [`Send`](crate::wallet::message_interface::AccountMethod::Send),
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
    /// [`CreateNativeToken`](crate::wallet::message_interface::AccountMethod::CreateNativeToken),
    CreateNativeTokenTransaction(CreateNativeTokenTransactionDto),
    /// Response for
    /// [`IsStrongholdPasswordAvailable`](crate::wallet::message_interface::Message::IsStrongholdPasswordAvailable)
    /// [`VerifyEd25519Signature`](crate::wallet::message_interface::account_method::AccountMethod::VerifyEd25519Signature)
    /// [`VerifySecp256k1EcdsaSignature`](crate::wallet::message_interface::account_method::AccountMethod::VerifySecp256k1EcdsaSignature)
    Bool(bool),
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
    /// Response for [`GenerateEd25519Address`](crate::wallet::message_interface::Message::GenerateEd25519Address)
    Bech32Address(Bech32Address),
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
            Self::GeneratedEd25519Addresses(addresses) => write!(f, "GeneratedEd25519Addresses({addresses:?})"),
            Self::GeneratedEvmAddresses(addresses) => write!(f, "GeneratedEvmAddresses({addresses:?})"),
            Self::Secp256k1EcdsaSignature { public_key, signature } => {
                write!(
                    f,
                    "Secp256k1EcdsaSignature{{ public_key: {public_key:?}, signature: {signature:?} }}"
                )
            }
            Self::Balance(balance) => write!(f, "Balance({balance:?})"),
            Self::SentTransaction(transaction) => write!(f, "SentTransaction({transaction:?})"),
            Self::CreateNativeTokenTransaction(create_transaction) => {
                write!(f, "CreateNativeTokenTransaction({create_transaction:?})")
            }
            Self::Bool(b) => {
                write!(f, "Bool({b})")
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
