// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "mqtt")]
use iota_sdk::client::mqtt::{MqttPayload, Topic};
use iota_sdk::{
    client::{request_funds_from_faucet, Client},
    types::{
        block::{
            output::{
                AccountOutputBuilder, BasicOutputBuilder, FoundryOutputBuilder, MinimumOutputAmount, NftOutputBuilder,
            },
            payload::Payload,
            Block, BlockDto, UnsignedBlockDto,
        },
        TryFromDto,
    },
};

use crate::{method::ClientMethod, response::Response};

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
            let payload = match &topic_event.payload {
                MqttPayload::Json(val) => serde_json::to_string(&val).expect("failed to serialize MqttPayload::Json"),
                MqttPayload::Block(block) => {
                    serde_json::to_string(block).expect("failed to serialize MqttPayload::Block")
                }
                e => panic!("received unknown mqtt type: {e:?}"),
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
pub(crate) async fn call_client_method_internal(
    client: &Client,
    method: ClientMethod,
) -> Result<Response, crate::Error> {
    let response = match method {
        ClientMethod::BuildAccountOutput {
            amount,
            mana,
            account_id,
            foundry_counter,
            unlock_conditions,
            features,
            immutable_features,
        } => {
            let mut output_builder = if let Some(amount) = amount {
                AccountOutputBuilder::new_with_amount(amount, account_id)
            } else {
                AccountOutputBuilder::new_with_minimum_amount(client.get_storage_score_parameters().await?, account_id)
            }
            .with_mana(mana)
            .with_foundry_counter(foundry_counter)
            .with_unlock_conditions(unlock_conditions);

            if let Some(features) = features {
                output_builder = output_builder.with_features(features);
            }
            if let Some(immutable_features) = immutable_features {
                output_builder = output_builder.with_immutable_features(immutable_features)
            }

            Response::Output(output_builder.finish_output()?)
        }
        ClientMethod::BuildBasicOutput {
            amount,
            mana,
            unlock_conditions,
            features,
        } => {
            let mut output_builder = if let Some(amount) = amount {
                BasicOutputBuilder::new_with_amount(amount)
            } else {
                BasicOutputBuilder::new_with_minimum_amount(client.get_storage_score_parameters().await?)
            }
            .with_mana(mana)
            .with_unlock_conditions(unlock_conditions);

            if let Some(features) = features {
                output_builder = output_builder.with_features(features);
            }

            Response::Output(output_builder.finish_output()?)
        }
        ClientMethod::BuildFoundryOutput {
            amount,
            serial_number,
            token_scheme,
            unlock_conditions,
            features,
            immutable_features,
        } => {
            let mut output_builder = if let Some(amount) = amount {
                FoundryOutputBuilder::new_with_amount(amount, serial_number, token_scheme)
            } else {
                FoundryOutputBuilder::new_with_minimum_amount(
                    client.get_storage_score_parameters().await?,
                    serial_number,
                    token_scheme,
                )
            }
            .with_unlock_conditions(unlock_conditions);

            if let Some(features) = features {
                output_builder = output_builder.with_features(features);
            }
            if let Some(immutable_features) = immutable_features {
                output_builder = output_builder.with_immutable_features(immutable_features)
            }

            Response::Output(output_builder.finish_output()?)
        }
        ClientMethod::BuildNftOutput {
            amount,
            mana,
            nft_id,
            unlock_conditions,
            features,
            immutable_features,
        } => {
            let mut output_builder = if let Some(amount) = amount {
                NftOutputBuilder::new_with_amount(amount, nft_id)
            } else {
                NftOutputBuilder::new_with_minimum_amount(client.get_storage_score_parameters().await?, nft_id)
            }
            .with_mana(mana)
            .with_unlock_conditions(unlock_conditions);

            if let Some(features) = features {
                output_builder = output_builder.with_features(features);
            }
            if let Some(immutable_features) = immutable_features {
                output_builder = output_builder.with_immutable_features(immutable_features)
            }

            Response::Output(output_builder.finish_output()?)
        }
        ClientMethod::BuildBasicBlock { issuer_id, payload } => {
            let payload = if let Some(payload) = payload {
                Some(Payload::try_from_dto_with_params(
                    payload,
                    &client.get_protocol_parameters().await?,
                )?)
            } else {
                None
            };
            Response::UnsignedBlock(UnsignedBlockDto::from(
                &client.build_basic_block(issuer_id, payload).await?,
            ))
        }
        #[cfg(feature = "mqtt")]
        ClientMethod::ClearListeners { topics } => {
            client.unsubscribe(topics).await?;
            Response::Ok
        }
        ClientMethod::GetNode => Response::Node(client.get_node().await?),
        ClientMethod::GetNetworkId => Response::NetworkId(client.get_network_id().await?.to_string()),
        ClientMethod::GetBech32Hrp => Response::Bech32Hrp(client.get_bech32_hrp().await?),
        ClientMethod::GetProtocolParameters => Response::ProtocolParameters(client.get_protocol_parameters().await?),
        #[cfg(not(target_family = "wasm"))]
        ClientMethod::UnhealthyNodes => Response::UnhealthyNodes(client.unhealthy_nodes().await.into_iter().collect()),
        ClientMethod::GetHealth { url } => Response::Bool(client.get_health(&url).await?),
        ClientMethod::GetInfo { url, auth } => Response::Info(Client::get_info(&url, auth).await?),
        ClientMethod::GetNodeInfo => Response::NodeInfo(client.get_node_info().await?),
        ClientMethod::GetNetworkMetrics => Response::NetworkMetrics(client.get_network_metrics().await?),
        ClientMethod::GetRoutes => Response::Routes(client.get_routes().await?),
        ClientMethod::GetAccountCongestion { account_id, work_score } => {
            Response::Congestion(client.get_account_congestion(&account_id, work_score).await?)
        }
        ClientMethod::GetOutputManaRewards { output_id, slot } => {
            Response::ManaRewards(client.get_output_mana_rewards(&output_id, slot).await?)
        }
        ClientMethod::GetValidators { page_size, cursor } => {
            Response::Validators(client.get_validators(page_size, cursor).await?)
        }
        ClientMethod::GetValidator { account_id } => Response::Validator(client.get_validator(&account_id).await?),
        ClientMethod::GetCommittee { epoch } => Response::Committee(client.get_committee(epoch).await?),
        ClientMethod::GetIssuance => Response::Issuance(client.get_issuance().await?),
        ClientMethod::PostBlockRaw { block_bytes } => Response::BlockId(
            client
                .post_block_raw(&Block::unpack_strict(
                    &block_bytes[..],
                    &client.get_protocol_parameters().await?,
                )?)
                .await?,
        ),
        ClientMethod::PostBlock { block } => Response::BlockId(
            client
                .post_block(&Block::try_from_dto_with_params(
                    block,
                    &client.get_protocol_parameters().await?,
                )?)
                .await?,
        ),
        ClientMethod::GetBlock { block_id } => Response::Block(BlockDto::from(&client.get_block(&block_id).await?)),
        ClientMethod::GetBlockRaw { block_id } => Response::Raw(client.get_block_raw(&block_id).await?),
        ClientMethod::GetBlockMetadata { block_id } => {
            Response::BlockMetadata(client.get_block_metadata(&block_id).await?)
        }
        ClientMethod::GetBlockWithMetadata { block_id } => {
            Response::BlockWithMetadata(client.get_block_with_metadata(&block_id).await?)
        }
        ClientMethod::GetOutput { output_id } => Response::OutputResponse(client.get_output(&output_id).await?),
        ClientMethod::GetOutputRaw { output_id } => Response::Raw(client.get_output_raw(&output_id).await?),
        ClientMethod::GetOutputMetadata { output_id } => {
            Response::OutputMetadata(client.get_output_metadata(&output_id).await?)
        }
        ClientMethod::GetOutputWithMetadata { output_id } => {
            Response::OutputWithMetadata(client.get_output_with_metadata(&output_id).await?)
        }
        ClientMethod::GetOutputs { output_ids } => Response::Outputs(client.get_outputs(&output_ids).await?),
        ClientMethod::GetOutputsIgnoreNotFound { output_ids } => {
            Response::Outputs(client.get_outputs_ignore_not_found(&output_ids).await?)
        }
        ClientMethod::GetIncludedBlock { transaction_id } => {
            Response::Block(BlockDto::from(&client.get_included_block(&transaction_id).await?))
        }
        ClientMethod::GetIncludedBlockRaw { transaction_id } => {
            Response::Raw(client.get_included_block_raw(&transaction_id).await?)
        }
        ClientMethod::GetIncludedBlockMetadata { transaction_id } => {
            Response::BlockMetadata(client.get_included_block_metadata(&transaction_id).await?)
        }
        ClientMethod::GetTransactionMetadata { transaction_id } => {
            Response::TransactionMetadata(client.get_transaction_metadata(&transaction_id).await?)
        }
        ClientMethod::GetCommitment { commitment_id } => {
            Response::SlotCommitment(client.get_commitment(&commitment_id).await?)
        }
        ClientMethod::GetCommitmentRaw { commitment_id } => {
            Response::Raw(client.get_commitment_raw(&commitment_id).await?)
        }
        ClientMethod::GetUtxoChanges { commitment_id } => {
            Response::UtxoChanges(client.get_utxo_changes(&commitment_id).await?)
        }
        ClientMethod::GetUtxoChangesFull { commitment_id } => {
            Response::UtxoChangesFull(client.get_utxo_changes_full(&commitment_id).await?)
        }
        ClientMethod::GetCommitmentBySlot { slot } => {
            Response::SlotCommitment(client.get_commitment_by_slot(slot).await?)
        }
        ClientMethod::GetCommitmentBySlotRaw { slot } => Response::Raw(client.get_commitment_by_slot_raw(slot).await?),
        ClientMethod::GetUtxoChangesBySlot { slot } => {
            Response::UtxoChanges(client.get_utxo_changes_by_slot(slot).await?)
        }
        ClientMethod::GetUtxoChangesFullBySlot { slot } => {
            Response::UtxoChangesFull(client.get_utxo_changes_full_by_slot(slot).await?)
        }
        ClientMethod::OutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.output_ids(query_parameters).await?)
        }
        ClientMethod::BasicOutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.basic_output_ids(query_parameters).await?)
        }
        ClientMethod::AccountOutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.account_output_ids(query_parameters).await?)
        }
        ClientMethod::AccountOutputId { account_id } => Response::OutputId(client.account_output_id(account_id).await?),
        ClientMethod::AnchorOutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.anchor_output_ids(query_parameters).await?)
        }
        ClientMethod::AnchorOutputId { anchor_id } => Response::OutputId(client.anchor_output_id(anchor_id).await?),
        ClientMethod::DelegationOutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.delegation_output_ids(query_parameters).await?)
        }
        ClientMethod::DelegationOutputId { delegation_id } => {
            Response::OutputId(client.delegation_output_id(delegation_id).await?)
        }
        ClientMethod::FoundryOutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.foundry_output_ids(query_parameters).await?)
        }
        ClientMethod::FoundryOutputId { foundry_id } => Response::OutputId(client.foundry_output_id(foundry_id).await?),
        ClientMethod::NftOutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.nft_output_ids(query_parameters).await?)
        }
        ClientMethod::NftOutputId { nft_id } => Response::OutputId(client.nft_output_id(nft_id).await?),
        ClientMethod::FindBlocks { block_ids } => Response::Blocks(
            client
                .find_blocks(&block_ids)
                .await?
                .iter()
                .map(BlockDto::from)
                .collect(),
        ),
        ClientMethod::FindInputs { addresses, amount } => {
            Response::Inputs(client.find_inputs(addresses, amount).await?)
        }
        ClientMethod::AddressToBech32 { address, bech32_hrp } => {
            Response::Bech32Address(client.address_to_bech32(address, bech32_hrp).await?)
        }
        ClientMethod::ComputeMinimumOutputAmount { output } => {
            let storage_score_params = client.get_storage_score_parameters().await?;

            Response::Amount(output.minimum_amount(storage_score_params))
        }
        ClientMethod::RequestFundsFromFaucet { url, address } => {
            Response::Faucet(request_funds_from_faucet(&url, &address).await?)
        }
        ClientMethod::CallPluginRoute {
            base_plugin_path,
            method,
            endpoint,
            query_params,
            request_object,
        } => {
            let data: serde_json::Value = client
                .call_plugin_route(&base_plugin_path, &method, &endpoint, query_params, request_object)
                .await?;
            Response::CustomJson(data)
        }
        ClientMethod::BlockId { block } => {
            let protocol_parameters = client.get_protocol_parameters().await?;
            let block = Block::try_from_dto_with_params(block, &protocol_parameters)?;
            Response::BlockId(block.id(&protocol_parameters))
        }
    };
    Ok(response)
}
