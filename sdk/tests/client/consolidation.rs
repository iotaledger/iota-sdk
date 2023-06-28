// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

use iota_sdk::{
    client::{api::GetAddressesOptions, node_api::indexer::query_parameters::QueryParameter, Result},
    types::block::{
        address::ToBech32Ext,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder},
    },
};

use crate::client::common::create_client_and_secret_manager_with_funds;

#[ignore]
#[tokio::test]
async fn consolidate_outputs() -> Result<()> {
    let (client, secret_manager) = create_client_and_secret_manager_with_funds(None).await?;

    let addresses = secret_manager
        .generate_ed25519_addresses(GetAddressesOptions::from_client(&client).await?.with_range(0..3))
        .await?;

    let bech32_hrp = client.get_bech32_hrp().await?;
    let output_ids_response = client
        .basic_output_ids([
            QueryParameter::Address(addresses[0].to_bech32(bech32_hrp)),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;
    assert_eq!(output_ids_response.items.len(), 1);

    let initial_output = client.get_output(&output_ids_response.items[0]).await?;
    let initial_base_coin_amount = initial_output.amount();

    // First split funds to multiple addresses
    let token_supply = client.get_token_supply().await?;
    let block = client
        .build_block()
        .with_secret_manager(&secret_manager)
        .with_outputs([
            BasicOutputBuilder::new_with_amount(1_000_000)
                .add_unlock_condition(AddressUnlockCondition::new(addresses[1]))
                .finish_output(token_supply)?,
            BasicOutputBuilder::new_with_amount(1_000_000)
                .add_unlock_condition(AddressUnlockCondition::new(addresses[2]))
                .finish_output(token_supply)?,
        ])?
        .finish()
        .await?;
    client.retry_until_included(&block.id(), None, None).await?;

    // Here all funds will be send to the address with the lowest index in the range
    let address_range = 0u32..5;
    let address = client
        .consolidate_funds(
            &secret_manager,
            GetAddressesOptions::from_client(&client)
                .await?
                .with_range(address_range),
        )
        .await?;
    assert_eq!(addresses[0], address);

    let output_ids_response = client
        .basic_output_ids([
            QueryParameter::Address(address.to_bech32(bech32_hrp)),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;
    // There is only one output at the end
    assert_eq!(output_ids_response.items.len(), 1);

    let final_output = client.get_output(&output_ids_response.items[0]).await?;
    let final_base_coin_amount = final_output.amount();
    // The output has the full amount again
    assert_eq!(final_base_coin_amount, initial_base_coin_amount);

    Ok(())
}
