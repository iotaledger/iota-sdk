// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block, with custom parents, which can be used for promoting.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --example block_custom_parents
//! ```

use crypto::keys::bip44::Bip44;
use iota_sdk::{
    client::{
        constants::IOTA_COIN_TYPE,
        secret::{SecretManager, SignBlock},
        Client,
    },
    types::block::{
        core::{basic::MaxBurnedManaAmount, BasicBlockBodyBuilder, BlockHeader},
        output::AccountId,
        UnsignedBlock,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let node_url = std::env::var("NODE_URL").unwrap();
    let issuer_id = std::env::var("ISSUER_ID").unwrap().parse::<AccountId>().unwrap();

    // Create a node client.
    let client = Client::builder().with_node(&node_url)?.finish().await?;

    let secret_manager = SecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    // Use issuance as custom parents.
    let issuance = client.get_issuance().await?;
    println!("Issuance:\n{issuance:#?}");

    let protocol_params = client.get_protocol_parameters().await?;

    // Create and send the block with custom parents.
    let block = UnsignedBlock::new(
        BlockHeader::new(
            protocol_params.version(),
            protocol_params.network_id(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos() as _,
            issuance.latest_commitment.id(),
            issuance.latest_finalized_slot,
            issuer_id,
        ),
        BasicBlockBodyBuilder::new(
            issuance.strong_parents()?,
            MaxBurnedManaAmount::MinimumAmount {
                params: protocol_params.work_score_parameters(),
                reference_mana_cost: client
                    .get_account_congestion(&issuer_id, None)
                    .await?
                    .reference_mana_cost,
            },
        )
        .finish_block_body()?,
    )
    .sign_ed25519(&secret_manager, Bip44::new(IOTA_COIN_TYPE))
    .await?;

    println!("{block:#?}");

    println!(
        "Block with custom parents sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        client.block_id(&block).await?
    );

    Ok(())
}
