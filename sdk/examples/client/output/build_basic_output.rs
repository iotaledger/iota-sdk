// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will build basic outputs with different feature blocks.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example build_basic_output [ADDRESS]
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::{
        address::Address,
        output::{
            feature::{MetadataFeature, SenderFeature, TagFeature},
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
                TimelockUnlockCondition,
            },
            BasicOutputBuilder,
        },
    },
};

const METADATA: &str = "Hello, World!";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let token_supply = client.get_token_supply().await?;

    let address = std::env::args()
        .nth(1)
        .unwrap_or("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy".to_string());
    let address = Address::try_from_bech32(address)?;

    let basic_output_builder =
        BasicOutputBuilder::new_with_amount(1_000_000).add_unlock_condition(AddressUnlockCondition::new(address));

    let outputs = [
        // most simple output
        basic_output_builder.clone().finish_output(token_supply)?,
        // with metadata feature block
        basic_output_builder
            .clone()
            .add_feature(MetadataFeature::new(METADATA)?)
            .finish_output(token_supply)?,
        // with storage deposit return
        basic_output_builder
            .clone()
            .add_unlock_condition(StorageDepositReturnUnlockCondition::new(
                address,
                1_000_000,
                token_supply,
            )?)
            .finish_output(token_supply)?,
        // with expiration
        basic_output_builder
            .clone()
            .add_unlock_condition(ExpirationUnlockCondition::new(address, 1)?)
            .finish_output(token_supply)?,
        // with timelock
        basic_output_builder
            .clone()
            .add_unlock_condition(TimelockUnlockCondition::new(1)?)
            .finish_output(token_supply)?,
        // with tag feature
        basic_output_builder
            .clone()
            .add_feature(TagFeature::new(METADATA)?)
            .finish_output(token_supply)?,
        // with sender feature
        basic_output_builder
            .add_feature(SenderFeature::new(address))
            .finish_output(token_supply)?,
    ];

    println!("{outputs:#?}");

    Ok(())
}
