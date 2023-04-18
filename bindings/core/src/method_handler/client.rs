// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "ledger_nano")]
use iota_sdk::client::secret::ledger_nano::LedgerSecretManager;
use iota_sdk::{
    client::{
        api::{PreparedTransactionData, PreparedTransactionDataDto},
        Client,
    },
    types::block::{
        input::dto::UtxoInputDto,
        output::{
            dto::{OutputBuilderAmountDto, OutputDto, RentStructureDto},
            AliasOutput, BasicOutput, FoundryOutput, NftOutput, Output,
        },
        payload::{
            dto::{MilestonePayloadDto, PayloadDto},
            Payload,
        },
        protocol::dto::ProtocolParametersDto,
        Block, BlockDto,
    },
};
#[cfg(feature = "mqtt")]
use {
    iota_sdk::client::mqtt::{MqttPayload, Topic},
    iota_sdk::types::block::payload::milestone::option::dto::ReceiptMilestoneOptionDto,
};

use crate::{method::ClientMethod, response::Response, Result};

/// Listen to MQTT events
#[cfg(feature = "mqtt")]
#[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
pub async fn listen_mqtt<F>(client: &Client, topics: Vec<Topic>, handler: F)
where
    F: Fn(String) + 'static + Clone + Send + Sync,
{
    client
        .subscribe(topics, move |topic_event| {
            #[derive(serde::Serialize)]
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

/// Call a client method.
pub(crate) async fn call_client_method_internal(client: &Client, method: ClientMethod) -> Result<Response> {
    match method {
        ClientMethod::BuildAliasOutput {
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
                    OutputBuilderAmountDto::MinimumStorageDeposit(client.get_rent_structure().await?)
                },
                native_tokens,
                &alias_id,
                state_index,
                state_metadata.map(prefix_hex::decode).transpose()?,
                foundry_counter,
                unlock_conditions,
                features,
                immutable_features,
                client.get_token_supply().await?,
            )?);

            Ok(Response::Output(OutputDto::from(&output)))
        }
        ClientMethod::BuildBasicOutput {
            amount,
            native_tokens,
            unlock_conditions,
            features,
        } => {
            let output = Output::from(BasicOutput::try_from_dtos(
                if let Some(amount) = amount {
                    OutputBuilderAmountDto::Amount(amount)
                } else {
                    OutputBuilderAmountDto::MinimumStorageDeposit(client.get_rent_structure().await?)
                },
                native_tokens,
                unlock_conditions,
                features,
                client.get_token_supply().await?,
            )?);

            Ok(Response::Output(OutputDto::from(&output)))
        }
        ClientMethod::BuildFoundryOutput {
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
                    OutputBuilderAmountDto::MinimumStorageDeposit(client.get_rent_structure().await?)
                },
                native_tokens,
                serial_number,
                &token_scheme,
                unlock_conditions,
                features,
                immutable_features,
                client.get_token_supply().await?,
            )?);

            Ok(Response::Output(OutputDto::from(&output)))
        }
        ClientMethod::BuildNftOutput {
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
                    OutputBuilderAmountDto::MinimumStorageDeposit(client.get_rent_structure().await?)
                },
                native_tokens,
                &nft_id,
                unlock_conditions,
                features,
                immutable_features,
                client.get_token_supply().await?,
            )?);

            Ok(Response::Output(OutputDto::from(&output)))
        }
        ClientMethod::GenerateAddresses {
            secret_manager,
            options,
        } => {
            let secret_manager = (&secret_manager).try_into()?;
            let addresses = client
                .get_addresses(&secret_manager)
                .set_options(options)?
                .finish()
                .await?;
            Ok(Response::GeneratedAddresses(addresses))
        }
        ClientMethod::BuildAndPostBlock {
            secret_manager,
            options,
        } => {
            // Prepare transaction
            let mut block_builder = client.block();

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
        ClientMethod::ClearListeners { topics } => {
            client.unsubscribe(topics).await?;
            Ok(Response::Ok)
        }
        ClientMethod::GetNode => Ok(Response::Node(client.get_node()?)),
        ClientMethod::GetNetworkInfo => Ok(Response::NetworkInfo(client.get_network_info().await?.into())),
        ClientMethod::GetNetworkId => Ok(Response::NetworkId(client.get_network_id().await?)),
        ClientMethod::GetBech32Hrp => Ok(Response::Bech32Hrp(client.get_bech32_hrp().await?)),
        ClientMethod::GetMinPowScore => Ok(Response::MinPowScore(client.get_min_pow_score().await?)),
        ClientMethod::GetTipsInterval => Ok(Response::TipsInterval(client.get_tips_interval())),
        ClientMethod::GetProtocolParameters => {
            let params = client.get_protocol_parameters().await?;
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
        ClientMethod::GetLocalPow => Ok(Response::Bool(client.get_local_pow())),
        ClientMethod::GetFallbackToLocalPow => Ok(Response::Bool(client.get_fallback_to_local_pow())),
        #[cfg(feature = "ledger_nano")]
        ClientMethod::GetLedgerNanoStatus { is_simulator } => {
            let ledger_nano = LedgerSecretManager::new(is_simulator);

            Ok(Response::LedgerNanoStatus(ledger_nano.get_ledger_nano_status().await))
        }
        ClientMethod::PrepareTransaction {
            secret_manager,
            options,
        } => {
            let mut block_builder = client.block();

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
        ClientMethod::SignTransaction {
            secret_manager,
            prepared_transaction_data,
        } => {
            let mut block_builder = client.block();

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
        ClientMethod::PostBlockPayload { payload_dto } => {
            let block_builder = client.block();

            let block = block_builder
                .finish_block(Some(Payload::try_from_dto(
                    &payload_dto,
                    &client.get_protocol_parameters().await?,
                )?))
                .await?;

            let block_id = block.id();

            Ok(Response::BlockIdWithBlock(block_id, BlockDto::from(&block)))
        }
        #[cfg(not(target_family = "wasm"))]
        ClientMethod::UnhealthyNodes => Ok(Response::UnhealthyNodes(
            client.unhealthy_nodes().into_iter().cloned().collect(),
        )),
        ClientMethod::GetHealth { url } => Ok(Response::Bool(client.get_health(&url).await?)),
        ClientMethod::GetNodeInfo { url, auth } => Ok(Response::NodeInfo(Client::get_node_info(&url, auth).await?)),
        ClientMethod::GetInfo => Ok(Response::Info(client.get_info().await?)),
        ClientMethod::GetPeers => Ok(Response::Peers(client.get_peers().await?)),
        ClientMethod::GetTips => Ok(Response::Tips(client.get_tips().await?)),
        ClientMethod::PostBlockRaw { block_bytes } => Ok(Response::BlockId(
            client
                .post_block_raw(&Block::unpack_strict(
                    &block_bytes[..],
                    &client.get_protocol_parameters().await?,
                )?)
                .await?,
        )),
        ClientMethod::PostBlock { block } => Ok(Response::BlockId(
            client
                .post_block(&Block::try_from_dto(&block, &client.get_protocol_parameters().await?)?)
                .await?,
        )),
        ClientMethod::GetBlock { block_id } => Ok(Response::Block(BlockDto::from(&client.get_block(&block_id).await?))),
        ClientMethod::GetBlockMetadata { block_id } => {
            Ok(Response::BlockMetadata(client.get_block_metadata(&block_id).await?))
        }
        ClientMethod::GetBlockRaw { block_id } => Ok(Response::BlockRaw(client.get_block_raw(&block_id).await?)),
        ClientMethod::GetOutput { output_id } => Ok(Response::OutputWithMetadataResponse(
            client.get_output(&output_id).await?,
        )),
        ClientMethod::GetOutputMetadata { output_id } => {
            Ok(Response::OutputMetadata(client.get_output_metadata(&output_id).await?))
        }
        ClientMethod::GetMilestoneById { milestone_id } => Ok(Response::Milestone(MilestonePayloadDto::from(
            &client.get_milestone_by_id(&milestone_id).await?,
        ))),
        ClientMethod::GetMilestoneByIdRaw { milestone_id } => Ok(Response::MilestoneRaw(
            client.get_milestone_by_id_raw(&milestone_id).await?,
        )),
        ClientMethod::GetMilestoneByIndex { index } => Ok(Response::Milestone(MilestonePayloadDto::from(
            &client.get_milestone_by_index(index).await?,
        ))),
        ClientMethod::GetMilestoneByIndexRaw { index } => {
            Ok(Response::MilestoneRaw(client.get_milestone_by_index_raw(index).await?))
        }
        ClientMethod::GetUtxoChangesById { milestone_id } => Ok(Response::MilestoneUtxoChanges(
            client.get_utxo_changes_by_id(&milestone_id).await?,
        )),
        ClientMethod::GetUtxoChangesByIndex { index } => Ok(Response::MilestoneUtxoChanges(
            client.get_utxo_changes_by_index(index).await?,
        )),
        ClientMethod::GetReceipts => Ok(Response::Receipts(client.get_receipts().await?)),
        ClientMethod::GetReceiptsMigratedAt { milestone_index } => Ok(Response::Receipts(
            client.get_receipts_migrated_at(milestone_index).await?,
        )),
        ClientMethod::GetTreasury => Ok(Response::Treasury(client.get_treasury().await?)),
        ClientMethod::GetIncludedBlock { transaction_id } => Ok(Response::Block(BlockDto::from(
            &client.get_included_block(&transaction_id).await?,
        ))),
        ClientMethod::GetIncludedBlockMetadata { transaction_id } => Ok(Response::BlockMetadata(
            client.get_included_block_metadata(&transaction_id).await?,
        )),
        ClientMethod::BasicOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
            client.basic_output_ids(query_parameters).await?,
        )),
        ClientMethod::AliasOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
            client.alias_output_ids(query_parameters).await?,
        )),
        ClientMethod::AliasOutputId { alias_id } => Ok(Response::OutputId(client.alias_output_id(alias_id).await?)),
        ClientMethod::NftOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
            client.nft_output_ids(query_parameters).await?,
        )),
        ClientMethod::NftOutputId { nft_id } => Ok(Response::OutputId(client.nft_output_id(nft_id).await?)),
        ClientMethod::FoundryOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
            client.foundry_output_ids(query_parameters).await?,
        )),
        ClientMethod::FoundryOutputId { foundry_id } => {
            Ok(Response::OutputId(client.foundry_output_id(foundry_id).await?))
        }
        ClientMethod::GetOutputs { output_ids } => Ok(Response::Outputs(client.get_outputs(output_ids).await?)),
        ClientMethod::TryGetOutputs { output_ids } => Ok(Response::Outputs(client.try_get_outputs(output_ids).await?)),
        ClientMethod::FindBlocks { block_ids } => Ok(Response::Blocks(
            client
                .find_blocks(&block_ids)
                .await?
                .iter()
                .map(BlockDto::from)
                .collect(),
        )),
        ClientMethod::Retry { block_id } => {
            let (block_id, block) = client.retry(&block_id).await?;
            Ok(Response::BlockIdWithBlock(block_id, BlockDto::from(&block)))
        }
        ClientMethod::RetryUntilIncluded {
            block_id,
            interval,
            max_attempts,
        } => {
            let res = client.retry_until_included(&block_id, interval, max_attempts).await?;
            let res = res
                .into_iter()
                .map(|(block_id, block)| (block_id, BlockDto::from(&block)))
                .collect();
            Ok(Response::RetryUntilIncludedSuccessful(res))
        }
        ClientMethod::ConsolidateFunds {
            secret_manager,
            generate_addresses_options,
        } => {
            let secret_manager = (&secret_manager).try_into()?;
            Ok(Response::ConsolidatedFunds(
                client
                    .consolidate_funds(&secret_manager, generate_addresses_options)
                    .await?,
            ))
        }
        ClientMethod::FindInputs { addresses, amount } => Ok(Response::Inputs(
            client
                .find_inputs(addresses, amount)
                .await?
                .iter()
                .map(UtxoInputDto::from)
                .collect(),
        )),
        ClientMethod::FindOutputs { output_ids, addresses } => {
            Ok(Response::Outputs(client.find_outputs(&output_ids, &addresses).await?))
        }
        ClientMethod::Reattach { block_id } => {
            let (block_id, block) = client.reattach(&block_id).await?;
            Ok(Response::Reattached((block_id, BlockDto::from(&block))))
        }
        ClientMethod::ReattachUnchecked { block_id } => {
            let (block_id, block) = client.reattach_unchecked(&block_id).await?;
            Ok(Response::Reattached((block_id, BlockDto::from(&block))))
        }
        ClientMethod::Promote { block_id } => {
            let (block_id, block) = client.promote(&block_id).await?;
            Ok(Response::Promoted((block_id, BlockDto::from(&block))))
        }
        ClientMethod::PromoteUnchecked { block_id } => {
            let (block_id, block) = client.promote_unchecked(&block_id).await?;
            Ok(Response::Promoted((block_id, BlockDto::from(&block))))
        }
        ClientMethod::HexToBech32 { hex, bech32_hrp } => Ok(Response::Bech32Address(
            client.hex_to_bech32(&hex, bech32_hrp.as_deref()).await?,
        )),
        ClientMethod::AliasIdToBech32 { alias_id, bech32_hrp } => Ok(Response::Bech32Address(
            client.alias_id_to_bech32(alias_id, bech32_hrp.as_deref()).await?,
        )),
        ClientMethod::NftIdToBech32 { nft_id, bech32_hrp } => Ok(Response::Bech32Address(
            client.nft_id_to_bech32(nft_id, bech32_hrp.as_deref()).await?,
        )),
        ClientMethod::HexPublicKeyToBech32Address { hex, bech32_hrp } => Ok(Response::Bech32Address(
            client
                .hex_public_key_to_bech32_address(&hex, bech32_hrp.as_deref())
                .await?,
        )),
    }
}
