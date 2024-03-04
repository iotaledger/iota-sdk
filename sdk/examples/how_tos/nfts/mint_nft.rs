// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint some NFTs.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example mint_nft
//! ```

use iota_sdk::{
    types::block::output::{
        feature::{Irc27Metadata, IssuerFeature, MetadataFeature, SenderFeature},
        unlock_condition::AddressUnlockCondition,
        NftId, NftOutputBuilder,
    },
    wallet::MintNftParams,
    Wallet,
};

// The owner address of the first NFT we'll mint
const NFT1_OWNER_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";
// The metadata of the first minted NFT
const NFT1_METADATA: &str = "some NFT metadata";
// The tag of the first minted NFT
const NFT1_TAG: &str = "some NFT tag";
// The base coin amount we sent with the second NFT
const NFT2_AMOUNT: u64 = 1_000_000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "STRONGHOLD_PASSWORD", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Get the wallet we generated with `create_wallet`.
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // Ensure the wallet is synced after minting.
    wallet.sync(None).await?;

    // We send from the wallet address.
    let sender_address = wallet.address().await;

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let metadata = Irc27Metadata::new(
        "video/mp4",
        "https://ipfs.io/ipfs/QmPoYcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5YjLgiT"
            .parse()
            .unwrap(),
        "Shimmer OG NFT",
    )
    .with_description("The original Shimmer NFT");

    let nft_params = [MintNftParams::new()
        .try_with_address(NFT1_OWNER_ADDRESS)?
        .try_with_sender(sender_address.clone())?
        .with_metadata(
            MetadataFeature::build()
                .with_key_value("data", NFT1_METADATA.as_bytes())
                .finish()
                .unwrap(),
        )
        .with_tag(NFT1_TAG.as_bytes().to_vec())
        .try_with_issuer(sender_address.clone())?
        .with_immutable_metadata(MetadataFeature::try_from(metadata).unwrap())];

    let transaction = wallet.mint_nfts(nft_params, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Tx accepted: {}/transactions/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction.transaction_id
    );
    println!("Minted NFT 1");

    // Build an NFT manually by using the `NftOutputBuilder`
    let outputs = [
        // address of the owner of the NFT
        NftOutputBuilder::new_with_amount(NFT2_AMOUNT, NftId::null())
            .add_unlock_condition(AddressUnlockCondition::new(sender_address.clone()))
            .add_feature(SenderFeature::new(sender_address.clone()))
            .add_immutable_feature(IssuerFeature::new(sender_address))
            .finish_output()?,
    ];

    let transaction = wallet.send_outputs(outputs, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Tx accepted: {}/transactions/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction.transaction_id
    );
    println!("Minted NFT 2");

    // Ensure the wallet is synced after minting.
    wallet.sync(None).await?;

    Ok(())
}
