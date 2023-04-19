// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example update_alias_output --release
// In this example we will update the state metadata of an alias output
// Rename `.env.example` to `.env` first

use std::{env, str::FromStr};

use iota_sdk::{
    types::block::output::{AliasId, AliasOutputBuilder, Output},
    wallet::{account::FilterOptions, Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    //  This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    account.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Replace with an AliasId
    let alias_id = AliasId::from_str("0xc94fc4d280d63c7de09c8cc49ecefba6192e104d200ab7472db9e943e0feef7c")?;

    // Get the alias output by it's alias id
    let alias_output = account
        .unspent_outputs(Some(FilterOptions {
            output_types: Some(vec![4]),
            ..Default::default()
        }))
        .await?
        .into_iter()
        .find_map(|output_data| match &output_data.output {
            Output::Alias(alias_output) => {
                let output_alias_id = alias_output.alias_id_non_null(&output_data.output_id);
                if output_alias_id == alias_id {
                    Some(output_data.clone())
                } else {
                    None
                }
            }
            _ => None,
        })
        .expect("output is not in the unspent outputs");

    let token_supply = account.client().get_token_supply().await?;
    let rent_structure = account.client().get_rent_structure().await?;

    let updated_alias_output = AliasOutputBuilder::from(alias_output.output.as_alias())
        // Minimum required storage deposit will change if the new metadata has a different size, so we will update the
        // amount
        .with_minimum_storage_deposit(rent_structure)
        .with_state_metadata("updated state metadata".as_bytes().to_vec())
        .finish_output(token_supply)?;

    // Send the updated output
    let transaction = account.send(vec![updated_alias_output], None).await?;
    println!(
        "Transaction: {} Block sent: {}/api/core/v2/blocks/{}",
        transaction.transaction_id,
        &env::var("NODE_URL").unwrap(),
        transaction.block_id.expect("no block created yet")
    );

    Ok(())
}
