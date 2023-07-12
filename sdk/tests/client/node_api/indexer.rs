// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{api::GetAddressesOptions, Result},
    types::block::output::{
        unlock_condition::{
            AddressUnlockCondition, GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition,
            UnlockCondition,
        },
        AliasId, AliasOutputBuilder, NftId, NftOutputBuilder,
    },
};

use super::{get_alias_output_id, get_nft_output_id};
use crate::client::common::create_client_and_secret_manager_with_funds;

#[ignore]
#[tokio::test]
async fn get_alias_output_id_test() -> Result<()> {
    let (client, secret_manager) = create_client_and_secret_manager_with_funds(None).await?;
    let protocol_parameters = client.get_protocol_parameters().await?;

    let address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..3))
        .await?[0];

    let alias_output =
        AliasOutputBuilder::new_with_minimum_storage_deposit(*protocol_parameters.rent_structure(), AliasId::null())
            .with_state_metadata([1, 2, 3])
            .add_unlock_condition(StateControllerAddressUnlockCondition::new(address))
            .add_unlock_condition(GovernorAddressUnlockCondition::new(address))
            .finish_output(protocol_parameters.token_supply())?;

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs([alias_output])?
        .finish()
        .await?;

    client.retry_until_included(&block.id(), None, None).await?;

    let output_id_0 = get_alias_output_id(block.payload().unwrap())?;
    let output_id_1 = client.alias_output_id(AliasId::from(&output_id_0)).await?;

    assert_eq!(output_id_0, output_id_1);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_nft_output_id_test() -> Result<()> {
    let (client, secret_manager) = create_client_and_secret_manager_with_funds(None).await?;
    let protocol_parameters = client.get_protocol_parameters().await?;

    let address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..3))
        .await?[0];

    let nft_output =
        NftOutputBuilder::new_with_minimum_storage_deposit(*protocol_parameters.rent_structure(), NftId::null())
            .with_unlock_conditions([UnlockCondition::Address(AddressUnlockCondition::new(address))])
            .finish_output(protocol_parameters.token_supply())?;

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs([nft_output])?
        .finish()
        .await?;

    client.retry_until_included(&block.id(), None, None).await?;

    let output_id_0 = get_nft_output_id(block.payload().unwrap())?;
    let output_id_1 = client.nft_output_id(NftId::from(&output_id_0)).await?;

    assert_eq!(output_id_0, output_id_1);

    Ok(())
}
