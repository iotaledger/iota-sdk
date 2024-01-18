// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will build an alias output.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example build_alias_output [METADATA] [ADDRESS]
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::{
        address::Address,
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature},
            unlock_condition::{GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition},
            AliasId, AliasOutputBuilder,
        },
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL"] {
        std::env::var(var).unwrap_or_else(|_| panic!(".env variable '{var}' is undefined, see .env.example"));
    }

    let metadata = std::env::args().nth(1).unwrap_or_else(|| "hello".to_string());
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
        .unwrap_or_else(|| "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy".to_string());
    let address = Address::try_from_bech32(address)?;

    // Alias id needs to be null the first time
    let alias_output = AliasOutputBuilder::new_with_minimum_storage_deposit(rent_structure, AliasId::null())
        .with_state_metadata(metadata)
        .add_feature(SenderFeature::new(address))
        .add_feature(MetadataFeature::new(metadata)?)
        .add_immutable_feature(IssuerFeature::new(address))
        .add_immutable_feature(MetadataFeature::new(metadata)?)
        .add_unlock_condition(StateControllerAddressUnlockCondition::new(address))
        .add_unlock_condition(GovernorAddressUnlockCondition::new(address))
        .finish_output(token_supply)?;

    println!("{alias_output:#?}");

    Ok(())
}
