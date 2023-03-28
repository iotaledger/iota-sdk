// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "participation")]
use std::str::FromStr;

use primitive_types::U256;

use super::Result;
use crate::{
    client::{
        api::{PreparedTransactionData, PreparedTransactionDataDto, SignedTransactionData, SignedTransactionDataDto},
        request_funds_from_faucet,
    },
    message_interface::{message::AccountMethod, panic::convert_async_panics, response::Response},
    types::block::{
        output::{
            dto::{OutputBuilderAmountDto, OutputDto},
            AliasId, AliasOutput, BasicOutput, FoundryOutput, NftId, NftOutput, Output, Rent, TokenId,
        },
        DtoError,
    },
    wallet::{
        account::{
            operations::transaction::{
                high_level::{create_alias::AliasOutputOptions, minting::mint_native_token::MintTokenTransactionDto},
                prepare_output::OutputOptions,
                TransactionOptions,
            },
            types::{AccountBalanceDto, TransactionDto},
            AccountHandle, OutputDataDto,
        },
        message_interface::AddressWithUnspentOutputsDto,
        AddressWithAmount, AddressWithMicroAmount, IncreaseNativeTokenSupplyOptions, NativeTokenOptions, NftOptions,
    },
};

impl AccountHandle {
    pub(crate) async fn handle_message(&self, method: AccountMethod) -> Result<Response> {
        match method {
            AccountMethod::BuildAliasOutput {
                amount,
                native_tokens,
                alias_id,
                state_index,
                state_metadata,
                foundry_counter,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output = Output::from(AliasOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.client.get_rent_structure().await?)
                    },
                    native_tokens,
                    &alias_id,
                    state_index,
                    state_metadata,
                    foundry_counter,
                    unlock_conditions,
                    features,
                    immutable_features,
                    self.client.get_token_supply().await?,
                )?);

                Ok(Response::Output(OutputDto::from(&output)))
            }
            AccountMethod::BuildBasicOutput {
                amount,
                native_tokens,
                unlock_conditions,
                features,
            } => {
                let output = Output::from(BasicOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.client.get_rent_structure().await?)
                    },
                    native_tokens,
                    unlock_conditions,
                    features,
                    self.client.get_token_supply().await?,
                )?);

                Ok(Response::Output(OutputDto::from(&output)))
            }
            AccountMethod::BuildFoundryOutput {
                amount,
                native_tokens,
                serial_number,
                token_scheme,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output = Output::from(FoundryOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.client.get_rent_structure().await?)
                    },
                    native_tokens,
                    serial_number,
                    &token_scheme,
                    unlock_conditions,
                    features,
                    immutable_features,
                    self.client.get_token_supply().await?,
                )?);

                Ok(Response::Output(OutputDto::from(&output)))
            }
            AccountMethod::BuildNftOutput {
                amount,
                native_tokens,
                nft_id,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output = Output::from(NftOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.client.get_rent_structure().await?)
                    },
                    native_tokens,
                    &nft_id,
                    unlock_conditions,
                    features,
                    immutable_features,
                    self.client.get_token_supply().await?,
                )?);

                Ok(Response::Output(OutputDto::from(&output)))
            }
            AccountMethod::BurnNativeToken {
                token_id,
                burn_amount,
                options,
            } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .burn_native_token(
                            TokenId::try_from(&token_id)?,
                            U256::try_from(&burn_amount).map_err(|_| DtoError::InvalidField("burn_amount"))?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::BurnNft { nft_id, options } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .burn_nft(
                            NftId::try_from(&nft_id)?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::ConsolidateOutputs {
                force,
                output_consolidation_threshold,
            } => {
                convert_async_panics(|| async {
                    let transaction = self.consolidate_outputs(force, output_consolidation_threshold).await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::CreateAliasOutput {
                alias_output_options,
                options,
            } => {
                convert_async_panics(|| async {
                    let alias_output_options = alias_output_options
                        .map(|options| AliasOutputOptions::try_from(&options))
                        .transpose()?;

                    let transaction = self
                        .create_alias_output(
                            alias_output_options,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::DestroyAlias { alias_id, options } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .destroy_alias(
                            AliasId::try_from(&alias_id)?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::DestroyFoundry { foundry_id, options } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .destroy_foundry(
                            foundry_id,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::GenerateAddresses { amount, options } => {
                let address = self.generate_addresses(amount, options).await?;
                Ok(Response::GeneratedAddress(address))
            }
            AccountMethod::GetOutputsWithAdditionalUnlockConditions { outputs_to_claim } => {
                let output_ids = self
                    .get_unlockable_outputs_with_additional_unlock_conditions(outputs_to_claim)
                    .await?;
                Ok(Response::OutputIds(output_ids))
            }
            AccountMethod::GetOutput { output_id } => {
                let output_data = self.get_output(&output_id).await;
                Ok(Response::OutputData(
                    output_data.as_ref().map(OutputDataDto::from).map(Box::new),
                ))
            }
            AccountMethod::GetFoundryOutput { token_id } => {
                let token_id = TokenId::try_from(&token_id)?;
                let output = self.get_foundry_output(token_id).await?;
                Ok(Response::Output(OutputDto::from(&output)))
            }
            AccountMethod::GetTransaction { transaction_id } => {
                let transaction = self.get_transaction(&transaction_id).await;
                Ok(Response::Transaction(
                    transaction.as_ref().map(TransactionDto::from).map(Box::new),
                ))
            }
            AccountMethod::GetIncomingTransactionData { transaction_id } => {
                let transaction = self.get_incoming_transaction_data(&transaction_id).await;

                transaction.map_or_else(
                    || Ok(Response::IncomingTransactionData(None)),
                    |transaction| {
                        Ok(Response::IncomingTransactionData(Some(Box::new((
                            transaction_id,
                            TransactionDto::from(&transaction),
                        )))))
                    },
                )
            }
            AccountMethod::Addresses => {
                let addresses = self.addresses().await?;
                Ok(Response::Addresses(addresses))
            }
            AccountMethod::AddressesWithUnspentOutputs => {
                let addresses = self.addresses_with_unspent_outputs().await?;
                Ok(Response::AddressesWithUnspentOutputs(
                    addresses.iter().map(AddressWithUnspentOutputsDto::from).collect(),
                ))
            }
            AccountMethod::Outputs { filter_options } => {
                let outputs = self.outputs(filter_options).await?;
                Ok(Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect()))
            }
            AccountMethod::UnspentOutputs { filter_options } => {
                let outputs = self.unspent_outputs(filter_options).await?;
                Ok(Response::OutputsData(outputs.iter().map(OutputDataDto::from).collect()))
            }
            AccountMethod::IncomingTransactions => {
                let transactions = self.incoming_transactions().await?;
                Ok(Response::IncomingTransactionsData(
                    transactions
                        .into_iter()
                        .map(|d| (d.0, TransactionDto::from(&d.1)))
                        .collect(),
                ))
            }
            AccountMethod::Transactions => {
                let transactions = self.transactions().await?;
                Ok(Response::Transactions(
                    transactions.iter().map(TransactionDto::from).collect(),
                ))
            }
            AccountMethod::PendingTransactions => {
                let transactions = self.pending_transactions().await?;
                Ok(Response::Transactions(
                    transactions.iter().map(TransactionDto::from).collect(),
                ))
            }
            AccountMethod::DecreaseNativeTokenSupply {
                token_id,
                melt_amount,
                options,
            } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .decrease_native_token_supply(
                            TokenId::try_from(&token_id)?,
                            U256::try_from(&melt_amount).map_err(|_| DtoError::InvalidField("melt_amount"))?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::IncreaseNativeTokenSupply {
                token_id,
                mint_amount,
                increase_native_token_supply_options,
                options,
            } => {
                convert_async_panics(|| async {
                    let increase_native_token_supply_options = match increase_native_token_supply_options {
                        Some(native_token_options) => {
                            Some(IncreaseNativeTokenSupplyOptions::try_from(&native_token_options)?)
                        }
                        None => None,
                    };
                    let transaction = self
                        .increase_native_token_supply(
                            TokenId::try_from(&token_id)?,
                            U256::try_from(&mint_amount).map_err(|_| DtoError::InvalidField("mint_amount"))?,
                            increase_native_token_supply_options,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::MintTokenTransaction(MintTokenTransactionDto::from(
                        &transaction,
                    )))
                })
                .await
            }
            AccountMethod::MintNativeToken {
                native_token_options,
                options,
            } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .mint_native_token(
                            NativeTokenOptions::try_from(&native_token_options)?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::MintTokenTransaction(MintTokenTransactionDto::from(
                        &transaction,
                    )))
                })
                .await
            }
            AccountMethod::MinimumRequiredStorageDeposit { output } => {
                convert_async_panics(|| async {
                    let output = Output::try_from_dto(&output, self.client.get_token_supply().await?)?;
                    let rent_structure = self.client.get_rent_structure().await?;

                    let minimum_storage_deposit = output.rent_cost(&rent_structure);

                    Ok(Response::MinimumRequiredStorageDeposit(
                        minimum_storage_deposit.to_string(),
                    ))
                })
                .await
            }
            AccountMethod::MintNfts { nfts_options, options } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .mint_nfts(
                            nfts_options
                                .iter()
                                .map(NftOptions::try_from)
                                .collect::<crate::wallet::Result<Vec<NftOptions>>>()?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::GetBalance => {
                convert_async_panics(|| async {
                    Ok(Response::Balance(AccountBalanceDto::from(&self.balance().await?)))
                })
                .await
            }
            AccountMethod::PrepareOutput {
                options,
                transaction_options,
            } => {
                convert_async_panics(|| async {
                    let output = self
                        .prepare_output(
                            OutputOptions::try_from(&options)?,
                            transaction_options
                                .as_ref()
                                .map(TransactionOptions::try_from_dto)
                                .transpose()?,
                        )
                        .await?;
                    Ok(Response::Output(OutputDto::from(&output)))
                })
                .await
            }
            AccountMethod::PrepareSendAmount {
                addresses_with_amount,
                options,
            } => {
                convert_async_panics(|| async {
                    let data = self
                        .prepare_send_amount(
                            addresses_with_amount
                                .iter()
                                .map(AddressWithAmount::try_from)
                                .collect::<crate::wallet::Result<Vec<AddressWithAmount>>>()?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::PreparedTransaction(PreparedTransactionDataDto::from(&data)))
                })
                .await
            }
            AccountMethod::PrepareTransaction { outputs, options } => {
                convert_async_panics(|| async {
                    let token_supply = self.client.get_token_supply().await?;
                    let data = self
                        .prepare_transaction(
                            outputs
                                .iter()
                                .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                                .collect::<Result<Vec<Output>>>()?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::PreparedTransaction(PreparedTransactionDataDto::from(&data)))
                })
                .await
            }
            AccountMethod::RetryTransactionUntilIncluded {
                transaction_id,
                interval,
                max_attempts,
            } => {
                convert_async_panics(|| async {
                    let block_id = self
                        .retry_transaction_until_included(&transaction_id, interval, max_attempts)
                        .await?;
                    Ok(Response::BlockId(block_id))
                })
                .await
            }
            AccountMethod::SyncAccount { options } => {
                convert_async_panics(|| async {
                    Ok(Response::Balance(AccountBalanceDto::from(&self.sync(options).await?)))
                })
                .await
            }
            AccountMethod::SendAmount {
                addresses_with_amount,
                options,
            } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .send_amount(
                            addresses_with_amount
                                .iter()
                                .map(AddressWithAmount::try_from)
                                .collect::<crate::wallet::Result<Vec<AddressWithAmount>>>()?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::SendMicroTransaction {
                addresses_with_micro_amount,
                options,
            } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .send_micro_transaction(
                            addresses_with_micro_amount
                                .iter()
                                .map(AddressWithMicroAmount::try_from)
                                .collect::<crate::wallet::Result<Vec<AddressWithMicroAmount>>>()?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::SendNativeTokens {
                addresses_native_tokens,
                options,
            } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .send_native_tokens(
                            addresses_native_tokens.clone(),
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::SendNft {
                addresses_nft_ids,
                options,
            } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .send_nft(
                            addresses_nft_ids.clone(),
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::SetAlias { alias } => {
                convert_async_panics(|| async {
                    self.set_alias(&alias).await?;
                    Ok(Response::Ok)
                })
                .await
            }
            AccountMethod::SendOutputs { outputs, options } => {
                convert_async_panics(|| async {
                    let token_supply = self.client.get_token_supply().await?;
                    let transaction = self
                        .send(
                            outputs
                                .iter()
                                .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                                .collect::<crate::wallet::Result<Vec<Output>>>()?,
                            options.as_ref().map(TransactionOptions::try_from_dto).transpose()?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::SignTransactionEssence {
                prepared_transaction_data,
            } => {
                convert_async_panics(|| async {
                    let signed_transaction_data = self
                        .sign_transaction_essence(&PreparedTransactionData::try_from_dto(
                            &prepared_transaction_data,
                            &self.client.get_protocol_parameters().await?,
                        )?)
                        .await?;
                    Ok(Response::SignedTransactionData(SignedTransactionDataDto::from(
                        &signed_transaction_data,
                    )))
                })
                .await
            }
            AccountMethod::SubmitAndStoreTransaction {
                signed_transaction_data,
            } => {
                convert_async_panics(|| async {
                    let signed_transaction_data = SignedTransactionData::try_from_dto(
                        &signed_transaction_data,
                        &self.client.get_protocol_parameters().await?,
                    )?;
                    let transaction = self.submit_and_store_transaction(signed_transaction_data).await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            AccountMethod::ClaimOutputs { output_ids_to_claim } => {
                convert_async_panics(|| async {
                    let transaction = self.claim_outputs(output_ids_to_claim.to_vec()).await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::Vote { event_id, answers } => {
                convert_async_panics(|| async {
                    let transaction = self.vote(event_id, answers).await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::StopParticipating { event_id } => {
                convert_async_panics(|| async {
                    let transaction = self.stop_participating(event_id).await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetVotingPower => {
                convert_async_panics(|| async {
                    let voting_power = self.get_voting_power().await?;
                    Ok(Response::VotingPower(voting_power.to_string()))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetParticipationOverview { event_ids } => {
                convert_async_panics(|| async {
                    let overview = self.get_participation_overview(event_ids).await?;
                    Ok(Response::AccountParticipationOverview(overview))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::IncreaseVotingPower { amount } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .increase_voting_power(
                            u64::from_str(&amount).map_err(|_| crate::client::Error::InvalidAmount(amount.clone()))?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::DecreaseVotingPower { amount } => {
                convert_async_panics(|| async {
                    let transaction = self
                        .decrease_voting_power(
                            u64::from_str(&amount).map_err(|_| crate::client::Error::InvalidAmount(amount.clone()))?,
                        )
                        .await?;
                    Ok(Response::SentTransaction(TransactionDto::from(&transaction)))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::RegisterParticipationEvents { options } => {
                convert_async_panics(|| async {
                    let events = self.register_participation_events(&options).await?;
                    Ok(Response::ParticipationEvents(events))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::DeregisterParticipationEvent { event_id } => {
                convert_async_panics(|| async {
                    self.deregister_participation_event(&event_id).await?;
                    Ok(Response::Ok)
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetParticipationEvent { event_id } => {
                convert_async_panics(|| async {
                    let event_and_nodes = self.get_participation_event(event_id).await?;
                    Ok(Response::ParticipationEvent(event_and_nodes))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetParticipationEventIds { node, event_type } => {
                convert_async_panics(|| async {
                    let event_ids = self.get_participation_event_ids(&node, event_type).await?;
                    Ok(Response::ParticipationEventIds(event_ids))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetParticipationEventStatus { event_id } => {
                convert_async_panics(|| async {
                    let event_status = self.get_participation_event_status(&event_id).await?;
                    Ok(Response::ParticipationEventStatus(event_status))
                })
                .await
            }
            #[cfg(feature = "participation")]
            AccountMethod::GetParticipationEvents => {
                convert_async_panics(|| async {
                    let events = self.get_participation_events().await?;
                    Ok(Response::ParticipationEvents(events))
                })
                .await
            }
            AccountMethod::RequestFundsFromFaucet { url, address } => {
                convert_async_panics(|| async {
                    Ok(Response::Faucet(request_funds_from_faucet(&url, &address).await?))
                })
                .await
            }
        }
    }
}
