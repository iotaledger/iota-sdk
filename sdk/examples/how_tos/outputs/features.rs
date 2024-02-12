// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will build NFT outputs with all possible features.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example output_features
//! ```

use iota_sdk::{
    client::{Client, Result},
    types::block::{
        address::Address,
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature, TagFeature},
            unlock_condition::AddressUnlockCondition,
            NftId, NftOutputBuilder,
        },
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let storage_score_params = client.get_storage_score_parameters().await?;

    let address = Address::try_from_bech32("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")?;

    let nft_output_builder = NftOutputBuilder::new_with_minimum_amount(storage_score_params, NftId::null())
        .add_unlock_condition(AddressUnlockCondition::new(address.clone()));

    let outputs = [
        // with sender feature
        nft_output_builder
            .clone()
            .add_feature(SenderFeature::new(address.clone()))
            .finish_output()?,
        // with issuer feature
        nft_output_builder
            .clone()
            .add_immutable_feature(IssuerFeature::new(address))
            .finish_output()?,
        // with metadata feature block
        nft_output_builder
            .clone()
            .add_feature(MetadataFeature::new([("Hello".to_owned(), b"World!".to_vec())])?)
            .finish_output()?,
        // with immutable metadata feature block
        nft_output_builder
            .clone()
            .add_immutable_feature(MetadataFeature::new([("Hello".to_owned(), b"World!".to_vec())])?)
            .finish_output()?,
        // with tag feature
        nft_output_builder
            .add_feature(TagFeature::new("Hello, World!")?)
            .finish_output()?,
    ];

    // Convert output array to json array
    let json_outputs = serde_json::to_string_pretty(&outputs)?;
    println!("{json_outputs}");

    Ok(())
}
