// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use crypto::signatures::ed25519::PublicKey;
use iota_sdk::{
    client::api::{
        PreparedTransactionData, PreparedTransactionDataDto, SignedTransactionData, SignedTransactionDataDto,
    },
    types::{block::address::ToBech32Ext, TryFromDto},
    wallet::{types::TransactionWithMetadataDto, PreparedCreateNativeTokenTransactionDto, Wallet},
};

use crate::{method::WalletMethod, response::Response};

/// Call a wallet method.
pub(crate) async fn call_wallet_method_internal(wallet: &Wallet, method: WalletMethod) -> crate::Result<Response> {
    let response = match method {
        WalletMethod::Accounts => Response::OutputsData(wallet.data().await.accounts().cloned().collect()),
        #[cfg(feature = "stronghold")]
        WalletMethod::Backup { destination, password } => {
            wallet.backup(destination, password).await?;
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::ChangeStrongholdPassword {
            current_password,
            new_password,
        } => {
            wallet
                .change_stronghold_password(current_password, new_password)
                .await?;
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::ClearStrongholdPassword => {
            wallet.clear_stronghold_password().await?;
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::IsStrongholdPasswordAvailable => {
            let is_available = wallet.is_stronghold_password_available().await?;
            Response::Bool(is_available)
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::RestoreBackup {
            source,
            password,
            ignore_if_coin_type_mismatch,
            ignore_if_bech32_mismatch,
        } => {
            wallet
                .restore_backup(
                    source,
                    password,
                    ignore_if_coin_type_mismatch,
                    ignore_if_bech32_mismatch,
                )
                .await?;
            Response::Ok
        }
        WalletMethod::SetClientOptions { client_options } => {
            wallet.set_client_options(*client_options).await?;
            Response::Ok
        }
        #[cfg(feature = "ledger_nano")]
        WalletMethod::GetLedgerNanoStatus => {
            let ledger_nano_status = wallet.get_ledger_nano_status().await?;
            Response::LedgerNanoStatus(ledger_nano_status)
        }
        WalletMethod::GenerateEd25519Address {
            account_index,
            address_index,
            options,
            bech32_hrp,
        } => {
            let address = wallet
                .generate_ed25519_address(account_index, address_index, options)
                .await?;

            let bech32_hrp = match bech32_hrp {
                Some(bech32_hrp) => bech32_hrp,
                None => *wallet.address().await.hrp(),
            };

            Response::Bech32Address(address.to_bech32(bech32_hrp))
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::SetStrongholdPassword { password } => {
            wallet.set_stronghold_password(password).await?;
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::SetStrongholdPasswordClearInterval {
            interval_in_milliseconds,
        } => {
            let duration = interval_in_milliseconds.map(Duration::from_millis);
            wallet.set_stronghold_password_clear_interval(duration).await?;
            Response::Ok
        }
        #[cfg(feature = "stronghold")]
        WalletMethod::StoreMnemonic { mnemonic } => {
            wallet.store_mnemonic(mnemonic.into()).await?;
            Response::Ok
        }
        WalletMethod::StartBackgroundSync {
            options,
            interval_in_milliseconds,
        } => {
            let duration = interval_in_milliseconds.map(Duration::from_millis);
            wallet.start_background_syncing(options, duration).await?;
            Response::Ok
        }
        WalletMethod::StopBackgroundSync => {
            wallet.stop_background_syncing().await?;
            Response::Ok
        }
        #[cfg(feature = "events")]
        WalletMethod::EmitTestEvent { event } => {
            wallet.emit_test_event(event.clone()).await;
            Response::Ok
        }
        #[cfg(feature = "events")]
        WalletMethod::ClearListeners { event_types } => {
            wallet.clear_listeners(event_types).await;
            Response::Ok
        }
        WalletMethod::UpdateNodeAuth { url, auth } => {
            wallet.update_node_auth(url, auth).await?;
            Response::Ok
        }
        WalletMethod::ClaimableOutputs { outputs_to_claim } => {
            let output_ids = wallet.claimable_outputs(outputs_to_claim).await?;
            Response::OutputIds(output_ids)
        }
        WalletMethod::ClaimOutputs { output_ids_to_claim } => {
            let transaction = wallet.claim_outputs(output_ids_to_claim.to_vec()).await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        // #[cfg(feature = "participation")]
        // WalletMethod::DeregisterParticipationEvent { event_id } => {
        //     wallet.deregister_participation_event(&event_id).await?;
        //     Response::Ok
        // }
        WalletMethod::GetAddress => {
            let address = wallet.address().await;
            Response::Address(address)
        }
        WalletMethod::GetBalance => Response::Balance(wallet.balance().await?),
        WalletMethod::GetFoundryOutput { token_id } => {
            let output = wallet.get_foundry_output(token_id).await?;
            Response::Output(output)
        }
        WalletMethod::GetIncomingTransaction { transaction_id } => wallet
            .data()
            .await
            .get_incoming_transaction(&transaction_id)
            .map_or_else(
                || Response::Transaction(None),
                |transaction| Response::Transaction(Some(Box::new(TransactionWithMetadataDto::from(transaction)))),
            ),
        WalletMethod::GetOutput { output_id } => {
            Response::OutputData(wallet.data().await.get_output(&output_id).cloned().map(Box::new))
        }
        // #[cfg(feature = "participation")]
        // WalletMethod::GetParticipationEvent { event_id } => {
        //     let event_and_nodes = wallet.get_participation_event(event_id).await?;
        //     Response::ParticipationEvent(event_and_nodes)
        // }
        // #[cfg(feature = "participation")]
        // WalletMethod::GetParticipationEventIds { node, event_type } => {
        //     let event_ids = wallet.get_participation_event_ids(&node, event_type).await?;
        //     Response::ParticipationEventIds(event_ids)
        // }
        // #[cfg(feature = "participation")]
        // WalletMethod::GetParticipationEventStatus { event_id } => {
        //     let event_status = wallet.get_participation_event_status(&event_id).await?;
        //     Response::ParticipationEventStatus(event_status)
        // }
        // #[cfg(feature = "participation")]
        // WalletMethod::GetParticipationEvents => {
        //     let events = wallet.get_participation_events().await?;
        //     Response::ParticipationEvents(events)
        // }
        // #[cfg(feature = "participation")]
        // WalletMethod::GetParticipationOverview { event_ids } => {
        //     let overview = wallet.get_participation_overview(event_ids).await?;
        //     Response::ParticipationOverview(overview)
        // }
        WalletMethod::GetTransaction { transaction_id } => Response::Transaction(
            wallet
                .data()
                .await
                .get_transaction(&transaction_id)
                .map(TransactionWithMetadataDto::from)
                .map(Box::new),
        ),
        // #[cfg(feature = "participation")]
        // WalletMethod::GetVotingPower => {
        //     let voting_power = wallet.get_voting_power().await?;
        //     Response::VotingPower(voting_power.to_string())
        // }
        WalletMethod::ImplicitAccountCreationAddress => {
            let implicit_account_creation_address = wallet.implicit_account_creation_address().await?;
            Response::Bech32Address(implicit_account_creation_address)
        }
        WalletMethod::PrepareImplicitAccountTransition {
            output_id,
            public_key,
            bip_path,
        } => {
            let data = if let Some(public_key_str) = public_key {
                let public_key = PublicKey::try_from_bytes(prefix_hex::decode(public_key_str)?)
                    .map_err(iota_sdk::wallet::Error::from)?;
                wallet
                    .prepare_implicit_account_transition(&output_id, Some(public_key))
                    .await?
            } else {
                wallet.prepare_implicit_account_transition(&output_id, bip_path).await?
            };
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::ImplicitAccounts => {
            Response::OutputsData(wallet.data().await.implicit_accounts().cloned().collect())
        }
        WalletMethod::IncomingTransactions => Response::Transactions(
            wallet
                .data()
                .await
                .incoming_transactions()
                .values()
                .map(TransactionWithMetadataDto::from)
                .collect(),
        ),
        WalletMethod::Outputs { filter_options } => {
            let wallet_data = wallet.data().await;
            Response::OutputsData(if let Some(filter) = filter_options {
                wallet_data.filtered_outputs(filter).cloned().collect()
            } else {
                wallet_data.outputs().values().cloned().collect()
            })
        }
        WalletMethod::PendingTransactions => Response::Transactions(
            wallet
                .data()
                .await
                .pending_transactions()
                .map(TransactionWithMetadataDto::from)
                .collect(),
        ),
        WalletMethod::PrepareBurn { burn, options } => {
            let data = wallet.prepare_burn(burn, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareClaimOutputs { output_ids_to_claim } => {
            let data = wallet.prepare_claim_outputs(output_ids_to_claim).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareConsolidateOutputs { params } => {
            let data = wallet.prepare_consolidate_outputs(params).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareCreateAccountOutput { params, options } => {
            let data = wallet.prepare_create_account_output(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareMeltNativeToken {
            token_id,
            melt_amount,
            options,
        } => {
            let data = wallet.prepare_melt_native_token(token_id, melt_amount, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        // #[cfg(feature = "participation")]
        // WalletMethod::PrepareDecreaseVotingPower { amount } => {
        //     let data = wallet.prepare_decrease_voting_power(amount).await?;
        //     Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        // }
        WalletMethod::PrepareMintNativeToken {
            token_id,
            mint_amount,
            options,
        } => {
            let data = wallet.prepare_mint_native_token(token_id, mint_amount, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        // #[cfg(feature = "participation")]
        // WalletMethod::PrepareIncreaseVotingPower { amount } => {
        //     let data = wallet.prepare_increase_voting_power(amount).await?;
        //     Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        // }
        WalletMethod::PrepareMintNfts { params, options } => {
            let data = wallet.prepare_mint_nfts(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareCreateNativeToken { params, options } => {
            let data = wallet.prepare_create_native_token(params, options).await?;
            Response::PreparedCreateNativeTokenTransaction(PreparedCreateNativeTokenTransactionDto::from(&data))
        }
        WalletMethod::PrepareOutput {
            params,
            transaction_options,
        } => {
            let output = wallet.prepare_output(*params, transaction_options).await?;
            Response::Output(output)
        }
        WalletMethod::PrepareSend { params, options } => {
            let data = wallet.prepare_send(params, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareSendNativeTokens { params, options } => {
            let data = wallet.prepare_send_native_tokens(params.clone(), options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        WalletMethod::PrepareSendNft { params, options } => {
            let data = wallet.prepare_send_nft(params.clone(), options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        // #[cfg(feature = "participation")]
        // WalletMethod::PrepareStopParticipating { event_id } => {
        //     let data = wallet.prepare_stop_participating(event_id).await?;
        //     Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        // }
        WalletMethod::PrepareTransaction { outputs, options } => {
            let data = wallet.prepare_transaction(outputs, options).await?;
            Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        }
        // #[cfg(feature = "participation")]
        // WalletMethod::PrepareVote { event_id, answers } => {
        //     let data = wallet.prepare_vote(event_id, answers).await?;
        //     Response::PreparedTransaction(PreparedTransactionDataDto::from(&data))
        // }
        // #[cfg(feature = "participation")]
        // WalletMethod::RegisterParticipationEvents { options } => {
        //     let events = wallet.register_participation_events(&options).await?;
        //     Response::ParticipationEvents(events)
        // }
        WalletMethod::ReissueTransactionUntilIncluded {
            transaction_id,
            interval,
            max_attempts,
        } => {
            let block_id = wallet
                .reissue_transaction_until_included(&transaction_id, interval, max_attempts)
                .await?;
            Response::BlockId(block_id)
        }
        WalletMethod::Send {
            amount,
            address,
            options,
        } => {
            let transaction = wallet.send(amount, address, options).await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        WalletMethod::SendWithParams { params, options } => {
            let transaction = wallet.send_with_params(params, options).await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        WalletMethod::SendOutputs { outputs, options } => {
            let transaction = wallet.send_outputs(outputs, options).await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        WalletMethod::SetAlias { alias } => {
            wallet.set_alias(&alias).await?;
            Response::Ok
        }
        WalletMethod::SetDefaultSyncOptions { options } => {
            wallet.set_default_sync_options(options).await?;
            Response::Ok
        }
        WalletMethod::SignAndSubmitTransaction {
            prepared_transaction_data,
        } => {
            let transaction = wallet
                .sign_and_submit_transaction(
                    PreparedTransactionData::try_from_dto_with_params(
                        prepared_transaction_data,
                        &wallet.client().get_protocol_parameters().await?,
                    )?,
                    None,
                    None,
                )
                .await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        WalletMethod::SignTransaction {
            prepared_transaction_data,
        } => {
            let signed_transaction_data = wallet
                .sign_transaction(&PreparedTransactionData::try_from_dto(prepared_transaction_data)?)
                .await?;
            Response::SignedTransactionData(SignedTransactionDataDto::from(&signed_transaction_data))
        }
        WalletMethod::SubmitAndStoreTransaction {
            signed_transaction_data,
        } => {
            let signed_transaction_data = SignedTransactionData::try_from_dto_with_params(
                signed_transaction_data,
                &wallet.client().get_protocol_parameters().await?,
            )?;
            let transaction = wallet
                .submit_and_store_transaction(signed_transaction_data, None, None)
                .await?;
            Response::SentTransaction(TransactionWithMetadataDto::from(&transaction))
        }
        WalletMethod::Sync { options } => Response::Balance(wallet.sync(options).await?),
        WalletMethod::Transactions => Response::Transactions(
            wallet
                .data()
                .await
                .transactions()
                .values()
                .map(TransactionWithMetadataDto::from)
                .collect(),
        ),
        WalletMethod::UnspentOutputs { filter_options } => {
            let wallet_data = wallet.data().await;
            Response::OutputsData(if let Some(filter) = filter_options {
                wallet_data.filtered_unspent_outputs(filter).cloned().collect()
            } else {
                wallet_data.unspent_outputs().values().cloned().collect()
            })
        }
    };
    Ok(response)
}
