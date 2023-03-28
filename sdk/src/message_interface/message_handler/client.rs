// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use zeroize::Zeroize;
#[cfg(feature = "mqtt")]
use {
    crate::client::mqtt::{MqttPayload, Topic},
    crate::types::block::payload::milestone::option::dto::ReceiptMilestoneOptionDto,
};

#[cfg(feature = "ledger_nano")]
use crate::client::secret::ledger_nano::LedgerSecretManager;
use crate::{
    client::{
        api::{PreparedTransactionData, PreparedTransactionDataDto},
        request_funds_from_faucet,
        secret::{SecretManage, SecretManager},
        Client,
    },
    message_interface::{message::ClientMessage, message_handler::Result, response::Response},
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
};

impl Client {
    /// Listen to MQTT events
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    pub async fn listen<F>(&self, topics: Vec<Topic>, handler: F)
    where
        F: Fn(String) + 'static + Clone + Send + Sync,
    {
        self.subscribe(topics, move |topic_event| {
            #[derive(Serialize)]
            struct MqttResponse {
                topic: String,
                payload: String,
            }
            // convert types to DTOs
            let payload = match &topic_event.payload {
                MqttPayload::Json(val) => serde_json::to_string(&val).expect("failed to serialize MqttPayload::Json"),
                MqttPayload::Block(block) => {
                    serde_json::to_string(&BlockDto::from(block)).expect("failed to serialize MqttPayload::Block")
                }
                MqttPayload::MilestonePayload(ms) => serde_json::to_string(&MilestonePayloadDto::from(ms))
                    .expect("failed to serialize MqttPayload::MilestonePayload"),
                MqttPayload::Receipt(receipt) => serde_json::to_string(&ReceiptMilestoneOptionDto::from(receipt))
                    .expect("failed to serialize MqttPayload::Receipt"),
            };
            let response = MqttResponse {
                topic: topic_event.topic.clone(),
                payload,
            };

            handler(serde_json::to_string(&response).expect("failed to serialize MQTT response"))
        })
        .await
        .expect("failed to listen to MQTT events");
    }
    /// Handle a message.
    pub(crate) async fn handle_message(&self, message: ClientMessage) -> Result<Response> {
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
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.get_rent_structure().await?)
                    },
                    native_tokens,
                    &alias_id,
                    state_index,
                    state_metadata.map(prefix_hex::decode).transpose()?,
                    foundry_counter,
                    unlock_conditions,
                    features,
                    immutable_features,
                    self.get_token_supply().await?,
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
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.get_rent_structure().await?)
                    },
                    native_tokens,
                    unlock_conditions,
                    features,
                    self.get_token_supply().await?,
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
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.get_rent_structure().await?)
                    },
                    native_tokens,
                    serial_number,
                    &token_scheme,
                    unlock_conditions,
                    features,
                    immutable_features,
                    self.get_token_supply().await?,
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
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.get_rent_structure().await?)
                    },
                    native_tokens,
                    &nft_id,
                    unlock_conditions,
                    features,
                    immutable_features,
                    self.get_token_supply().await?,
                )?);

                Ok(Response::BuiltOutput(OutputDto::from(&output)))
            }
            ClientMessage::GenerateAddresses {
                secret_manager,
                options,
            } => {
                let secret_manager = (&secret_manager).try_into()?;
                let addresses = self
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
                let mut block_builder = self.block();

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
                self.unsubscribe(topics).await?;
                Ok(Response::Ok)
            }
            ClientMessage::GetNode => Ok(Response::Node(self.get_node()?)),
            ClientMessage::GetNetworkInfo => Ok(Response::NetworkInfo(self.get_network_info().await?.into())),
            ClientMessage::GetNetworkId => Ok(Response::NetworkId(self.get_network_id().await?)),
            ClientMessage::GetBech32Hrp => Ok(Response::Bech32Hrp(self.get_bech32_hrp().await?)),
            ClientMessage::GetMinPowScore => Ok(Response::MinPowScore(self.get_min_pow_score().await?)),
            ClientMessage::GetTipsInterval => Ok(Response::TipsInterval(self.get_tips_interval())),
            ClientMessage::GetProtocolParameters => {
                let params = self.get_protocol_parameters().await?;
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
            ClientMessage::GetLocalPow => Ok(Response::LocalPow(self.get_local_pow())),
            ClientMessage::GetFallbackToLocalPow => Ok(Response::FallbackToLocalPow(self.get_fallback_to_local_pow())),
            #[cfg(feature = "ledger_nano")]
            ClientMessage::GetLedgerNanoStatus { is_simulator } => {
                let ledger_nano = LedgerSecretManager::new(is_simulator);

                Ok(Response::LedgerNanoStatus(ledger_nano.get_ledger_nano_status().await))
            }
            ClientMessage::PrepareTransaction {
                secret_manager,
                options,
            } => {
                let mut block_builder = self.block();

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
                let mut block_builder = self.block();

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
                let block_builder = self.block();

                let block = block_builder
                    .finish_block(Some(Payload::try_from_dto(
                        &payload_dto,
                        &self.get_protocol_parameters().await?,
                    )?))
                    .await?;

                let block_id = block.id();

                Ok(Response::BlockIdWithBlock(block_id, BlockDto::from(&block)))
            }
            #[cfg(not(target_family = "wasm"))]
            ClientMessage::UnhealthyNodes => Ok(Response::UnhealthyNodes(
                self.unhealthy_nodes().into_iter().cloned().collect(),
            )),
            ClientMessage::GetHealth { url } => Ok(Response::Health(self.get_health(&url).await?)),
            ClientMessage::GetNodeInfo { url, auth } => Ok(Response::NodeInfo(Self::get_node_info(&url, auth).await?)),
            ClientMessage::GetInfo => Ok(Response::Info(self.get_info().await?)),
            ClientMessage::GetPeers => Ok(Response::Peers(self.get_peers().await?)),
            ClientMessage::GetTips => Ok(Response::Tips(self.get_tips().await?)),
            ClientMessage::PostBlockRaw { block_bytes } => Ok(Response::BlockId(
                self.post_block_raw(&Block::unpack_strict(
                    &block_bytes[..],
                    &self.get_protocol_parameters().await?,
                )?)
                .await?,
            )),
            ClientMessage::PostBlock { block } => Ok(Response::BlockId(
                self.post_block(&Block::try_from_dto(&block, &self.get_protocol_parameters().await?)?)
                    .await?,
            )),
            ClientMessage::GetBlock { block_id } => {
                Ok(Response::Block(BlockDto::from(&self.get_block(&block_id).await?)))
            }
            ClientMessage::GetBlockMetadata { block_id } => {
                Ok(Response::BlockMetadata(self.get_block_metadata(&block_id).await?))
            }
            ClientMessage::GetBlockRaw { block_id } => Ok(Response::BlockRaw(self.get_block_raw(&block_id).await?)),
            ClientMessage::GetOutput { output_id } => {
                Ok(Response::OutputWithMetadataResponse(self.get_output(&output_id).await?))
            }
            ClientMessage::GetOutputMetadata { output_id } => {
                Ok(Response::OutputMetadata(self.get_output_metadata(&output_id).await?))
            }
            ClientMessage::GetMilestoneById { milestone_id } => Ok(Response::Milestone(MilestonePayloadDto::from(
                &self.get_milestone_by_id(&milestone_id).await?,
            ))),
            ClientMessage::GetMilestoneByIdRaw { milestone_id } => Ok(Response::MilestoneRaw(
                self.get_milestone_by_id_raw(&milestone_id).await?,
            )),
            ClientMessage::GetMilestoneByIndex { index } => Ok(Response::Milestone(MilestonePayloadDto::from(
                &self.get_milestone_by_index(index).await?,
            ))),
            ClientMessage::GetMilestoneByIndexRaw { index } => {
                Ok(Response::MilestoneRaw(self.get_milestone_by_index_raw(index).await?))
            }
            ClientMessage::GetUtxoChangesById { milestone_id } => Ok(Response::MilestoneUtxoChanges(
                self.get_utxo_changes_by_id(&milestone_id).await?,
            )),
            ClientMessage::GetUtxoChangesByIndex { index } => Ok(Response::MilestoneUtxoChanges(
                self.get_utxo_changes_by_index(index).await?,
            )),
            ClientMessage::GetReceipts => Ok(Response::Receipts(self.get_receipts().await?)),
            ClientMessage::GetReceiptsMigratedAt { milestone_index } => Ok(Response::Receipts(
                self.get_receipts_migrated_at(milestone_index).await?,
            )),
            ClientMessage::GetTreasury => Ok(Response::Treasury(self.get_treasury().await?)),
            ClientMessage::GetIncludedBlock { transaction_id } => Ok(Response::Block(BlockDto::from(
                &self.get_included_block(&transaction_id).await?,
            ))),
            ClientMessage::GetIncludedBlockMetadata { transaction_id } => Ok(Response::BlockMetadata(
                self.get_included_block_metadata(&transaction_id).await?,
            )),
            ClientMessage::BasicOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                self.basic_output_ids(query_parameters).await?,
            )),
            ClientMessage::AliasOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                self.alias_output_ids(query_parameters).await?,
            )),
            ClientMessage::AliasOutputId { alias_id } => Ok(Response::OutputId(self.alias_output_id(alias_id).await?)),
            ClientMessage::NftOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                self.nft_output_ids(query_parameters).await?,
            )),
            ClientMessage::NftOutputId { nft_id } => Ok(Response::OutputId(self.nft_output_id(nft_id).await?)),
            ClientMessage::FoundryOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                self.foundry_output_ids(query_parameters).await?,
            )),
            ClientMessage::FoundryOutputId { foundry_id } => {
                Ok(Response::OutputId(self.foundry_output_id(foundry_id).await?))
            }
            ClientMessage::GetOutputs { output_ids } => Ok(Response::Outputs(self.get_outputs(output_ids).await?)),
            ClientMessage::TryGetOutputs { output_ids } => {
                Ok(Response::Outputs(self.try_get_outputs(output_ids).await?))
            }
            ClientMessage::FindBlocks { block_ids } => Ok(Response::Blocks(
                self.find_blocks(&block_ids).await?.iter().map(BlockDto::from).collect(),
            )),
            ClientMessage::Retry { block_id } => {
                let (block_id, block) = self.retry(&block_id).await?;
                Ok(Response::BlockIdWithBlock(block_id, BlockDto::from(&block)))
            }
            ClientMessage::RetryUntilIncluded {
                block_id,
                interval,
                max_attempts,
            } => {
                let res = self.retry_until_included(&block_id, interval, max_attempts).await?;
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
                    self.consolidate_funds(&secret_manager, generate_addresses_options)
                        .await?,
                ))
            }
            ClientMessage::FindInputs { addresses, amount } => Ok(Response::Inputs(
                self.find_inputs(addresses, amount)
                    .await?
                    .iter()
                    .map(UtxoInputDto::from)
                    .collect(),
            )),
            ClientMessage::FindOutputs { output_ids, addresses } => {
                Ok(Response::Outputs(self.find_outputs(&output_ids, &addresses).await?))
            }
            ClientMessage::Reattach { block_id } => {
                let (block_id, block) = self.reattach(&block_id).await?;
                Ok(Response::Reattached((block_id, BlockDto::from(&block))))
            }
            ClientMessage::ReattachUnchecked { block_id } => {
                let (block_id, block) = self.reattach_unchecked(&block_id).await?;
                Ok(Response::Reattached((block_id, BlockDto::from(&block))))
            }
            ClientMessage::Promote { block_id } => {
                let (block_id, block) = self.promote(&block_id).await?;
                Ok(Response::Promoted((block_id, BlockDto::from(&block))))
            }
            ClientMessage::PromoteUnchecked { block_id } => {
                let (block_id, block) = self.promote_unchecked(&block_id).await?;
                Ok(Response::Promoted((block_id, BlockDto::from(&block))))
            }
            ClientMessage::Bech32ToHex { bech32 } => Ok(Response::Bech32ToHex(Client::bech32_to_hex(&bech32)?)),
            ClientMessage::HexToBech32 { hex, bech32_hrp } => Ok(Response::Bech32Address(
                self.hex_to_bech32(&hex, bech32_hrp.as_deref()).await?,
            )),
            ClientMessage::AliasIdToBech32 { alias_id, bech32_hrp } => Ok(Response::Bech32Address(
                self.alias_id_to_bech32(alias_id, bech32_hrp.as_deref()).await?,
            )),
            ClientMessage::NftIdToBech32 { nft_id, bech32_hrp } => Ok(Response::Bech32Address(
                self.nft_id_to_bech32(nft_id, bech32_hrp.as_deref()).await?,
            )),
            ClientMessage::HexPublicKeyToBech32Address { hex, bech32_hrp } => Ok(Response::Bech32Address(
                self.hex_public_key_to_bech32_address(&hex, bech32_hrp.as_deref())
                    .await?,
            )),
            ClientMessage::ParseBech32Address { address } => Ok(Response::ParsedBech32Address(AddressDto::from(
                &Address::try_from_bech32(address)?,
            ))),
            ClientMessage::IsAddressValid { address } => {
                Ok(Response::IsAddressValid(Address::is_valid_bech32(&address)))
            }
            ClientMessage::GenerateMnemonic => Ok(Response::GeneratedMnemonic(Self::generate_mnemonic()?)),
            ClientMessage::MnemonicToHexSeed { mut mnemonic } => {
                let response = Response::MnemonicHexSeed(Self::mnemonic_to_hex_seed(&mnemonic)?);

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
}
