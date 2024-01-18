// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will build an NFT output.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example build_nft_output [ADDRESS]
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

const MUTABLE_METADATA: &str = "mutable metadata";
const TAG: &str = "my tag";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL"] {
        std::env::var(var).unwrap_or_else(|_| panic!(".env variable '{var}' is undefined, see .env.example"));
    }

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

    // IOTA NFT Standard - IRC27: https://github.com/iotaledger/tips/blob/main/tips/TIP-0027/tip-0027.md
    let tip_27_immutable_metadata = serde_json::from_str::<serde_json::Value>(
        r#"
        {
            "standard": "IRC27",
            "version": "v1.0",
            "type":"image/jpeg",
            "uri":"https://mywebsite.com/my-nft-files-1.jpeg",
            "name":"My NFT #0001"
        }"#,
    )?
    .to_string();

    // NftId needs to be null the first time
    let nft_output = NftOutputBuilder::new_with_minimum_storage_deposit(rent_structure, NftId::null())
        .add_unlock_condition(AddressUnlockCondition::new(address))
        .add_feature(SenderFeature::new(address))
        .add_feature(MetadataFeature::new(MUTABLE_METADATA)?)
        .add_feature(TagFeature::new(TAG)?)
        .add_immutable_feature(IssuerFeature::new(address))
        .add_immutable_feature(MetadataFeature::new(tip_27_immutable_metadata)?)
        .finish_output(token_supply)?;

    println!("{nft_output:#?}");

    Ok(())
}
