// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "participation")]
use std::str::FromStr;

use iota_sdk::{
    client::api::{
        PreparedTransactionData, PreparedTransactionDataDto, SignedTransactionData, SignedTransactionDataDto,
    },
    types::block::{
        output::{dto::OutputDto, AliasId, NftId, Output, Rent, TokenId},
        DtoError,
    },
    wallet::{
        account::{
            types::{AccountBalanceDto, TransactionDto},
            AccountHandle, AliasOutputOptions, MintTokenTransactionDto, OutputDataDto, OutputOptions,
            TransactionOptions,
        },
        message_interface::AddressWithUnspentOutputsDto,
        AddressWithAmount, IncreaseNativeTokenSupplyOptions, NativeTokenOptions, NftOptions,
    },
};
use primitive_types::U256;

use super::Result;
use crate::{method::AccountMethod, Response};

pub(crate) async fn call_account_method(account: &AccountHandle, method: AccountMethod) -> Result<Response> {
    let response = match method {
        AccountMethod::BurnNativeToken {
            token_id,
            burn_amount,
            options,
        } => {
            let transaction = account
                .burn_native_token(
                    TokenId::try_from(&token_id)?,
                    U256::try_from(&burn_amount).map_err(|_| DtoError::InvalidField("burn_amount"))?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::BurnNft { nft_id, options } => {
            let transaction = account
                .burn_nft(
                    NftId::try_from(&nft_id)?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::ConsolidateOutputs {
            force,
            output_consolidation_threshold,
        } => {
            let transaction = account
                .consolidate_outputs(force, output_consolidation_threshold)
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::CreateAliasOutput {
            alias_output_options,
            options,
        } => {
            let alias_output_options = alias_output_options
                .map(|options| AliasOutputOptions::try_from(&options))
                .transpose()?;

            let transaction = account
                .create_alias_output(
                    alias_output_options,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::DestroyAlias { alias_id, options } => {
            let transaction = account
                .destroy_alias(
                    AliasId::try_from(&alias_id)?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::DestroyFoundry { foundry_id, options } => {
            let transaction = account
                .destroy_foundry(
                    foundry_id,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::GenerateAddresses { amount, options } => {
            let address = account.generate_addresses(amount, options).await?;
            Response::GeneratedAddress(address)
        }
        AccountMethod::GetOutputsWithAdditionalUnlockConditions { outputs_to_claim } => {
            let output_ids = account
                .get_unlockable_outputs_with_additional_unlock_conditions(outputs_to_claim)
                .await?;
            Response::OutputIds(output_ids)
        }
        AccountMethod::GetOutput { output_id } => {
            let output_data = account.get_output(&output_id).await;
            Response::OutputData(output_data.as_ref().map(OutputDataDto::from).map(Box::new))
        }
        AccountMethod::GetFoundryOutput { token_id } => {
            let token_id = TokenId::try_from(&token_id)?;
            let output = account.get_foundry_output(token_id).await?;
            Response::Output(OutputDto::from(&output))
        }
        AccountMethod::GetTransaction { transaction_id } => {
            let transaction = account.get_transaction(&transaction_id).await;
            Response::Transaction(transaction.as_ref().map(TransactionDto::from).map(Box::new))
        }
        AccountMethod::GetIncomingTransactionData { transaction_id } => {
            let transaction = account.get_incoming_transaction_data(&transaction_id).await;

            transaction.map_or_else(
                || Response::IncomingTransactionData(None),
                |transaction| {
                    Response::IncomingTransactionData(Some(Box::new((
                        transaction_id,
                        TransactionDto::from(&transaction),
                    ))))
                },
            )
        }
        AccountMethod::Addresses => {
            let addresses = account.addresses().await?;
            Response::Addresses(addresses)
        }
        AccountMethod::AddressesWithUnspentOutputs => {
            let addresses = account.addresses_with_unspent_outputs().await?;
            Response::AddressesWithUnspentOutputs(addresses.iter().map(AddressWithUnspentOutputsDto::from).collect())
        }
        AccountMethod::Outputs { filter_options } => {
            let outputs = account.outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
        AccountMethod::UnspentOutputs { filter_options } => {
            let outputs = account.unspent_outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
        AccountMethod::IncomingTransactions => {
            let transactions = account.incoming_transactions().await?;
            Response::IncomingTransactionsData(
                transactions
                    .into_iter()
                    .map(|d| (d.0, TransactionDto::from(&d.1)))
                    .collect(),
            )
        }
        AccountMethod::Transactions => {
            let transactions = account.transactions().await?;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        AccountMethod::PendingTransactions => {
            let transactions = account.pending_transactions().await?;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        AccountMethod::DecreaseNativeTokenSupply {
            token_id,
            melt_amount,
            options,
        } => {
            let transaction = account
                .decrease_native_token_supply(
                    TokenId::try_from(&token_id)?,
                    U256::try_from(&melt_amount).map_err(|_| DtoError::InvalidField("melt_amount"))?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::IncreaseNativeTokenSupply {
            token_id,
            mint_amount,
            increase_native_token_supply_options,
            options,
        } => {
            let increase_native_token_supply_options = match increase_native_token_supply_options {
                Some(native_token_options) => Some(IncreaseNativeTokenSupplyOptions::try_from(&native_token_options)?),
                None => None,
            };
            let transaction = account
                .increase_native_token_supply(
                    TokenId::try_from(&token_id)?,
                    U256::try_from(&mint_amount).map_err(|_| DtoError::InvalidField("mint_amount"))?,
                    increase_native_token_supply_options,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::MintTokenTransaction(MintTokenTransactionDto::from(&transaction))
        }
        AccountMethod::MintNativeToken {
            native_token_options,
            options,
        } => {
            let transaction = account
                .mint_native_token(
                    NativeTokenOptions::try_from(&native_token_options)?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::MintTokenTransaction(MintTokenTransactionDto::from(&transaction))
        }
        AccountMethod::MinimumRequiredStorageDeposit { output } => {
            let output = Output::try_from_dto(&output, account.client().get_token_supply().await?)?;
            let rent_structure = account.client().get_rent_structure().await?;

            let minimum_storage_deposit = output.rent_cost(&rent_structure);

            Response::MinimumRequiredStorageDeposit(minimum_storage_deposit.to_string())
        }
        AccountMethod::MintNfts { nfts_options, options } => {
            let transaction = account
                .mint_nfts(
                    nfts_options
                        .iter()
                        .map(NftOptions::try_from)
                        .collect::<iota_sdk::wallet::Result<Vec<NftOptions>>>()?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::GetBalance => Response::Balance(AccountBalanceDto::from(&account.balance().await?)),
        AccountMethod::PrepareOutput {
            options,
            transaction_options,
        } => {
            let output = account
                .prepare_output(
                    OutputOptions::try_from(&options)?,
                    transaction_options
                        .as_ref()
                        .map(TransactionOptions::try_from_dto)
                        .transpose()?,
                )
                .await?;
            Response::Output(OutputDto::from(&output))
        }
        AccountMethod::PrepareSendAmount {
            addresses_with_amount,
            options,
        } => {
            let data = account
                .prepare_send_amount(
                    addresses_with_amount
                        .iter()
                        .map(AddressWithAmount::try_from)
                        .collect::<iota_sdk::wallet::Result<Vec<AddressWithAmount>>>()?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareTransaction { outputs, options } => {
            let token_supply = account.client().get_token_supply().await?;
            let data = account
                .prepare_transaction(
                    outputs
                        .iter()
                        .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                        .collect::<Result<Vec<Output>>>()?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::RetryTransactionUntilIncluded {
            transaction_id,
            interval,
            max_attempts,
        } => {
            let block_id = account
                .retry_transaction_until_included(&transaction_id, interval, max_attempts)
                .await?;
            Response::BlockId(block_id)
        }
        AccountMethod::SyncAccount { options } => {
            Response::Balance(AccountBalanceDto::from(&account.sync(options).await?))
        }
        AccountMethod::SendAmount {
            addresses_with_amount,
            options,
        } => {
            let transaction = account
                .send_amount(
                    addresses_with_amount
                        .iter()
                        .map(AddressWithAmount::try_from)
                        .collect::<iota_sdk::wallet::Result<Vec<AddressWithAmount>>>()?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SendNativeTokens {
            addresses_native_tokens,
            options,
        } => {
            let transaction = account
                .send_native_tokens(
                    addresses_native_tokens.clone(),
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SendNft {
            addresses_nft_ids,
            options,
        } => {
            let transaction = account
                .send_nft(
                    addresses_nft_ids.clone(),
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SetAlias { alias } => {
            account.set_alias(&alias).await?;
            Response::Ok
        }
        AccountMethod::SendOutputs { outputs, options } => {
            let token_supply = account.client().get_token_supply().await?;
            let transaction = account
                .send(
                    outputs
                        .iter()
                        .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                        .collect::<iota_sdk::wallet::Result<Vec<Output>>>()?,
                    options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SignTransactionEssence {
            prepared_transaction_data,
        } => {
            let signed_transaction_data = account
                .sign_transaction_essence(&PreparedTransactionData::try_from_dto(
                    &prepared_transaction_data,
                    &account.client().get_protocol_parameters().await?,
                )?)
                .await?;
            Response::SignedTransactionData(SignedTransactionDataDto::from(&signed_transaction_data))
        }
        AccountMethod::SubmitAndStoreTransaction {
            signed_transaction_data,
        } => {
            let signed_transaction_data = SignedTransactionData::try_from_dto(
                &signed_transaction_data,
                &account.client().get_protocol_parameters().await?,
            )?;
            let transaction = account.submit_and_store_transaction(signed_transaction_data).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::ClaimOutputs { output_ids_to_claim } => {
            let transaction = account.claim_outputs(output_ids_to_claim.to_vec()).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::Vote { event_id, answers } => {
            let transaction = account.vote(event_id, answers).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::StopParticipating { event_id } => {
            let transaction = account.stop_participating(event_id).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetVotingPower => {
            let voting_power = account.get_voting_power().await?;
            Response::VotingPower(voting_power.to_string())
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationOverview { event_ids } => {
            let overview = account.get_participation_overview(event_ids).await?;
            Response::AccountParticipationOverview(overview)
        }
        #[cfg(feature = "participation")]
        AccountMethod::IncreaseVotingPower { amount } => {
            let transaction = account
                .increase_voting_power(
                    u64::from_str(&amount).map_err(|_| iota_sdk::client::Error::InvalidAmount(amount.clone()))?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::DecreaseVotingPower { amount } => {
            let transaction = account
                .decrease_voting_power(
                    u64::from_str(&amount).map_err(|_| iota_sdk::client::Error::InvalidAmount(amount.clone()))?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::RegisterParticipationEvents { options } => {
            let events = account.register_participation_events(&options).await?;
            Response::ParticipationEvents(events)
        }
        #[cfg(feature = "participation")]
        AccountMethod::DeregisterParticipationEvent { event_id } => {
            account.deregister_participation_event(&event_id).await?;
            Response::Ok
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationEvent { event_id } => {
            let event_and_nodes = account.get_participation_event(event_id).await?;
            Response::ParticipationEvent(event_and_nodes)
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationEventIds { node, event_type } => {
            let event_ids = account.get_participation_event_ids(&node, event_type).await?;
            Response::ParticipationEventIds(event_ids)
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationEventStatus { event_id } => {
            let event_status = account.get_participation_event_status(&event_id).await?;
            Response::ParticipationEventStatus(event_status)
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationEvents => {
            let events = account.get_participation_events().await?;
            Response::ParticipationEvents(events)
        }
    };
    Ok(response)
}
