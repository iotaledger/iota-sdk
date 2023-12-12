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
            unlock_condition::AddressUnlockCondition,
            AccountId, AccountOutputBuilder,
        },
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    #[allow(clippy::single_element_loop)]
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

    let storage_score_params = client.get_storage_score_parameters().await?;

    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy".to_string());
    let address = Address::try_from_bech32(address)?;

    // Account id needs to be null the first time
    let account_output = AccountOutputBuilder::new_with_minimum_amount(storage_score_params, AccountId::null())
        .add_feature(SenderFeature::new(address.clone()))
        // TODO: enable again when MetadataFeature is cleared up
        // .add_feature(MetadataFeature::new(metadata)?)
        .add_immutable_feature(IssuerFeature::new(address.clone()))
        // TODO: enable again when MetadataFeature is cleared up
        // .add_immutable_feature(MetadataFeature::new(metadata)?)
        .add_unlock_condition(AddressUnlockCondition::new(address))
        .finish_output()?;

    println!("{account_output:#?}");

    Ok(())
}
