// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will build an account output.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example build_account_output [METADATA] [ADDRESS]
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::{
        address::Address,
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature},
            unlock_condition::{GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition},
            AccountId, AccountOutputBuilder,
        },
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let metadata = std::env::args().nth(1).unwrap_or("hello".to_string());
    let metadata = metadata.as_bytes();

    // Create a node client.
    let client = Client::builder()
        .with_node(&std::env::var("NODE_URL").unwrap())?
        .finish()
        .await?;

    let token_supply = client.get_token_supply().await?;
    let rent_structure = client.get_rent_structure().await?;

    let address = std::env::args()
        .nth(1)
        .unwrap_or("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy".to_string());
    let address = Address::try_from_bech32(address)?;

    // Account id needs to be null the first time
    let account_output = AccountOutputBuilder::new_with_minimum_amount(rent_structure, AccountId::null())
        .with_state_metadata(metadata)
        .add_feature(SenderFeature::new(address))
        .add_feature(MetadataFeature::new(metadata)?)
        .add_immutable_feature(IssuerFeature::new(address))
        .add_immutable_feature(MetadataFeature::new(metadata)?)
        .add_unlock_condition(StateControllerAddressUnlockCondition::new(address))
        .add_unlock_condition(GovernorAddressUnlockCondition::new(address))
        .finish_output(token_supply)?;

    println!("{account_output:#?}");

    Ok(())
}
