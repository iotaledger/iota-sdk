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

    #[allow(clippy::single_element_loop)]
    for var in ["NODE_URL"] {
        std::env::var(var).unwrap_or_else(|_| panic!(".env variable '{var}' is undefined, see .env.example"));
    }

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
    let nft_output = NftOutputBuilder::new_with_minimum_amount(storage_score_params, NftId::null())
        .add_unlock_condition(AddressUnlockCondition::new(address.clone()))
        .add_feature(SenderFeature::new(address.clone()))
        .add_feature(MetadataFeature::new(std::collections::BTreeMap::from_iter(vec![(
            "mutable".as_bytes().to_vec(),
            MUTABLE_METADATA.as_bytes().to_vec(),
        )]))?)
        .add_feature(TagFeature::new(TAG)?)
        .add_immutable_feature(IssuerFeature::new(address))
        .add_immutable_feature(MetadataFeature::new(std::collections::BTreeMap::from_iter(vec![(
            "IRC27".as_bytes().to_vec(),
            tip_27_immutable_metadata.as_bytes().to_vec(),
        )]))?)
        .finish_output()?;

    println!("{nft_output:#?}");

    Ok(())
}
