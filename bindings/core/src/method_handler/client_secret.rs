// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{secret::SecretManage, Client},
    types::{
        block::{payload::Payload, BlockWrapperDto},
        TryFromDto,
    },
};

use crate::{method::ClientSecretMethod, Response, Result};

/// Call a secret manager method.
pub(crate) async fn call_client_secret_method_internal<S: SecretManage>(
    client: &Client,
    secret_manager: &S,
    method: ClientSecretMethod,
) -> Result<Response>
where
    iota_sdk::client::Error: From<S::Error>,
{
    let response = match method {
        ClientSecretMethod::PostBasicBlockPayload {
            issuer_id,
            strong_parents,
            payload,
            chain,
        } => {
            let block = client
                .build_basic_block(
                    issuer_id,
                    None,
                    strong_parents,
                    Some(Payload::try_from_dto_with_params(
                        payload,
                        &client.get_protocol_parameters().await?,
                    )?),
                    secret_manager,
                    chain,
                )
                .await?;

            let block_id = client.block_id(&block).await?;

            Response::BlockIdWithBlock(block_id, BlockWrapperDto::from(&block))
        }
    };
    Ok(response)
}
