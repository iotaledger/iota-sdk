// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use zeroize::Zeroize;

#[cfg(feature = "ledger_nano")]
use crate::client::secret::ledger_nano::LedgerSecretManager;
use crate::{
    client::{
        api::{PreparedTransactionData, PreparedTransactionDataDto},
        constants::SHIMMER_TESTNET_BECH32_HRP,
        request_funds_from_faucet,
        secret::{SecretManage, SecretManager},
        utils, Client, NodeInfoWrapper,
    },
    message_interface::{
        message::{ClientMessage, Message, WalletMessage},
        message_handler::Result,
        panic::{convert_async_panics, convert_panics},
        response::Response,
        MessageHandler,
    },
    types::block::{
        address::{dto::AddressDto, Address},
        input::dto::UtxoInputDto,
        output::{
            dto::{OutputBuilderAmountDto, OutputDto, RentStructureDto},
            AliasId, AliasOutput, BasicOutput, FoundryId, FoundryOutput, NftId, NftOutput, Output,
        },
        payload::{
            dto::{MilestonePayloadDto, PayloadDto},
            transaction::TransactionEssence,
            Payload, TransactionPayload,
        },
        protocol::dto::ProtocolParametersDto,
        unlock::Unlock,
        Block, BlockDto, DtoError,
    },
    wallet::message_interface::dtos::AccountDto,
};

impl MessageHandler {
    /// Send a message.
    pub async fn send_message(&self, message: Message) -> Response {
        match &message {
            Message::Client(message) => {
                match message {
                    // Don't log secrets
                    ClientMessage::GenerateAddresses {
                        secret_manager: _,
                        options,
                    } => {
                        log::debug!("Response: GenerateAddresses{{ secret_manager: <omitted>, options: {options:?} }}")
                    }
                    ClientMessage::BuildAndPostBlock {
                        secret_manager: _,
                        options,
                    } => {
                        log::debug!("Response: BuildAndPostBlock{{ secret_manager: <omitted>, options: {options:?} }}")
                    }
                    ClientMessage::PrepareTransaction {
                        secret_manager: _,
                        options,
                    } => {
                        log::debug!("Response: PrepareTransaction{{ secret_manager: <omitted>, options: {options:?} }}")
                    }
                    ClientMessage::SignTransaction {
                        secret_manager: _,
                        prepared_transaction_data,
                    } => {
                        log::debug!(
                            "Response: SignTransaction{{ secret_manager: <omitted>, prepared_transaction_data: {prepared_transaction_data:?} }}"
                        )
                    }
                    #[cfg(feature = "stronghold")]
                    ClientMessage::StoreMnemonic { .. } => {
                        log::debug!("Response: StoreMnemonic{{ <omitted> }}")
                    }
                    ClientMessage::ConsolidateFunds {
                        secret_manager: _,
                        generate_addresses_options,
                    } => {
                        log::debug!(
                            "Response: ConsolidateFunds{{ secret_manager: <omitted>, generate_addresses_options: {generate_addresses_options:?} }}"
                        )
                    }
                    ClientMessage::MnemonicToHexSeed { .. } => {
                        log::debug!("Response: MnemonicToHexSeed{{ <omitted> }}")
                    }
                    _ => log::debug!("Message: {:?}", message),
                }
            }
            _ => log::debug!("Message: {:?}", message),
        }

        let result = convert_async_panics(|| async { self.handle_message(message).await }).await;

        let response = match result {
            Ok(r) => r,
            Err(e) => Response::Error(e),
        };

        match response {
            // Don't log secrets
            Response::GeneratedMnemonic { .. } => {
                log::debug!("Response: GeneratedMnemonic(<omitted>)")
            }
            Response::MnemonicHexSeed { .. } => {
                log::debug!("Response: MnemonicHexSeed(<omitted>)")
            }
            _ => log::debug!("Response: {:?}", response),
        }

        response
    }

    /// Handle a message.
    pub(crate) async fn handle_message(&self, message: Message) -> Result<Response> {
        match message {
            Message::Client(message) => {
                match message {
                    ClientMessage::BuildAliasOutput {
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
                            state_metadata.map(prefix_hex::decode).transpose()?,
                            foundry_counter,
                            unlock_conditions,
                            features,
                            immutable_features,
                            self.client.get_token_supply().await?,
                        )?);

                        Ok(Response::BuiltOutput(OutputDto::from(&output)))
                    }
                    ClientMessage::BuildBasicOutput {
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

                        Ok(Response::BuiltOutput(OutputDto::from(&output)))
                    }
                    ClientMessage::BuildFoundryOutput {
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

                        Ok(Response::BuiltOutput(OutputDto::from(&output)))
                    }
                    ClientMessage::BuildNftOutput {
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

                        Ok(Response::BuiltOutput(OutputDto::from(&output)))
                    }
                    ClientMessage::GenerateAddresses {
                        secret_manager,
                        options,
                    } => {
                        let secret_manager = (&secret_manager).try_into()?;
                        let addresses = self
                            .client
                            .get_addresses(&secret_manager)
                            .set_options(options)?
                            .finish()
                            .await?;
                        Ok(Response::GeneratedAddresses(addresses))
                    }
                    ClientMessage::BuildAndPostBlock {
                        secret_manager,
                        options,
                    } => {
                        // Prepare transaction
                        let mut block_builder = self.client.block();

                        let secret_manager = match secret_manager {
                            Some(secret_manager) => Some((&secret_manager).try_into()?),
                            None => None,
                        };

                        if let Some(secret_manager) = &secret_manager {
                            block_builder = block_builder.with_secret_manager(secret_manager);
                        }

                        if let Some(options) = options {
                            block_builder = block_builder.set_options(options).await?;
                        }

                        let block = block_builder.finish().await?;
                        let block_id = block.id();

                        Ok(Response::BlockIdWithBlock(block_id, BlockDto::from(&block)))
                    }
                    #[cfg(feature = "mqtt")]
                    ClientMessage::ClearListeners { topics } => {
                        self.client.unsubscribe(topics).await?;
                        Ok(Response::Ok)
                    }
                    ClientMessage::GetNode => Ok(Response::Node(self.client.get_node()?)),
                    ClientMessage::GetNetworkInfo => {
                        Ok(Response::NetworkInfo(self.client.get_network_info().await?.into()))
                    }
                    ClientMessage::GetNetworkId => Ok(Response::NetworkId(self.client.get_network_id().await?)),
                    ClientMessage::GetBech32Hrp => Ok(Response::Bech32Hrp(self.client.get_bech32_hrp().await?)),
                    ClientMessage::GetMinPowScore => Ok(Response::MinPowScore(self.client.get_min_pow_score().await?)),
                    ClientMessage::GetTipsInterval => Ok(Response::TipsInterval(self.client.get_tips_interval())),
                    ClientMessage::GetProtocolParameters => {
                        let params = self.client.get_protocol_parameters().await?;
                        let protocol_response = ProtocolParametersDto {
                            protocol_version: params.protocol_version(),
                            network_name: params.network_name().to_string(),
                            bech32_hrp: params.bech32_hrp().to_string(),
                            min_pow_score: params.min_pow_score(),
                            below_max_depth: params.below_max_depth(),
                            rent_structure: RentStructureDto {
                                v_byte_cost: params.rent_structure().byte_cost(),
                                v_byte_factor_key: params.rent_structure().byte_factor_key(),
                                v_byte_factor_data: params.rent_structure().byte_factor_data(),
                            },
                            token_supply: params.token_supply().to_string(),
                        };
                        Ok(Response::ProtocolParameters(protocol_response))
                    }
                    ClientMessage::GetLocalPow => Ok(Response::LocalPow(self.client.get_local_pow())),
                    ClientMessage::GetFallbackToLocalPow => {
                        Ok(Response::FallbackToLocalPow(self.client.get_fallback_to_local_pow()))
                    }
                    #[cfg(feature = "ledger_nano")]
                    ClientMessage::GetLedgerNanoStatus { is_simulator } => {
                        let ledger_nano = LedgerSecretManager::new(is_simulator);

                        Ok(Response::LedgerNanoStatus(ledger_nano.get_ledger_nano_status().await))
                    }
                    ClientMessage::PrepareTransaction {
                        secret_manager,
                        options,
                    } => {
                        let mut block_builder = self.client.block();

                        let secret_manager = match secret_manager {
                            Some(secret_manager) => Some((&secret_manager).try_into()?),
                            None => None,
                        };

                        if let Some(secret_manager) = &secret_manager {
                            block_builder = block_builder.with_secret_manager(secret_manager);
                        }

                        if let Some(options) = options {
                            block_builder = block_builder.set_options(options).await?;
                        }

                        Ok(Response::PreparedTransactionData(PreparedTransactionDataDto::from(
                            &block_builder.prepare_transaction().await?,
                        )))
                    }
                    ClientMessage::SignTransaction {
                        secret_manager,
                        prepared_transaction_data,
                    } => {
                        let mut block_builder = self.client.block();

                        let secret_manager = (&secret_manager).try_into()?;

                        block_builder = block_builder.with_secret_manager(&secret_manager);

                        Ok(Response::SignedTransaction(PayloadDto::from(
                            &block_builder
                                .sign_transaction(PreparedTransactionData::try_from_dto_unverified(
                                    &prepared_transaction_data,
                                )?)
                                .await?,
                        )))
                    }
                    ClientMessage::SignatureUnlock {
                        secret_manager,
                        transaction_essence_hash,
                        chain,
                    } => {
                        let secret_manager: SecretManager = (&secret_manager).try_into()?;
                        let transaction_essence_hash: [u8; 32] = transaction_essence_hash
                            .try_into()
                            .map_err(|_| DtoError::InvalidField("expected 32 bytes for transactionEssenceHash"))?;

                        let unlock: Unlock = secret_manager
                            .signature_unlock(&transaction_essence_hash, &chain)
                            .await?;

                        Ok(Response::SignatureUnlock((&unlock).into()))
                    }
                    #[cfg(feature = "stronghold")]
                    ClientMessage::StoreMnemonic {
                        secret_manager,
                        mnemonic,
                    } => {
                        let mut secret_manager = (&secret_manager).try_into()?;
                        if let SecretManager::Stronghold(secret_manager) = &mut secret_manager {
                            secret_manager.store_mnemonic(mnemonic).await?;
                        } else {
                            return Err(crate::client::Error::SecretManagerMismatch.into());
                        }

                        Ok(Response::Ok)
                    }
                    ClientMessage::PostBlockPayload { payload_dto } => {
                        let block_builder = self.client.block();

                        let block = block_builder
                            .finish_block(Some(Payload::try_from_dto(
                                &payload_dto,
                                &self.client.get_protocol_parameters().await?,
                            )?))
                            .await?;

                        let block_id = block.id();

                        Ok(Response::BlockIdWithBlock(block_id, BlockDto::from(&block)))
                    }
                    #[cfg(not(target_family = "wasm"))]
                    ClientMessage::UnhealthyNodes => Ok(Response::UnhealthyNodes(
                        self.client.unhealthy_nodes().into_iter().cloned().collect(),
                    )),
                    ClientMessage::GetHealth { url } => Ok(Response::Health(self.client.get_health(&url).await?)),
                    ClientMessage::GetNodeInfo { url, auth } => {
                        Ok(Response::NodeInfo(Client::get_node_info(&url, auth).await?))
                    }
                    ClientMessage::GetInfo => Ok(Response::Info(self.client.get_info().await?)),
                    ClientMessage::GetPeers => Ok(Response::Peers(self.client.get_peers().await?)),
                    ClientMessage::GetTips => Ok(Response::Tips(self.client.get_tips().await?)),
                    ClientMessage::PostBlockRaw { block_bytes } => Ok(Response::BlockId(
                        self.client
                            .post_block_raw(&Block::unpack_strict(
                                &block_bytes[..],
                                &self.client.get_protocol_parameters().await?,
                            )?)
                            .await?,
                    )),
                    ClientMessage::PostBlock { block } => Ok(Response::BlockId(
                        self.client
                            .post_block(&Block::try_from_dto(
                                &block,
                                &self.client.get_protocol_parameters().await?,
                            )?)
                            .await?,
                    )),
                    ClientMessage::GetBlock { block_id } => Ok(Response::Block(BlockDto::from(
                        &self.client.get_block(&block_id).await?,
                    ))),
                    ClientMessage::GetBlockMetadata { block_id } => Ok(Response::BlockMetadata(
                        self.client.get_block_metadata(&block_id).await?,
                    )),
                    ClientMessage::GetBlockRaw { block_id } => {
                        Ok(Response::BlockRaw(self.client.get_block_raw(&block_id).await?))
                    }
                    ClientMessage::GetOutput { output_id } => Ok(Response::OutputWithMetadataResponse(
                        self.client.get_output(&output_id).await?,
                    )),
                    ClientMessage::GetOutputMetadata { output_id } => Ok(Response::OutputMetadata(
                        self.client.get_output_metadata(&output_id).await?,
                    )),
                    ClientMessage::GetMilestoneById { milestone_id } => Ok(Response::Milestone(
                        MilestonePayloadDto::from(&self.client.get_milestone_by_id(&milestone_id).await?),
                    )),
                    ClientMessage::GetMilestoneByIdRaw { milestone_id } => Ok(Response::MilestoneRaw(
                        self.client.get_milestone_by_id_raw(&milestone_id).await?,
                    )),
                    ClientMessage::GetMilestoneByIndex { index } => Ok(Response::Milestone(MilestonePayloadDto::from(
                        &self.client.get_milestone_by_index(index).await?,
                    ))),
                    ClientMessage::GetMilestoneByIndexRaw { index } => Ok(Response::MilestoneRaw(
                        self.client.get_milestone_by_index_raw(index).await?,
                    )),
                    ClientMessage::GetUtxoChangesById { milestone_id } => Ok(Response::MilestoneUtxoChanges(
                        self.client.get_utxo_changes_by_id(&milestone_id).await?,
                    )),
                    ClientMessage::GetUtxoChangesByIndex { index } => Ok(Response::MilestoneUtxoChanges(
                        self.client.get_utxo_changes_by_index(index).await?,
                    )),
                    ClientMessage::GetReceipts => Ok(Response::Receipts(self.client.get_receipts().await?)),
                    ClientMessage::GetReceiptsMigratedAt { milestone_index } => Ok(Response::Receipts(
                        self.client.get_receipts_migrated_at(milestone_index).await?,
                    )),
                    ClientMessage::GetTreasury => Ok(Response::Treasury(self.client.get_treasury().await?)),
                    ClientMessage::GetIncludedBlock { transaction_id } => Ok(Response::Block(BlockDto::from(
                        &self.client.get_included_block(&transaction_id).await?,
                    ))),
                    ClientMessage::GetIncludedBlockMetadata { transaction_id } => Ok(Response::BlockMetadata(
                        self.client.get_included_block_metadata(&transaction_id).await?,
                    )),
                    ClientMessage::BasicOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                        self.client.basic_output_ids(query_parameters).await?,
                    )),
                    ClientMessage::AliasOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                        self.client.alias_output_ids(query_parameters).await?,
                    )),
                    ClientMessage::AliasOutputId { alias_id } => {
                        Ok(Response::OutputId(self.client.alias_output_id(alias_id).await?))
                    }
                    ClientMessage::NftOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                        self.client.nft_output_ids(query_parameters).await?,
                    )),
                    ClientMessage::NftOutputId { nft_id } => {
                        Ok(Response::OutputId(self.client.nft_output_id(nft_id).await?))
                    }
                    ClientMessage::FoundryOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                        self.client.foundry_output_ids(query_parameters).await?,
                    )),
                    ClientMessage::FoundryOutputId { foundry_id } => {
                        Ok(Response::OutputId(self.client.foundry_output_id(foundry_id).await?))
                    }
                    ClientMessage::GetOutputs { output_ids } => {
                        Ok(Response::Outputs(self.client.get_outputs(output_ids).await?))
                    }
                    ClientMessage::TryGetOutputs { output_ids } => {
                        Ok(Response::Outputs(self.client.try_get_outputs(output_ids).await?))
                    }
                    ClientMessage::FindBlocks { block_ids } => Ok(Response::Blocks(
                        self.client
                            .find_blocks(&block_ids)
                            .await?
                            .iter()
                            .map(BlockDto::from)
                            .collect(),
                    )),
                    ClientMessage::Retry { block_id } => {
                        let (block_id, block) = self.client.retry(&block_id).await?;
                        Ok(Response::BlockIdWithBlock(block_id, BlockDto::from(&block)))
                    }
                    ClientMessage::RetryUntilIncluded {
                        block_id,
                        interval,
                        max_attempts,
                    } => {
                        let res = self
                            .client
                            .retry_until_included(&block_id, interval, max_attempts)
                            .await?;
                        let res = res
                            .into_iter()
                            .map(|(block_id, block)| (block_id, BlockDto::from(&block)))
                            .collect();
                        Ok(Response::RetryUntilIncludedSuccessful(res))
                    }
                    ClientMessage::ConsolidateFunds {
                        secret_manager,
                        generate_addresses_options,
                    } => {
                        let secret_manager = (&secret_manager).try_into()?;
                        Ok(Response::ConsolidatedFunds(
                            self.client
                                .consolidate_funds(&secret_manager, generate_addresses_options)
                                .await?,
                        ))
                    }
                    ClientMessage::FindInputs { addresses, amount } => Ok(Response::Inputs(
                        self.client
                            .find_inputs(addresses, amount)
                            .await?
                            .iter()
                            .map(UtxoInputDto::from)
                            .collect(),
                    )),
                    ClientMessage::FindOutputs { output_ids, addresses } => Ok(Response::Outputs(
                        self.client.find_outputs(&output_ids, &addresses).await?,
                    )),
                    ClientMessage::Reattach { block_id } => {
                        let (block_id, block) = self.client.reattach(&block_id).await?;
                        Ok(Response::Reattached((block_id, BlockDto::from(&block))))
                    }
                    ClientMessage::ReattachUnchecked { block_id } => {
                        let (block_id, block) = self.client.reattach_unchecked(&block_id).await?;
                        Ok(Response::Reattached((block_id, BlockDto::from(&block))))
                    }
                    ClientMessage::Promote { block_id } => {
                        let (block_id, block) = self.client.promote(&block_id).await?;
                        Ok(Response::Promoted((block_id, BlockDto::from(&block))))
                    }
                    ClientMessage::PromoteUnchecked { block_id } => {
                        let (block_id, block) = self.client.promote_unchecked(&block_id).await?;
                        Ok(Response::Promoted((block_id, BlockDto::from(&block))))
                    }
                    ClientMessage::Bech32ToHex { bech32 } => Ok(Response::Bech32ToHex(Client::bech32_to_hex(&bech32)?)),
                    ClientMessage::HexToBech32 { hex, bech32_hrp } => Ok(Response::Bech32Address(
                        self.client.hex_to_bech32(&hex, bech32_hrp.as_deref()).await?,
                    )),
                    ClientMessage::AliasIdToBech32 { alias_id, bech32_hrp } => Ok(Response::Bech32Address(
                        self.client.alias_id_to_bech32(alias_id, bech32_hrp.as_deref()).await?,
                    )),
                    ClientMessage::NftIdToBech32 { nft_id, bech32_hrp } => Ok(Response::Bech32Address(
                        self.client.nft_id_to_bech32(nft_id, bech32_hrp.as_deref()).await?,
                    )),
                    ClientMessage::HexPublicKeyToBech32Address { hex, bech32_hrp } => Ok(Response::Bech32Address(
                        self.client
                            .hex_public_key_to_bech32_address(&hex, bech32_hrp.as_deref())
                            .await?,
                    )),
                    ClientMessage::ParseBech32Address { address } => Ok(Response::ParsedBech32Address(
                        AddressDto::from(&Address::try_from_bech32(address)?),
                    )),
                    ClientMessage::IsAddressValid { address } => {
                        Ok(Response::IsAddressValid(Address::is_valid_bech32(&address)))
                    }
                    ClientMessage::GenerateMnemonic => Ok(Response::GeneratedMnemonic(Client::generate_mnemonic()?)),
                    ClientMessage::MnemonicToHexSeed { mut mnemonic } => {
                        let response = Response::MnemonicHexSeed(Client::mnemonic_to_hex_seed(&mnemonic)?);

                        mnemonic.zeroize();

                        Ok(response)
                    }
                    ClientMessage::BlockId { block } => {
                        let block = Block::try_from_dto_unverified(&block)?;
                        Ok(Response::BlockId(block.id()))
                    }
                    ClientMessage::TransactionId { payload } => {
                        let payload = TransactionPayload::try_from_dto_unverified(&payload)?;
                        Ok(Response::TransactionId(payload.id()))
                    }
                    ClientMessage::ComputeAliasId { output_id } => Ok(Response::AliasId(AliasId::from(&output_id))),
                    ClientMessage::ComputeNftId { output_id } => Ok(Response::NftId(NftId::from(&output_id))),
                    ClientMessage::ComputeFoundryId {
                        alias_address,
                        serial_number,
                        token_scheme_kind,
                    } => Ok(Response::FoundryId(FoundryId::build(
                        &alias_address,
                        serial_number,
                        token_scheme_kind,
                    ))),
                    ClientMessage::Faucet { url, address } => {
                        Ok(Response::Faucet(request_funds_from_faucet(&url, &address).await?))
                    }
                    ClientMessage::HashTransactionEssence { essence } => Ok(Response::TransactionEssenceHash(
                        prefix_hex::encode(TransactionEssence::try_from_dto_unverified(&essence)?.hash()),
                    )),
                }
            }
            Message::Wallet(message) => match message {
                WalletMessage::CreateAccount { alias, bech32_hrp } => self.create_account(alias, bech32_hrp).await,
                WalletMessage::GetAccount { account_id } => self.get_account(&account_id).await,
                WalletMessage::GetAccountIndexes => {
                    let accounts = self.account_manager.get_accounts().await?;
                    let mut account_indexes = Vec::new();
                    for account in accounts.iter() {
                        account_indexes.push(*account.read().await.index());
                    }
                    Ok(Response::AccountIndexes(account_indexes))
                }
                WalletMessage::GetAccounts => self.get_accounts().await,
                WalletMessage::CallAccountMethod { account_id, method } => {
                    self.call_account_method(&account_id, method).await
                }
                #[cfg(feature = "stronghold")]
                WalletMessage::Backup { destination, password } => {
                    self.backup(destination.to_path_buf(), password).await
                }
                #[cfg(feature = "stronghold")]
                WalletMessage::ChangeStrongholdPassword {
                    mut current_password,
                    mut new_password,
                } => {
                    self.account_manager
                        .change_stronghold_password(&current_password, &new_password)
                        .await?;
                    current_password.zeroize();
                    new_password.zeroize();
                    Ok(Response::Ok)
                }
                #[cfg(feature = "stronghold")]
                WalletMessage::ClearStrongholdPassword => {
                    self.account_manager.clear_stronghold_password().await?;
                    Ok(Response::Ok)
                }
                #[cfg(feature = "stronghold")]
                WalletMessage::IsStrongholdPasswordAvailable => {
                    let is_available = self.account_manager.is_stronghold_password_available().await?;
                    Ok(Response::StrongholdPasswordIsAvailable(is_available))
                }
                WalletMessage::RecoverAccounts {
                    account_start_index,
                    account_gap_limit,
                    address_gap_limit,
                    sync_options,
                } => {
                    let account_handles = self
                        .account_manager
                        .recover_accounts(account_start_index, account_gap_limit, address_gap_limit, sync_options)
                        .await?;
                    let mut accounts = Vec::new();
                    for account_handle in account_handles {
                        let account = account_handle.read().await;
                        accounts.push(AccountDto::from(&*account));
                    }
                    Ok(Response::Accounts(accounts))
                }
                WalletMessage::RemoveLatestAccount => {
                    self.account_manager.remove_latest_account().await?;
                    Ok(Response::Ok)
                }
                #[cfg(feature = "stronghold")]
                WalletMessage::RestoreBackup {
                    source,
                    password,
                    ignore_if_coin_type_mismatch,
                } => {
                    self.restore_backup(source.to_path_buf(), password, ignore_if_coin_type_mismatch)
                        .await
                }
                WalletMessage::GenerateMnemonic => convert_panics(|| {
                    self.account_manager
                        .generate_mnemonic()
                        .map(Response::GeneratedMnemonic)
                        .map_err(Into::into)
                }),
                WalletMessage::VerifyMnemonic { mut mnemonic } => convert_panics(|| {
                    self.account_manager.verify_mnemonic(&mnemonic)?;
                    mnemonic.zeroize();
                    Ok(Response::Ok)
                }),
                WalletMessage::SetClientOptions { client_options } => {
                    self.account_manager.set_client_options(*client_options).await?;
                    Ok(Response::Ok)
                }
                #[cfg(feature = "ledger_nano")]
                WalletMessage::GetLedgerNanoStatus => {
                    let ledger_nano_status = self.account_manager.get_ledger_nano_status().await?;
                    Ok(Response::LedgerNanoStatus(ledger_nano_status))
                }
                WalletMessage::GenerateAddress {
                    account_index,
                    internal,
                    address_index,
                    options,
                    bech32_hrp,
                } => {
                    let address = self
                        .account_manager
                        .generate_address(account_index, internal, address_index, options)
                        .await?;

                    let bech32_hrp = match bech32_hrp {
                        Some(bech32_hrp) => bech32_hrp,
                        None => self.account_manager.get_bech32_hrp().await?,
                    };

                    Ok(Response::Bech32Address(address.to_bech32(bech32_hrp)))
                }
                WalletMessage::GetNodeInfo { url, auth } => match url {
                    Some(url) => {
                        let node_info = Client::get_node_info(&url, auth).await?;
                        Ok(Response::NodeInfoWrapper(NodeInfoWrapper { node_info, url }))
                    }
                    None => self
                        .account_manager
                        .get_node_info()
                        .await
                        .map(Response::NodeInfoWrapper)
                        .map_err(Into::into),
                },
                #[cfg(feature = "stronghold")]
                WalletMessage::SetStrongholdPassword { mut password } => {
                    self.account_manager.set_stronghold_password(&password).await?;
                    password.zeroize();
                    Ok(Response::Ok)
                }
                #[cfg(feature = "stronghold")]
                WalletMessage::SetStrongholdPasswordClearInterval {
                    interval_in_milliseconds,
                } => {
                    let duration = interval_in_milliseconds.map(Duration::from_millis);
                    self.account_manager
                        .set_stronghold_password_clear_interval(duration)
                        .await?;
                    Ok(Response::Ok)
                }
                #[cfg(feature = "stronghold")]
                WalletMessage::StoreMnemonic { mnemonic } => {
                    self.account_manager.store_mnemonic(mnemonic).await?;
                    Ok(Response::Ok)
                }
                WalletMessage::StartBackgroundSync {
                    options,
                    interval_in_milliseconds,
                } => {
                    let duration = interval_in_milliseconds.map(Duration::from_millis);
                    self.account_manager.start_background_syncing(options, duration).await?;
                    Ok(Response::Ok)
                }
                WalletMessage::StopBackgroundSync => {
                    self.account_manager.stop_background_syncing().await?;
                    Ok(Response::Ok)
                }
                #[cfg(feature = "events")]
                WalletMessage::EmitTestEvent { event } => {
                    self.account_manager.emit_test_event(event.clone()).await?;
                    Ok(Response::Ok)
                }
                WalletMessage::Bech32ToHex { bech32_address } => {
                    convert_panics(|| Ok(Response::HexAddress(utils::bech32_to_hex(&bech32_address)?)))
                }
                WalletMessage::HexToBech32 { hex, bech32_hrp } => {
                    let bech32_hrp = match bech32_hrp {
                        Some(bech32_hrp) => bech32_hrp,
                        None => match self.account_manager.get_node_info().await {
                            Ok(node_info_wrapper) => node_info_wrapper.node_info.protocol.bech32_hrp,
                            Err(_) => SHIMMER_TESTNET_BECH32_HRP.into(),
                        },
                    };

                    Ok(Response::Bech32Address(utils::hex_to_bech32(&hex, &bech32_hrp)?))
                }
                #[cfg(feature = "events")]
                WalletMessage::ClearListeners { event_types } => {
                    self.account_manager.clear_listeners(event_types).await;
                    Ok(Response::Ok)
                }
                WalletMessage::UpdateNodeAuth { url, auth } => {
                    self.account_manager.update_node_auth(url, auth).await?;
                    Ok(Response::Ok)
                }
            },
        }
    }
}
