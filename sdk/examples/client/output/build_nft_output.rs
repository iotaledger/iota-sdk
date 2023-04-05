// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example build_nft_output --release --features="client"

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

/// In this example we will build an NFT output
#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity but should generally not be done in production!
    dotenvy::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance.
    let client = Client::builder().with_node(&node_url)?.finish()?;

    let token_supply = client.get_token_supply().await?;
    let rent_structure = client.get_rent_structure().await?;

    let address = Address::try_from_bech32("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")?;

    // IOTA NFT Standard - IRC27: https://github.com/iotaledger/tips/blob/main/tips/TIP-0027/tip-0027.md
    let tip_27_immutable_metadata = serde_json::from_str::<serde_json::Value>(
        r#"{
    "standard": "IRC27",
    "version": "v1.0",
    "type":"image/jpeg",
    "uri":"https://mywebsite.com/my-nft-files-1.jpeg",
    "name":"My NFT #0001"
    }"#,
    )?
    .to_string()
    .as_bytes()
    .to_vec();

    // NftId needs to be null the first time
    let nft_output = NftOutputBuilder::new_with_minimum_storage_deposit(rent_structure, NftId::null())?
        .add_unlock_condition(AddressUnlockCondition::new(address))
        .add_feature(SenderFeature::new(address))
        .add_feature(MetadataFeature::new("mutable metadata".as_bytes().to_vec())?)
        .add_feature(TagFeature::new("my tag".as_bytes().to_vec())?)
        .add_immutable_feature(IssuerFeature::new(address))
        .add_immutable_feature(MetadataFeature::new(tip_27_immutable_metadata)?)
        .finish_output(token_supply)?;

    println!("{nft_output:#?}");

    Ok(())
}
