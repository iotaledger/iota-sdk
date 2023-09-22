// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::api::{
        PreparedTransactionData, PreparedTransactionDataDto, SignedTransactionData, SignedTransactionDataDto,
    },
    types::{
        block::output::{dto::OutputDto, Output},
        TryFromDto,
    },
    wallet::account::{types::TransactionDto, OutputDataDto, PreparedCreateNativeTokenTransactionDto},
};

use crate::{method::WalletMethod, Response, Result};

pub(crate) async fn call_account_method_internal(account: &Account, method: WalletMethod) -> Result<Response> {
    let response = match method {
        WalletMethod::Addresses => {
            let addresses = account.addresses().await?;
            Response::Addresses(addresses)
        }
        WalletMethod::AddressesWithUnspentOutputs => {
            let addresses = account.addresses_with_unspent_outputs().await?;
            Response::AddressesWithUnspentOutputs(addresses)
        }
        WalletMethod::ClaimableOutputs { outputs_to_claim } => {
            let output_ids = account.claimable_outputs(outputs_to_claim).await?;
            Response::OutputIds(output_ids)
        }
        WalletMethod::ClaimOutputs { output_ids_to_claim } => {
            let transaction = account.claim_outputs(output_ids_to_claim.to_vec()).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        WalletMethod::DeregisterParticipationEvent { event_id } => {
            account.deregister_participation_event(&event_id).await?;
            Response::Ok
        }
        WalletMethod::GenerateEd25519Addresses { amount, options } => {
            let address = account.generate_ed25519_addresses(amount, options).await?;
            Response::GeneratedAccountAddresses(address)
        }
        WalletMethod::GetBalance => Response::Balance(account.balance().await?),
        WalletMethod::GetFoundryOutput { token_id } => {
            let output = account.get_foundry_output(token_id).await?;
            Response::Output(OutputDto::from(&output))
        }
        WalletMethod::GetIncomingTransaction { transaction_id } => {
            let transaction = account.get_incoming_transaction(&transaction_id).await;

            transaction.map_or_else(
                || Response::Transaction(None),
                |transaction| Response::Transaction(Some(Box::new(TransactionDto::from(&transaction)))),
            )
        }
        WalletMethod::GetOutput { output_id } => {
            let output_data = account.get_output(&output_id).await;
            Response::OutputData(output_data.as_ref().map(OutputDataDto::from).map(Box::new))
        }
        #[cfg(feature = "participation")]
        WalletMethod::GetParticipationEvent { event_id } => {
            let event_and_nodes = account.get_participation_event(event_id).await?;
            Response::ParticipationEvent(event_and_nodes)
        }
        #[cfg(feature = "participation")]
        WalletMethod::GetParticipationEventIds { node, event_type } => {
            let event_ids = account.get_participation_event_ids(&node, event_type).await?;
            Response::ParticipationEventIds(event_ids)
        }
        #[cfg(feature = "participation")]
        WalletMethod::GetParticipationEventStatus { event_id } => {
            let event_status = account.get_participation_event_status(&event_id).await?;
            Response::ParticipationEventStatus(event_status)
        }
        #[cfg(feature = "participation")]
        WalletMethod::GetParticipationEvents => {
            let events = account.get_participation_events().await?;
            Response::ParticipationEvents(events)
        }
        #[cfg(feature = "participation")]
        WalletMethod::GetParticipationOverview { event_ids } => {
            let overview = account.get_participation_overview(event_ids).await?;
            Response::AccountParticipationOverview(overview)
        }
        WalletMethod::GetTransaction { transaction_id } => {
            let transaction = account.get_transaction(&transaction_id).await;
            Response::Transaction(transaction.as_ref().map(TransactionDto::from).map(Box::new))
        }
        #[cfg(feature = "participation")]
        WalletMethod::GetVotingPower => {
            let voting_power = account.get_voting_power().await?;
            Response::VotingPower(voting_power.to_string())
        }
        WalletMethod::IncomingTransactions => {
            let transactions = account.incoming_transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        WalletMethod::Outputs { filter_options } => {
            let outputs = account.outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
        WalletMethod::PendingTransactions => {
            let transactions = account.pending_transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        WalletMethod::PrepareBurn { burn, options } => {
            let data = account.prepare_burn(burn, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareConsolidateOutputs { params } => {
            let data = account.prepare_consolidate_outputs(params).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareCreateAccountOutput { params, options } => {
            let data = account.prepare_create_account_output(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareMeltNativeToken {
            token_id,
            melt_amount,
            options,
        } => {
            let data = account
                .prepare_melt_native_token(token_id, melt_amount, options)
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletMethod::PrepareDecreaseVotingPower { amount } => {
            let data = account.prepare_decrease_voting_power(amount).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareMintNativeToken {
            token_id,
            mint_amount,
            options,
        } => {
            let data = account
                .prepare_mint_native_token(token_id, mint_amount, options)
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletMethod::PrepareIncreaseVotingPower { amount } => {
            let data = account.prepare_increase_voting_power(amount).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareMintNfts { params, options } => {
            let data = account.prepare_mint_nfts(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareCreateNativeToken { params, options } => {
            let data = account.prepare_create_native_token(params, options).await?;
            Response::PreparedCreateNativeTokenTransaction(PreparedCreateNativeTokenTransactionDto::from(&data))
        }
        WalletMethod::PrepareOutput {
            params,
            transaction_options,
        } => {
            let output = account.prepare_output(*params, transaction_options).await?;
            Response::Output(OutputDto::from(&output))
        }
        WalletMethod::PrepareSend { params, options } => {
            let data = account.prepare_send(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareSendNativeTokens { params, options } => {
            let data = account.prepare_send_native_tokens(params.clone(), options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareSendNft { params, options } => {
            let data = account.prepare_send_nft(params.clone(), options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletMethod::PrepareStopParticipating { event_id } => {
            let data = account.prepare_stop_participating(event_id).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareTransaction { outputs, options } => {
            let token_supply = account.client().get_token_supply().await?;
            let data = account
                .prepare_transaction(
                    outputs
                        .into_iter()
                        .map(|o| Ok(Output::try_from_dto_with_params(o, token_supply)?))
                        .collect::<Result<Vec<Output>>>()?,
                    options,
                )
                .await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletMethod::PrepareVote { event_id, answers } => {
            let data = account.prepare_vote(event_id, answers).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletMethod::RegisterParticipationEvents { options } => {
            let events = account.register_participation_events(&options).await?;
            Response::ParticipationEvents(events)
        }
        WalletMethod::ReissueTransactionUntilIncluded {
            transaction_id,
            interval,
            max_attempts,
        } => {
            let block_id = account
                .reissue_transaction_until_included(&transaction_id, interval, max_attempts)
                .await?;
            Response::BlockId(block_id)
        }
        WalletMethod::Send {
            amount,
            address,
            options,
        } => {
            let transaction = account.send(amount, address, options).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        WalletMethod::SendWithParams { params, options } => {
            let transaction = account.send_with_params(params, options).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        WalletMethod::SendOutputs { outputs, options } => {
            let token_supply = account.client().get_token_supply().await?;
            let transaction = account
                .send_outputs(
                    outputs
                        .into_iter()
                        .map(|o| Ok(Output::try_from_dto_with_params(o, token_supply)?))
                        .collect::<iota_sdk::wallet::Result<Vec<Output>>>()?,
                    options,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        WalletMethod::SetAlias { alias } => {
            account.set_alias(&alias).await?;
            Response::Ok
        }
        WalletMethod::SetDefaultSyncOptions { options } => {
            account.set_default_sync_options(options).await?;
            Response::Ok
        }
        WalletMethod::SignAndSubmitTransaction {
            prepared_transaction_data,
        } => {
            let transaction = account
                .sign_and_submit_transaction(
                    PreparedTransactionData::try_from_dto_with_params(
                        prepared_transaction_data,
                        account.client().get_protocol_parameters().await?,
                    )?,
                    None,
                )
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        WalletMethod::SignTransactionEssence {
            prepared_transaction_data,
        } => {
            let signed_transaction_data = account
                .sign_transaction_essence(&PreparedTransactionData::try_from_dto(prepared_transaction_data)?)
                .await?;
            Response::SignedTransactionData(SignedTransactionDataDto::from(&signed_transaction_data))
        }
        WalletMethod::SubmitAndStoreTransaction {
            signed_transaction_data,
        } => {
            let signed_transaction_data = SignedTransactionData::try_from_dto_with_params(
                signed_transaction_data,
                account.client().get_protocol_parameters().await?,
            )?;
            let transaction = account
                .submit_and_store_transaction(signed_transaction_data, None)
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        WalletMethod::Sync { options } => Response::Balance(account.sync(options).await?),
        WalletMethod::Transactions => {
            let transactions = account.transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        WalletMethod::UnspentOutputs { filter_options } => {
            let outputs = account.unspent_outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
    };
    Ok(response)
}
