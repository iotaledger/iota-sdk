// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{api::GetAddressesOptions, Result},
    types::block::{
        address::AliasAddress,
        output::{
            unlock_condition::{
                AddressUnlockCondition, GovernorAddressUnlockCondition, ImmutableAliasAddressUnlockCondition,
                StateControllerAddressUnlockCondition, UnlockCondition,
            },
            AliasId, AliasOutputBuilder, FoundryId, FoundryOutputBuilder, NftId, NftOutputBuilder, SimpleTokenScheme,
            TokenScheme,
        },
    },
};

use super::{get_alias_output_id, get_foundry_output_id, get_nft_output_id};
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

#[ignore]
#[tokio::test]
async fn get_foundry_output_id_test() -> Result<()> {
    let (client, secret_manager) = create_client_and_secret_manager_with_funds(None).await?;
    let protocol_parameters = client.get_protocol_parameters().await?;

    let address = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..3))
        .await?[0];

    let alias_output_0 =
        AliasOutputBuilder::new_with_minimum_storage_deposit(*protocol_parameters.rent_structure(), AliasId::null())
            .with_state_metadata([1, 2, 3])
            .add_unlock_condition(StateControllerAddressUnlockCondition::new(address))
            .add_unlock_condition(GovernorAddressUnlockCondition::new(address))
            .finish_output(protocol_parameters.token_supply())?;

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs([alias_output_0.clone()])?
        .finish()
        .await?;

    client.retry_until_included(&block.id(), None, None).await?;

    let alias_id = AliasId::from(&get_alias_output_id(block.payload().unwrap())?);

    let alias_output_1 = AliasOutputBuilder::from(alias_output_0.as_alias())
        .with_alias_id(alias_id)
        .with_state_index(alias_output_0.as_alias().state_index() + 1)
        .with_foundry_counter(alias_output_0.as_alias().foundry_counter() + 1)
        .finish_output(protocol_parameters.token_supply())?;

    let foundry_id = FoundryId::build(
        &AliasAddress::new(alias_id),
        alias_output_0.as_alias().foundry_counter() + 1,
        SimpleTokenScheme::KIND,
    );

    let foundry_output = FoundryOutputBuilder::new_with_minimum_storage_deposit(
        *protocol_parameters.rent_structure(),
        alias_output_0.as_alias().foundry_counter() + 1,
        TokenScheme::Simple(SimpleTokenScheme::new(100, 0, 500)?),
    )
    .add_unlock_condition(ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)))
    .finish_output(protocol_parameters.token_supply())?;

    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs([alias_output_1, foundry_output])?
        .finish()
        .await?;

    client.retry_until_included(&block.id(), None, None).await?;

    let output_id_0 = get_foundry_output_id(block.payload().unwrap())?;
    let output_id_1 = client.foundry_output_id(foundry_id).await?;

    assert_eq!(output_id_0, output_id_1);

    Ok(())
}
