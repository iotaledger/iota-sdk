// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "mqtt")]
use iota_sdk::client::mqtt::{MqttPayload, Topic};
use iota_sdk::{
    client::{request_funds_from_faucet, secret::SecretManager, Client},
    types::{
        api::core::response::OutputWithMetadataResponse,
        block::{
            output::{
                dto::OutputDto, AccountOutput, BasicOutput, FoundryOutput, NftOutput, Output, OutputBuilderAmount, Rent,
            },
            payload::Payload,
            Block, BlockDto,
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
            native_tokens,
            account_id,
            state_index,
            state_metadata,
            foundry_counter,
            unlock_conditions,
            features,
            immutable_features,
        } => {
            let output = Output::from(AccountOutput::try_from_dtos(
                if let Some(amount) = amount {
                    OutputBuilderAmount::Amount(amount)
                } else {
                    OutputBuilderAmount::MinimumStorageDeposit(client.get_rent_structure().await?)
                },
                mana,
                native_tokens,
                &account_id,
                state_index,
                state_metadata.map(prefix_hex::decode).transpose()?,
                foundry_counter,
                unlock_conditions,
                features,
                immutable_features,
                client.get_token_supply().await?,
            )?);

            Response::Output(OutputDto::from(&output))
        }
        ClientMethod::BuildBasicOutput {
            amount,
            mana,
            native_tokens,
            unlock_conditions,
            features,
        } => {
            let output = Output::from(BasicOutput::try_from_dtos(
                if let Some(amount) = amount {
                    OutputBuilderAmount::Amount(amount)
                } else {
                    OutputBuilderAmount::MinimumStorageDeposit(client.get_rent_structure().await?)
                },
                mana,
                native_tokens,
                unlock_conditions,
                features,
                client.get_token_supply().await?,
            )?);

            Response::Output(OutputDto::from(&output))
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
                    OutputBuilderAmount::Amount(amount)
                } else {
                    OutputBuilderAmount::MinimumStorageDeposit(client.get_rent_structure().await?)
                },
                native_tokens,
                serial_number,
                token_scheme,
                unlock_conditions,
                features,
                immutable_features,
                client.get_token_supply().await?,
            )?);

            Response::Output(OutputDto::from(&output))
        }
        ClientMethod::BuildNftOutput {
            amount,
            mana,
            native_tokens,
            nft_id,
            unlock_conditions,
            features,
            immutable_features,
        } => {
            let output = Output::from(NftOutput::try_from_dtos(
                if let Some(amount) = amount {
                    OutputBuilderAmount::Amount(amount)
                } else {
                    OutputBuilderAmount::MinimumStorageDeposit(client.get_rent_structure().await?)
                },
                mana,
                native_tokens,
                &nft_id,
                unlock_conditions,
                features,
                immutable_features,
                client.get_token_supply().await?,
            )?);

            Response::Output(OutputDto::from(&output))
        }
        #[cfg(feature = "mqtt")]
        ClientMethod::ClearListeners { topics } => {
            client.unsubscribe(topics).await?;
            Response::Ok
        }
        ClientMethod::GetNode => Response::Node(client.get_node().await?),
        ClientMethod::GetNetworkInfo => Response::NetworkInfo(client.get_network_info().await?),
        ClientMethod::GetNetworkId => Response::NetworkId(client.get_network_id().await?),
        ClientMethod::GetBech32Hrp => Response::Bech32Hrp(client.get_bech32_hrp().await?),
        ClientMethod::GetProtocolParameters => Response::ProtocolParameters(client.get_protocol_parameters().await?),
        ClientMethod::PostBlockPayload { payload } => {
            let protocol_params = client.get_protocol_parameters().await?;
            let block = client
                .finish_basic_block_builder::<SecretManager>(
                    todo!("issuer id"),
                    todo!("issuing time"),
                    None,
                    Some(Payload::try_from_dto_with_params(payload, &protocol_params)?),
                    todo!("coin type"),
                    todo!("secret manager"),
                )
                .await?;

            let block_id = block.id(&protocol_params);

            Response::BlockIdWithBlock(block_id, BlockDto::from(&block))
        }
        #[cfg(not(target_family = "wasm"))]
        ClientMethod::UnhealthyNodes => Response::UnhealthyNodes(client.unhealthy_nodes().await.into_iter().collect()),
        ClientMethod::GetHealth { url } => Response::Bool(client.get_health(&url).await?),
        ClientMethod::GetNodeInfo { url, auth } => Response::NodeInfo(Client::get_node_info(&url, auth).await?),
        ClientMethod::GetInfo => Response::Info(client.get_info().await?),
        ClientMethod::GetPeers => Response::Peers(client.get_peers().await?),
        ClientMethod::GetTips => Response::Tips(client.get_tips().await?),
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
                    client.get_protocol_parameters().await?,
                )?)
                .await?,
        ),
        ClientMethod::GetBlock { block_id } => Response::Block(BlockDto::from(&client.get_block(&block_id).await?)),
        ClientMethod::GetBlockMetadata { block_id } => {
            Response::BlockMetadata(client.get_block_metadata(&block_id).await?)
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
        ClientMethod::GetIncludedBlock { transaction_id } => {
            Response::Block(BlockDto::from(&client.get_included_block(&transaction_id).await?))
        }
        ClientMethod::GetIncludedBlockMetadata { transaction_id } => {
            Response::BlockMetadata(client.get_included_block_metadata(&transaction_id).await?)
        }
        ClientMethod::BasicOutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.basic_output_ids(query_parameters).await?)
        }
        ClientMethod::AccountOutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.account_output_ids(query_parameters).await?)
        }
        ClientMethod::AccountOutputId { account_id } => Response::OutputId(client.account_output_id(account_id).await?),
        ClientMethod::NftOutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.nft_output_ids(query_parameters).await?)
        }
        ClientMethod::NftOutputId { nft_id } => Response::OutputId(client.nft_output_id(nft_id).await?),
        ClientMethod::FoundryOutputIds { query_parameters } => {
            Response::OutputIdsResponse(client.foundry_output_ids(query_parameters).await?)
        }
        ClientMethod::FoundryOutputId { foundry_id } => Response::OutputId(client.foundry_output_id(foundry_id).await?),
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
                .get_outputs_with_metadata_ignore_errors(&output_ids)
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
        ClientMethod::Retry { block_id } => {
            let (block_id, block) = client
                .retry::<SecretManager>(&block_id, todo!("coin type"), todo!("secret manager"))
                .await?;
            Response::BlockIdWithBlock(block_id, BlockDto::from(&block))
        }
        ClientMethod::RetryUntilIncluded {
            block_id,
            interval,
            max_attempts,
        } => {
            let res = client
                .retry_until_included::<SecretManager>(
                    &block_id,
                    interval,
                    max_attempts,
                    todo!("coin type"),
                    todo!("secret manager"),
                )
                .await?;
            let res = res
                .into_iter()
                .map(|(block_id, block)| (block_id, BlockDto::from(&block)))
                .collect();
            Response::RetryUntilIncludedSuccessful(res)
        }
        ClientMethod::FindInputs { addresses, amount } => {
            Response::Inputs(client.find_inputs(addresses, amount).await?)
        }
        ClientMethod::Reattach { block_id } => {
            let (block_id, block) = client
                .reattach::<SecretManager>(&block_id, todo!("coin type"), todo!("secret manager"))
                .await?;
            Response::Reattached((block_id, BlockDto::from(&block)))
        }
        ClientMethod::ReattachUnchecked { block_id } => {
            let (block_id, block) = client
                .reattach_unchecked::<SecretManager>(&block_id, todo!("coin type"), todo!("secret manager"))
                .await?;
            Response::Reattached((block_id, BlockDto::from(&block)))
        }
        ClientMethod::Promote { block_id } => {
            let (block_id, block) = client
                .promote::<SecretManager>(&block_id, todo!("coin type"), todo!("secret manager"))
                .await?;
            Response::Promoted((block_id, BlockDto::from(&block)))
        }
        ClientMethod::PromoteUnchecked { block_id } => {
            let (block_id, block) = client
                .promote_unchecked::<SecretManager>(&block_id, todo!("coin type"), todo!("secret manager"))
                .await?;
            Response::Promoted((block_id, BlockDto::from(&block)))
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
        ClientMethod::MinimumRequiredStorageDeposit { output } => {
            let output = Output::try_from_dto_with_params(output, client.get_token_supply().await?)?;
            let rent_structure = client.get_rent_structure().await?;

            let minimum_storage_deposit = output.rent_cost(&rent_structure);

            Response::MinimumRequiredStorageDeposit(minimum_storage_deposit.to_string())
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
            let protocol_params = client.get_protocol_parameters().await?;
            Response::BlockId(Block::try_from_dto_with_params(block, &protocol_params)?.id(&protocol_params))
        }
    };
    Ok(response)
}
