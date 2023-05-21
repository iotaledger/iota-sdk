// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will build NFT outputs with all possible features.
//!
//! `cargo run --example output_features --release`

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

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let token_supply = client.get_token_supply().await?;

    let address = Address::try_from_bech32("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")?;

    let nft_output_builder = NftOutputBuilder::new_with_amount(1_000_000, NftId::null()).add_unlock_condition(AddressUnlockCondition::new(address));

    let outputs = vec![
        // with sender feature
        nft_output_builder
            .clone()
            .add_feature(SenderFeature::new(address))
            .finish_output(token_supply)?,
        // with issuer feature
        nft_output_builder
            .clone()
            .add_immutable_feature(IssuerFeature::new(address))
            .finish_output(token_supply)?,
        // with metadata feature block
        nft_output_builder
            .clone()
            .add_feature(MetadataFeature::new("Hello, World!".as_bytes().to_owned())?)
            .finish_output(token_supply)?,
        // with immutable metadata feature block
        nft_output_builder
            .clone()
            .add_immutable_feature(MetadataFeature::new("Hello, World!".as_bytes().to_owned())?)
            .finish_output(token_supply)?,
        // with tag feature
        nft_output_builder
            .clone()
            .add_feature(TagFeature::new("Hello, World!".as_bytes().to_owned())?)
            .finish_output(token_supply)?,
    ];

    println!("{outputs:#?}");

    Ok(())
}
