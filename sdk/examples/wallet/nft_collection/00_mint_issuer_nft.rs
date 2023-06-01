// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint the issuer nft.
//! Rename `.env.example` to `.env` and run 01_create_wallet.rs before.
//!
//! `cargo run --example mint_issuer_nft --release`

use iota_sdk::{
    types::block::{
        output::{NftId, Output, OutputId},
        payload::transaction::TransactionEssence,
    },
    wallet::{MintNftParams, Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    account.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let nft_options = vec![
        MintNftParams::new()
            .with_immutable_metadata(b"This NFT will be the issuer from the awesome NFT collection".to_vec()),
    ];

    let transaction = account.mint_nfts(nft_options, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Block with NFTs mint included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    let TransactionEssence::Regular(essence) = transaction.payload.essence();
    for (output_index, output) in essence.outputs().iter().enumerate() {
        if let Output::Nft(nft_output) = output {
            // New minted nft id is empty in the output
            if nft_output.nft_id().is_null() {
                let output_id = OutputId::new(transaction.transaction_id, output_index as u16)?;
                let nft_id = NftId::from(&output_id);
                println!("New minted NFT id: {nft_id}");
            }
        }
    }

    Ok(())
}
