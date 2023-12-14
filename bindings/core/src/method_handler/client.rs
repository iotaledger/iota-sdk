// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "mqtt")]
use iota_sdk::client::mqtt::{MqttPayload, Topic};
use iota_sdk::{
    client::{request_funds_from_faucet, Client},
    types::{
        api::core::OutputWithMetadataResponse,
        block::{
            output::{
                AccountOutput, BasicOutput, FoundryOutput, MinimumOutputAmount, NftOutput, Output, OutputBuilderAmount,
            },
            payload::Payload,
            Block, BlockDto, UnsignedBlockDto,
        },
        TryFromDto,
    },
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
pub(crate) async fn call_client_method_internal(client: &Client, method: ClientMethod) -> Result<Response> {
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
            let output = Output::from(AccountOutput::try_from_dtos(
                if let Some(amount) = amount {
                    OutputBuilderAmount::Amount(amount)
                } else {
                    OutputBuilderAmount::MinimumAmount(client.get_storage_score_parameters().await?)
                },
                mana,
                &account_id,
                foundry_counter,
                unlock_conditions,
                features,
                immutable_features,
            )?);

            Response::Output(output)
        }
        ClientMethod::BuildBasicOutput {
            amount,
            mana,
            unlock_conditions,
            features,
        } => {
            let output = Output::from(BasicOutput::try_from_dtos(
                if let Some(amount) = amount {
                    OutputBuilderAmount::Amount(amount)
                } else {
                    OutputBuilderAmount::MinimumAmount(client.get_storage_score_parameters().await?)
                },
                mana,
                unlock_conditions,
                features,
            )?);

            Response::Output(output)
        }
        ClientMethod::BuildFoundryOutput {
            amount,
            serial_number,
            token_scheme,
            unlock_conditions,
            features,
            immutable_features,
        } => {
            let output = Output::from(FoundryOutput::try_from_dtos(
                if let Some(amount) = amount {
                    OutputBuilderAmount::Amount(amount)
                } else {
                    OutputBuilderAmount::MinimumAmount(client.get_storage_score_parameters().await?)
                },
                serial_number,
                token_scheme,
                unlock_conditions,
                features,
                immutable_features,
            )?);

            Response::Output(output)
        }
        ClientMethod::BuildNftOutput {
            amount,
            mana,
            nft_id,
            unlock_conditions,
            features,
            immutable_features,
        } => {
            let output = Output::from(NftOutput::try_from_dtos(
                if let Some(amount) = amount {
                    OutputBuilderAmount::Amount(amount)
                } else {
                    OutputBuilderAmount::MinimumAmount(client.get_storage_score_parameters().await?)
                },
                mana,
                &nft_id,
                unlock_conditions,
                features,
                immutable_features,
            )?);

            Response::Output(output)
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
        ClientMethod::GetNetworkInfo => Response::NetworkInfo(client.get_network_info().await?),
        ClientMethod::GetNetworkId => Response::NetworkId(client.get_network_id().await?.to_string()),
        ClientMethod::GetBech32Hrp => Response::Bech32Hrp(client.get_bech32_hrp().await?),
        ClientMethod::GetProtocolParameters => Response::ProtocolParameters(client.get_protocol_parameters().await?),
        #[cfg(not(target_family = "wasm"))]
        ClientMethod::UnhealthyNodes => Response::UnhealthyNodes(client.unhealthy_nodes().await.into_iter().collect()),
        ClientMethod::GetHealth { url } => Response::Bool(client.get_health(&url).await?),
        ClientMethod::GetNodeInfo { url, auth } => Response::NodeInfo(Client::get_node_info(&url, auth).await?),
        ClientMethod::GetInfo => Response::Info(client.get_info().await?),
        ClientMethod::GetAccountCongestion { account_id } => {
            Response::Congestion(client.get_account_congestion(&account_id).await?)
        }
        ClientMethod::GetRewards { output_id, slot_index } => {
            Response::ManaRewards(client.get_output_mana_rewards(&output_id, slot_index).await?)
        }
        ClientMethod::GetValidators { page_size, cursor } => {
            Response::Validators(client.get_validators(page_size, cursor).await?)
        }
        ClientMethod::GetValidator { account_id } => Response::Validator(client.get_validator(&account_id).await?),
        ClientMethod::GetCommittee { epoch_index } => Response::Committee(client.get_committee(epoch_index).await?),
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
        ClientMethod::GetBlockMetadata { block_id } => {
            Response::BlockMetadata(client.get_block_metadata(&block_id).await?)
        }
        ClientMethod::GetBlockWithMetadata { block_id } => {
            Response::BlockWithMetadata(client.get_block_with_metadata(&block_id).await?)
        }
        ClientMethod::GetBlockRaw { block_id } => Response::Raw(client.get_block_raw(&block_id).await?),
        ClientMethod::GetOutput { output_id } => Response::OutputWithMetadataResponse(
            client
                .get_output_with_metadata(&output_id)
                .await
                .map(OutputWithMetadataResponse::from)?,
        ),
        ClientMethod::GetOutputMetadata { output_id } => {
            Response::OutputMetadata(client.get_output_metadata(&output_id).await?)
        }
        ClientMethod::GetOutputWithMetadata { output_id } => {
            Response::OutputWithMetadata(client.get_output_with_metadata(&output_id).await?)
        }
        ClientMethod::GetIncludedBlock { transaction_id } => {
            Response::Block(BlockDto::from(&client.get_included_block(&transaction_id).await?))
        }
        ClientMethod::GetIncludedBlockMetadata { transaction_id } => {
            Response::BlockMetadata(client.get_included_block_metadata(&transaction_id).await?)
        }
        ClientMethod::GetTransactionMetadata { transaction_id } => {
            Response::TransactionMetadata(client.get_transaction_metadata(&transaction_id).await?)
        }
        ClientMethod::GetCommitment { commitment_id } => {
            Response::SlotCommitment(client.get_slot_commitment_by_id(&commitment_id).await?)
        }
        ClientMethod::GetUtxoChanges { commitment_id } => {
            Response::UtxoChanges(client.get_utxo_changes_by_slot_commitment_id(&commitment_id).await?)
        }
        ClientMethod::GetUtxoChangesFull { commitment_id } => Response::UtxoChangesFull(
            client
                .get_utxo_changes_full_by_slot_commitment_id(&commitment_id)
                .await?,
        ),
        ClientMethod::GetCommitmentByIndex { index } => {
            Response::SlotCommitment(client.get_slot_commitment_by_slot(index).await?)
        }
        ClientMethod::GetUtxoChangesByIndex { index } => {
            Response::UtxoChanges(client.get_utxo_changes_by_slot(index).await?)
        }
        ClientMethod::GetUtxoChangesFullByIndex { index } => {
            Response::UtxoChangesFull(client.get_utxo_changes_full_by_slot(index).await?)
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
        ClientMethod::GetOutputs { output_ids } => {
            let outputs_response = client
                .get_outputs_with_metadata(&output_ids)
                .await?
                .iter()
                .map(OutputWithMetadataResponse::from)
                .collect();
            Response::Outputs(outputs_response)
        }
        ClientMethod::GetOutputsIgnoreErrors { output_ids } => {
            let outputs_response = client
                .get_outputs_with_metadata_ignore_not_found(&output_ids)
                .await?
                .iter()
                .map(OutputWithMetadataResponse::from)
                .collect();
            Response::Outputs(outputs_response)
        }
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
        ClientMethod::HexToBech32 { hex, bech32_hrp } => {
            Response::Bech32Address(client.hex_to_bech32(&hex, bech32_hrp).await?)
        }
        ClientMethod::AccountIdToBech32 { account_id, bech32_hrp } => {
            Response::Bech32Address(client.account_id_to_bech32(account_id, bech32_hrp).await?)
        }
        ClientMethod::NftIdToBech32 { nft_id, bech32_hrp } => {
            Response::Bech32Address(client.nft_id_to_bech32(nft_id, bech32_hrp).await?)
        }
        ClientMethod::HexPublicKeyToBech32Address { hex, bech32_hrp } => {
            Response::Bech32Address(client.hex_public_key_to_bech32_address(&hex, bech32_hrp).await?)
        }
        ClientMethod::ComputeMinimumOutputAmount { output } => {
            let storage_score_params = client.get_storage_score_parameters().await?;

            Response::OutputAmount(output.minimum_amount(storage_score_params))
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
