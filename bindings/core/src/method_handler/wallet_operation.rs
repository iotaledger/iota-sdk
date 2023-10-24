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
    wallet::{types::TransactionWithMetadataDto, OutputDataDto, PreparedCreateNativeTokenTransactionDto, Wallet},
};

use crate::{method::WalletCommandMethod, Response, Result};

pub(crate) async fn call_wallet_operation_method_internal(
    wallet: &Wallet,
    method: WalletCommandMethod,
) -> Result<Response> {
    let response = match method {
        WalletCommandMethod::ClaimableOutputs { outputs_to_claim } => {
            let output_ids = wallet.claimable_outputs(outputs_to_claim).await?;
            Response::OutputIds(output_ids)
        }
        WalletCommandMethod::ClaimOutputs { output_ids_to_claim } => {
            let transaction = wallet.claim_outputs(output_ids_to_claim.to_vec()).await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::DeregisterParticipationEvent { event_id } => {
            wallet.deregister_participation_event(&event_id).await?;
            Response::Ok
        }
        WalletCommandMethod::GetAddress => {
            let address = wallet.address().await;
            Response::Address(address)
        }
        WalletCommandMethod::GetBalance => Response::Balance(wallet.balance().await?),
        WalletCommandMethod::GetFoundryOutput { token_id } => {
            let output = wallet.get_foundry_output(token_id).await?;
            Response::Output(OutputDto::from(&output))
        }
        WalletCommandMethod::GetIncomingTransaction { transaction_id } => {
            let transaction = wallet.get_incoming_transaction(&transaction_id).await;

            transaction.map_or_else(
                || Response::Transaction(None),
                |transaction| Response::Transaction(Some(Box::new(TransactionWithMetadataDto::from(&transaction)))),
            )
        }
        WalletCommandMethod::GetOutput { output_id } => {
            let output_data = wallet.get_output(&output_id).await;
            Response::OutputData(output_data.as_ref().map(OutputDataDto::from).map(Box::new))
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::GetParticipationEvent { event_id } => {
            let event_and_nodes = wallet.get_participation_event(event_id).await?;
            Response::ParticipationEvent(event_and_nodes)
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::GetParticipationEventIds { node, event_type } => {
            let event_ids = wallet.get_participation_event_ids(&node, event_type).await?;
            Response::ParticipationEventIds(event_ids)
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::GetParticipationEventStatus { event_id } => {
            let event_status = wallet.get_participation_event_status(&event_id).await?;
            Response::ParticipationEventStatus(event_status)
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::GetParticipationEvents => {
            let events = wallet.get_participation_events().await?;
            Response::ParticipationEvents(events)
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::GetParticipationOverview { event_ids } => {
            let overview = wallet.get_participation_overview(event_ids).await?;
            Response::ParticipationOverview(overview)
        }
        WalletCommandMethod::GetTransaction { transaction_id } => {
            let transaction = wallet.get_transaction(&transaction_id).await;
            Response::Transaction(transaction.as_ref().map(TransactionWithMetadataDto::from).map(Box::new))
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::GetVotingPower => {
            let voting_power = wallet.get_voting_power().await?;
            Response::VotingPower(voting_power.to_string())
        }
        WalletCommandMethod::IncomingTransactions => {
            let transactions = wallet.incoming_transactions().await;
            Response::Transactions(transactions.iter().map(TransactionWithMetadataDto::from).collect())
        }
        WalletCommandMethod::Outputs { filter_options } => {
            let outputs = wallet.outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
        WalletCommandMethod::PendingTransactions => {
            let transactions = wallet.pending_transactions().await;
            Response::Transactions(transactions.iter().map(TransactionWithMetadataDto::from).collect())
        }
        WalletCommandMethod::PrepareBurn { burn, options } => {
            let data = wallet.prepare_burn(burn, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletCommandMethod::PrepareConsolidateOutputs { params } => {
            let data = wallet.prepare_consolidate_outputs(params).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletCommandMethod::PrepareCreateAccountOutput { params, options } => {
            let data = wallet.prepare_create_account_output(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletCommandMethod::PrepareMeltNativeToken {
            token_id,
            melt_amount,
            options,
        } => {
            let data = wallet.prepare_melt_native_token(token_id, melt_amount, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::PrepareDecreaseVotingPower { amount } => {
            let data = wallet.prepare_decrease_voting_power(amount).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletCommandMethod::PrepareMintNativeToken {
            token_id,
            mint_amount,
            options,
        } => {
            let data = wallet.prepare_mint_native_token(token_id, mint_amount, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::PrepareIncreaseVotingPower { amount } => {
            let data = wallet.prepare_increase_voting_power(amount).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletCommandMethod::PrepareMintNfts { params, options } => {
            let data = wallet.prepare_mint_nfts(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletCommandMethod::PrepareCreateNativeToken { params, options } => {
            let data = wallet.prepare_create_native_token(params, options).await?;
            Response::PreparedCreateNativeTokenTransaction(PreparedCreateNativeTokenTransactionDto::from(&data))
        }
        WalletCommandMethod::PrepareOutput {
            params,
            transaction_options,
        } => {
            let output = wallet.prepare_output(*params, transaction_options).await?;
            Response::Output(OutputDto::from(&output))
        }
        WalletCommandMethod::PrepareSend { params, options } => {
            let data = wallet.prepare_send(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletCommandMethod::PrepareSendNativeTokens { params, options } => {
            let data = wallet.prepare_send_native_tokens(params.clone(), options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletCommandMethod::PrepareSendNft { params, options } => {
            let data = wallet.prepare_send_nft(params.clone(), options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::PrepareStopParticipating { event_id } => {
            let data = wallet.prepare_stop_participating(event_id).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletCommandMethod::PrepareTransaction { outputs, options } => {
            let token_supply = wallet.client().get_token_supply().await?;
            let data = wallet
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
        WalletCommandMethod::PrepareVote { event_id, answers } => {
            let data = wallet.prepare_vote(event_id, answers).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletCommandMethod::RegisterParticipationEvents { options } => {
            let events = wallet.register_participation_events(&options).await?;
            Response::ParticipationEvents(events)
        }
        WalletCommandMethod::ReissueTransactionUntilIncluded {
            transaction_id,
            interval,
            max_attempts,
        } => {
            let block_id = wallet
                .reissue_transaction_until_included(&transaction_id, interval, max_attempts)
                .await?;
            Response::BlockId(block_id)
        }
        WalletCommandMethod::Send {
            amount,
            address,
            options,
        } => {
            let transaction = wallet.send(amount, address, options).await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        WalletCommandMethod::SendWithParams { params, options } => {
            let transaction = wallet.send_with_params(params, options).await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        WalletCommandMethod::SendOutputs { outputs, options } => {
            let token_supply = wallet.client().get_token_supply().await?;
            let transaction = wallet
                .send_outputs(
                    outputs
                        .into_iter()
                        .map(|o| Ok(Output::try_from_dto_with_params(o, token_supply)?))
                        .collect::<iota_sdk::wallet::Result<Vec<Output>>>()?,
                    options,
                )
                .await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        WalletCommandMethod::SetAlias { alias } => {
            wallet.set_alias(&alias).await?;
            Response::Ok
        }
        WalletCommandMethod::SetDefaultSyncOptions { options } => {
            wallet.set_default_sync_options(options).await?;
            Response::Ok
        }
        WalletCommandMethod::SignAndSubmitTransaction {
            prepared_transaction_data,
        } => {
            let transaction = wallet
                .sign_and_submit_transaction(
                    PreparedTransactionData::try_from_dto_with_params(
                        prepared_transaction_data,
                        wallet.client().get_protocol_parameters().await?,
                    )?,
                    None,
                )
                .await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        WalletCommandMethod::SignTransaction {
            prepared_transaction_data,
        } => {
            let signed_transaction_data = wallet
                .sign_transaction(&PreparedTransactionData::try_from_dto(prepared_transaction_data)?)
                .await?;
            Response::SignedTransactionData(SignedTransactionDataDto::from(&signed_transaction_data))
        }
        WalletCommandMethod::SubmitAndStoreTransaction {
            signed_transaction_data,
        } => {
            let signed_transaction_data = SignedTransactionData::try_from_dto_with_params(
                signed_transaction_data,
                wallet.client().get_protocol_parameters().await?,
            )?;
            let transaction = wallet
                .submit_and_store_transaction(signed_transaction_data, None)
                .await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        WalletCommandMethod::Sync { options } => Response::Balance(wallet.sync(options).await?),
        WalletCommandMethod::Transactions => {
            let transactions = wallet.transactions().await;
            Response::Transactions(transactions.iter().map(TransactionWithMetadataDto::from).collect())
        }
        WalletCommandMethod::UnspentOutputs { filter_options } => {
            let outputs = wallet.unspent_outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
    };
    Ok(response)
}
