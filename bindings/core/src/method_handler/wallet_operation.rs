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
    wallet::{types::TransactionDto, OutputDataDto, PreparedCreateNativeTokenTransactionDto, Wallet},
};

use crate::{method::WalletOperationMethod, Response, Result};

pub(crate) async fn call_wallet_operation_method_internal(
    wallet: &Wallet,
    method: WalletOperationMethod,
) -> Result<Response> {
    let response = match method {
        // TODO: remove
        // WalletOperationMethod::AddressesWithUnspentOutputs => {
        //     let addresses = wallet.unspent_outputs().await?;
        //     Response::AddressesWithUnspentOutputs(addresses)
        // }
        WalletOperationMethod::ClaimableOutputs { outputs_to_claim } => {
            let output_ids = wallet.claimable_outputs(outputs_to_claim).await?;
            Response::OutputIds(output_ids)
        }
        WalletOperationMethod::ClaimOutputs { output_ids_to_claim } => {
            let transaction = wallet.claim_outputs(output_ids_to_claim.to_vec()).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::DeregisterParticipationEvent { event_id } => {
            wallet.deregister_participation_event(&event_id).await?;
            Response::Ok
        }
        WalletOperationMethod::GenerateEd25519Addresses { num, options } => {
            // let address = wallet.generate_ed25519_address(num, options).await?;
            // Response::GeneratedAccountAddress(address)
            todo!("generate ed25519 addresses")
        }
        WalletOperationMethod::GetAddress => {
            let address = wallet.address().await;
            Response::Address(address)
        }
        WalletOperationMethod::GetBalance => Response::Balance(wallet.balance().await?),
        WalletOperationMethod::GetFoundryOutput { token_id } => {
            let output = wallet.get_foundry_output(token_id).await?;
            Response::Output(OutputDto::from(&output))
        }
        WalletOperationMethod::GetIncomingTransaction { transaction_id } => {
            let transaction = wallet.get_incoming_transaction(&transaction_id).await;

            transaction.map_or_else(
                || Response::Transaction(None),
                |transaction| Response::Transaction(Some(Box::new(TransactionDto::from(&transaction)))),
            )
        }
        WalletOperationMethod::GetOutput { output_id } => {
            let output_data = wallet.get_output(&output_id).await;
            Response::OutputData(output_data.as_ref().map(OutputDataDto::from).map(Box::new))
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::GetParticipationEvent { event_id } => {
            let event_and_nodes = wallet.get_participation_event(event_id).await?;
            Response::ParticipationEvent(event_and_nodes)
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::GetParticipationEventIds { node, event_type } => {
            let event_ids = wallet.get_participation_event_ids(&node, event_type).await?;
            Response::ParticipationEventIds(event_ids)
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::GetParticipationEventStatus { event_id } => {
            let event_status = wallet.get_participation_event_status(&event_id).await?;
            Response::ParticipationEventStatus(event_status)
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::GetParticipationEvents => {
            let events = wallet.get_participation_events().await?;
            Response::ParticipationEvents(events)
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::GetParticipationOverview { event_ids } => {
            let overview = wallet.get_participation_overview(event_ids).await?;
            Response::ParticipationOverview(overview)
        }
        WalletOperationMethod::GetTransaction { transaction_id } => {
            let transaction = wallet.get_transaction(&transaction_id).await;
            Response::Transaction(transaction.as_ref().map(TransactionDto::from).map(Box::new))
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::GetVotingPower => {
            let voting_power = wallet.get_voting_power().await?;
            Response::VotingPower(voting_power.to_string())
        }
        WalletOperationMethod::IncomingTransactions => {
            let transactions = wallet.incoming_transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        WalletOperationMethod::Outputs { filter_options } => {
            let outputs = wallet.outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
        WalletOperationMethod::PendingTransactions => {
            let transactions = wallet.pending_transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        WalletOperationMethod::PrepareBurn { burn, options } => {
            let data = wallet.prepare_burn(burn, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletOperationMethod::PrepareConsolidateOutputs { params } => {
            let data = wallet.prepare_consolidate_outputs(params).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletOperationMethod::PrepareCreateAccountOutput { params, options } => {
            let data = wallet.prepare_create_account_output(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletOperationMethod::PrepareMeltNativeToken {
            token_id,
            melt_amount,
            options,
        } => {
            let data = wallet.prepare_melt_native_token(token_id, melt_amount, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::PrepareDecreaseVotingPower { amount } => {
            let data = wallet.prepare_decrease_voting_power(amount).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletOperationMethod::PrepareMintNativeToken {
            token_id,
            mint_amount,
            options,
        } => {
            let data = wallet.prepare_mint_native_token(token_id, mint_amount, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::PrepareIncreaseVotingPower { amount } => {
            let data = wallet.prepare_increase_voting_power(amount).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletOperationMethod::PrepareMintNfts { params, options } => {
            let data = wallet.prepare_mint_nfts(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletOperationMethod::PrepareCreateNativeToken { params, options } => {
            let data = wallet.prepare_create_native_token(params, options).await?;
            Response::PreparedCreateNativeTokenTransaction(PreparedCreateNativeTokenTransactionDto::from(&data))
        }
        WalletOperationMethod::PrepareOutput {
            params,
            transaction_options,
        } => {
            let output = wallet.prepare_output(*params, transaction_options).await?;
            Response::Output(OutputDto::from(&output))
        }
        WalletOperationMethod::PrepareSend { params, options } => {
            let data = wallet.prepare_send(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletOperationMethod::PrepareSendNativeTokens { params, options } => {
            let data = wallet.prepare_send_native_tokens(params.clone(), options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletOperationMethod::PrepareSendNft { params, options } => {
            let data = wallet.prepare_send_nft(params.clone(), options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::PrepareStopParticipating { event_id } => {
            let data = wallet.prepare_stop_participating(event_id).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletOperationMethod::PrepareTransaction { outputs, options } => {
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
        WalletOperationMethod::PrepareVote { event_id, answers } => {
            let data = wallet.prepare_vote(event_id, answers).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        #[cfg(feature = "participation")]
        WalletOperationMethod::RegisterParticipationEvents { options } => {
            let events = wallet.register_participation_events(&options).await?;
            Response::ParticipationEvents(events)
        }
        WalletOperationMethod::ReissueTransactionUntilIncluded {
            transaction_id,
            interval,
            max_attempts,
        } => {
            let block_id = wallet
                .reissue_transaction_until_included(&transaction_id, interval, max_attempts)
                .await?;
            Response::BlockId(block_id)
        }
        WalletOperationMethod::Send {
            amount,
            address,
            options,
        } => {
            let transaction = wallet.send(amount, address, options).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        WalletOperationMethod::SendWithParams { params, options } => {
            let transaction = wallet.send_with_params(params, options).await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        WalletOperationMethod::SendOutputs { outputs, options } => {
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
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        WalletOperationMethod::SetAlias { alias } => {
            wallet.set_alias(&alias).await?;
            Response::Ok
        }
        WalletOperationMethod::SetDefaultSyncOptions { options } => {
            wallet.set_default_sync_options(options).await?;
            Response::Ok
        }
        WalletOperationMethod::SignAndSubmitTransaction {
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
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        WalletOperationMethod::SignTransactionEssence {
            prepared_transaction_data,
        } => {
            let signed_transaction_data = wallet
                .sign_transaction_essence(&PreparedTransactionData::try_from_dto(prepared_transaction_data)?)
                .await?;
            Response::SignedTransactionData(SignedTransactionDataDto::from(&signed_transaction_data))
        }
        WalletOperationMethod::SubmitAndStoreTransaction {
            signed_transaction_data,
        } => {
            let signed_transaction_data = SignedTransactionData::try_from_dto_with_params(
                signed_transaction_data,
                wallet.client().get_protocol_parameters().await?,
            )?;
            let transaction = wallet
                .submit_and_store_transaction(signed_transaction_data, None)
                .await?;
            Response::SentTransaction(TransactionDto::from(&transaction))
        }
        WalletOperationMethod::Sync { options } => Response::Balance(wallet.sync(options).await?),
        WalletOperationMethod::Transactions => {
            let transactions = wallet.transactions().await;
            Response::Transactions(transactions.iter().map(TransactionDto::from).collect())
        }
        WalletOperationMethod::UnspentOutputs { filter_options } => {
            let outputs = wallet.unspent_outputs(filter_options).await?;
            Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect())
        }
    };
    Ok(response)
}
