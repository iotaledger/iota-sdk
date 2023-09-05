// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will build outputs with all possible unlock conditions.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example output_unlock_conditions
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::{
        address::Address,
        output::{
            dto::OutputDto,
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, GovernorAddressUnlockCondition,
                ImmutableAccountAddressUnlockCondition, StateControllerAddressUnlockCondition,
                StorageDepositReturnUnlockCondition, TimelockUnlockCondition,
            },
            AccountId, AccountOutputBuilder, BasicOutputBuilder, FoundryOutputBuilder, SimpleTokenScheme, TokenScheme,
        },
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let token_supply = client.get_token_supply().await?;
    let rent_structure = client.get_rent_structure().await?;

    let address = Address::try_from_bech32("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")?;
    let account_address = Address::try_from_bech32("rms1pr59qm43mjtvhcajfmupqf23x29llam88yecn6pyul80rx099krmv2fnnux")?;

    let token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(50, 0, 100)?);

    let basic_output_builder = BasicOutputBuilder::new_with_minimum_amount(rent_structure)
        .add_unlock_condition(AddressUnlockCondition::new(address));
    let account_output_builder = AccountOutputBuilder::new_with_minimum_amount(rent_structure, AccountId::null());
    let foundry_output_builder = FoundryOutputBuilder::new_with_minimum_amount(rent_structure, 1, token_scheme);

    let outputs = [
        //// most simple output
        basic_output_builder.clone().finish_output(token_supply)?,
        // with storage deposit return unlock condition
        basic_output_builder
            .clone()
            .add_unlock_condition(StorageDepositReturnUnlockCondition::new(
                address,
                1000000,
                token_supply,
            )?)
            .finish_output(token_supply)?,
        // with timeout unlock condition
        basic_output_builder
            .clone()
            .add_unlock_condition(TimelockUnlockCondition::new(1)?)
            .finish_output(token_supply)?,
        // with expiration unlock condition
        basic_output_builder
            .add_unlock_condition(ExpirationUnlockCondition::new(address, 1)?)
            .finish_output(token_supply)?,
        // with governor and state controller unlock condition
        account_output_builder
            .add_unlock_condition(GovernorAddressUnlockCondition::new(address))
            .add_unlock_condition(StateControllerAddressUnlockCondition::new(address))
            .finish_output(token_supply)?,
        // with immutable account unlock condition
        foundry_output_builder
            .add_unlock_condition(ImmutableAccountAddressUnlockCondition::new(
                *account_address.as_account(),
            ))
            .finish_output(token_supply)?,
    ];

    // Convert ouput array to json array
    let json_outputs = serde_json::to_string_pretty(&outputs.iter().map(OutputDto::from).collect::<Vec<OutputDto>>())?;
    println!("{json_outputs}");

    Ok(())
}
