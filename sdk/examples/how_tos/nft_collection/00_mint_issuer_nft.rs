// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint the issuer NFT for the NFT collection.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/wallet/create_wallet.rs` example!
//!
//! Make sure that the wallet already has some funds by running the
//! `./how_tos/simple_transaction/request_funds.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example mint_issuer_nft
//! ```

use iota_sdk::{
    types::block::{
        output::{feature::MetadataFeature, NftId, Output, OutputId},
        payload::signed_transaction::TransactionId,
    },
    wallet::MintNftParams,
    Wallet,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "STRONGHOLD_PASSWORD", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    wallet.sync(None).await?;
    println!("Wallet synced!");

    // Set the stronghold password
    wallet
        .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Issue the minting transaction and wait for its inclusion
    println!("Sending NFT minting transaction...");
    let nft_mint_params = [MintNftParams::new().with_immutable_metadata(
        MetadataFeature::new([(
            "data".to_owned(),
            b"This NFT will be the issuer from the awesome NFT collection".to_vec(),
        )])
        .unwrap(),
    )];
    let transaction = dbg!(wallet.mint_nfts(nft_mint_params, None).await)?;

    wait_for_inclusion(&transaction.transaction_id, &wallet).await?;

    for (output_index, output) in transaction.payload.transaction().outputs().iter().enumerate() {
        if let Output::Nft(nft_output) = output {
            // New minted nft id is empty in the output
            if nft_output.nft_id().is_null() {
                let output_id = OutputId::new(transaction.transaction_id, output_index as u16);
                let nft_id = NftId::from(&output_id);
                println!("New minted issuer NFT id: {nft_id}");
            }
        }
    }

    Ok(())
}

async fn wait_for_inclusion(transaction_id: &TransactionId, wallet: &Wallet) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Transaction sent: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    wallet
        .wait_for_transaction_acceptance(transaction_id, None, None)
        .await?;

    println!(
        "Tx accepted: {}/transactions/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );

    Ok(())
}
