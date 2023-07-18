// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "participation")]
use std::str::FromStr;

use iota_sdk::{
    client::api::{
        input_selection::Burn, PreparedTransactionData, PreparedTransactionDataDto, SignedTransactionData,
        SignedTransactionDataDto,
    },
    types::block::output::{dto::OutputDto, Output},
    wallet::account::{
        types::TransactionDto, Account, OutputDataDto, PreparedCreateNativeTokenTransactionDto, TransactionOptions,
    },
};

use crate::{method::AccountMethod, Response, Result};

pub(crate) async fn call_account_method_internal(account: &Account, method: AccountMethod) -> Result<Response> {
    let response = match method {
        AccountMethod::Addresses => {
            let addresses = account.addresses().await?;
            Response::Addresses(addresses)
        }
        AccountMethod::AddressesWithUnspentOutputs => {
            let addresses = account.addresses_with_unspent_outputs().await?;
            Response::AddressesWithUnspentOutputs(addresses)
        }
        AccountMethod::ClaimableOutputs { outputs_to_claim } => {
            let output_ids = account.claimable_outputs(outputs_to_claim).await?;
            Response::OutputIds(output_ids)
        }
        AccountMethod::ClaimOutputs { output_ids_to_claim } => {
            let transaction = account.claim_outputs(output_ids_to_claim.to_vec()).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        AccountMethod::DeregisterParticipationEvent { event_id } => {
            account.deregister_participation_event(&event_id).await?;
            Response::Ok
        }
        AccountMethod::GenerateEd25519Addresses { amount, options } => {
            let address = account.generate_ed25519_addresses(amount, options).await?;
            Response::GeneratedAccountAddresses(address)
        }
        AccountMethod::GetBalance => Response::Balance(account.balance().await?),
        AccountMethod::GetFoundryOutput { token_id } => {
            let output = account.get_foundry_output(token_id).await?;
            Response::Output(OutputDto::from(&output))
        }
        AccountMethod::GetIncomingTransaction { transaction_id } => {
            let transaction = account.get_incoming_transaction(&transaction_id).await;

            transaction.map_or_else(
                || Response::Transaction(None),
                |transaction| Response::Transaction(Some(Box::new(TransactionDto::from(&transaction)))),
            )
        }
        AccountMethod::GetOutput { output_id } => {
            let output_data = account.get_output(&output_id).await;
            Response::OutputData(output_data.as_ref().map(OutputDataDto::from).map(Box::new))
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
        #[cfg(feature = "participation")]
        AccountMethod::GetParticipationOverview { event_ids } => {
            let overview = account.get_participation_overview(event_ids).await?;
            Response::AccountParticipationOverview(overview)
        }
        AccountMethod::GetTransaction { transaction_id } => {
            let transaction = account.get_transaction(&transaction_id).await;
            Response::Transaction(transaction.as_ref().map(TransactionDto::from).map(Box::new))
        }
        #[cfg(feature = "participation")]
        AccountMethod::GetVotingPower => {
            let voting_power = account.get_voting_power().await?;
            Response::VotingPower(voting_power.to_string())
        }
        AccountMethod::IncomingTransactions => {
            let transactions = account.incoming_transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        AccountMethod::Outputs { filter_options } => {
            let outputs = account.outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
        AccountMethod::PendingTransactions => {
            let transactions = account.pending_transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        AccountMethod::PrepareBurn { burn, options } => {
            let data = account
                .prepare_burn(
                    Burn::try_from(burn)?,
                    options.map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareConsolidateOutputs {
            force,
            output_consolidation_threshold,
        } => {
            let data = account
                .prepare_consolidate_outputs(force, output_consolidation_threshold)
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareCreateAliasOutput { params, options } => {
            let data = account
                .prepare_create_alias_output(params, options.map(TransactionOptions::try_from_dto).transpose()?)
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareMeltNativeToken {
            token_id,
            melt_amount,
            options,
        } => {
            let data = account
                .prepare_melt_native_token(
                    token_id,
                    melt_amount,
                    options.map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        AccountMethod::PrepareDecreaseVotingPower { amount } => {
            let data = account
                .prepare_decrease_voting_power(
                    u64::from_str(&amount).map_err(|_| iota_sdk::client::Error::InvalidAmount(amount.clone()))?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareMintNativeToken {
            token_id,
            mint_amount,
            options,
        } => {
            let data = account
                .prepare_mint_native_token(
                    token_id,
                    mint_amount,
                    options.map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        AccountMethod::PrepareIncreaseVotingPower { amount } => {
            let data = account
                .prepare_increase_voting_power(
                    u64::from_str(&amount).map_err(|_| iota_sdk::client::Error::InvalidAmount(amount.clone()))?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareMintNfts { params, options } => {
            let data = account
                .prepare_mint_nfts(params, options.map(TransactionOptions::try_from_dto).transpose()?)
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareCreateNativeToken { params, options } => {
            let data = account
                .prepare_create_native_token(params, options.map(TransactionOptions::try_from_dto).transpose()?)
                .await?;
            Response::PreparedCreateNativeTokenTransaction(PreparedCreateNativeTokenTransactionDto::from(&data))
        }
        AccountMethod::PrepareOutput {
            params,
            transaction_options,
        } => {
            let output = account
                .prepare_output(
                    *params,
                    transaction_options.map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::Output(OutputDto::from(&output))
        }
        AccountMethod::PrepareSend { params, options } => {
            let data = account
                .prepare_send(params, options.map(TransactionOptions::try_from_dto).transpose()?)
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareSendNativeTokens { params, options } => {
            let data = account
                .prepare_send_native_tokens(
                    params.clone(),
                    options.map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareSendNft { params, options } => {
            let data = account
                .prepare_send_nft(
                    params.clone(),
                    options.map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        AccountMethod::PrepareStopParticipating { event_id } => {
            let data = account.prepare_stop_participating(event_id).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        AccountMethod::PrepareTransaction { outputs, options } => {
            let token_supply = account.client().get_token_supply().await?;
            let data = account
                .prepare_transaction(
                    outputs
                        .into_iter()
                        .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                        .collect::<Result<Vec<Output>>>()?,
                    options.map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        AccountMethod::PrepareVote { event_id, answers } => {
            let data = account.prepare_vote(event_id, answers).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        AccountMethod::RegisterParticipationEvents { options } => {
            let events = account.register_participation_events(&options).await?;
            Response::ParticipationEvents(events)
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
        AccountMethod::Send {
            amount,
            address,
            options,
        } => {
            let transaction = account
                .send(
                    amount,
                    address,
                    options.map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SendWithParams { params, options } => {
            let transaction = account
                .send_with_params(params, options.map(TransactionOptions::try_from_dto).transpose()?)
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SendOutputs { outputs, options } => {
            let token_supply = account.client().get_token_supply().await?;
            let transaction = account
                .send_outputs(
                    outputs
                        .into_iter()
                        .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                        .collect::<iota_sdk::wallet::Result<Vec<Output>>>()?,
                    options.map(TransactionOptions::try_from_dto).transpose()?,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SetAlias { alias } => {
            account.set_alias(&alias).await?;
            Response::Ok
        }
        AccountMethod::SetDefaultSyncOptions { options } => {
            account.set_default_sync_options(options).await?;
            Response::Ok
        }
        AccountMethod::SignAndSubmitTransaction {
            prepared_transaction_data,
        } => {
            let transaction = account
                .sign_and_submit_transaction(
                    PreparedTransactionData::try_from_dto(
                        prepared_transaction_data,
                        &account.client().get_protocol_parameters().await?,
                    )?,
                    None,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::SignTransactionEssence {
            prepared_transaction_data,
        } => {
            let signed_transaction_data = account
                .sign_transaction_essence(&PreparedTransactionData::try_from_dto(
                    prepared_transaction_data,
                    &account.client().get_protocol_parameters().await?,
                )?)
                .await?;
            Response::SignedTransactionData(SignedTransactionDataDto::from(&signed_transaction_data))
        }
        AccountMethod::SubmitAndStoreTransaction {
            signed_transaction_data,
        } => {
            let signed_transaction_data = SignedTransactionData::try_from_dto(
                signed_transaction_data,
                &account.client().get_protocol_parameters().await?,
            )?;
            let transaction = account
                .submit_and_store_transaction(signed_transaction_data, None)
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        AccountMethod::Sync { options } => Response::Balance(account.sync(options).await?),
        AccountMethod::Transactions => {
            let transactions = account.transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        AccountMethod::UnspentOutputs { filter_options } => {
            let outputs = account.unspent_outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
    };
    Ok(response)
}
