// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint a native token.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example mint_nft --release
//! ```

use iota_sdk::{
    types::block::output::{
        feature::{IssuerFeature, SenderFeature},
        unlock_condition::AddressUnlockCondition,
        NftId, NftOutputBuilder,
    },
    wallet::{NftOptions, Result, Wallet},
};

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The base token amount sent with the NFT
const AMOUNT: u64 = 1_000_000;
// The NFT immutable metadata feature
const IMMUTABLE_METADATA: &str = "some NFT immutable metadata";
// The NFT metadata feature
const METADATA: &str = "some NFT metadata";
// The minting address of the NFT
const MINT_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";
// The NFT sender feature
const SENDER: &str = "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy";
// The NFT tag feature
const TAG: &str = "some NFT tag";
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder().with_storage_path(WALLET_DB_PATH).finish().await?;
    let account = wallet.get_account(ACCOUNT_ALIAS).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let nft_options = vec![NftOptions {
        address: Some(MINT_ADDRESS.to_string()),
        sender: Some(SENDER.to_string()),
        metadata: Some(METADATA.as_bytes().to_vec()),
        tag: Some(TAG.as_bytes().to_vec()),
        issuer: Some(SENDER.to_string()),
        immutable_metadata: Some(IMMUTABLE_METADATA.as_bytes().to_vec()),
    }];

    println!("Preparing minting transaction ...");

    let transaction = account.mint_nfts(nft_options, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    // Build nft output manually
    let sender_address = account.addresses().await?[0].address().clone();
    let token_supply = account.client().get_token_supply().await?;
    let outputs = vec![
        // address of the owner of the NFT
        NftOutputBuilder::new_with_amount(AMOUNT, NftId::null())
            .add_unlock_condition(AddressUnlockCondition::new(*sender_address.as_ref()))
            .add_feature(SenderFeature::new(*sender_address.as_ref()))
            .add_immutable_feature(IssuerFeature::new(*sender_address.as_ref()))
            .finish_output(token_supply)?,
    ];

    let transaction = account.send(outputs, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;

    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );

    // Ensure the account is synced after minting.
    account.sync(None).await?;

    Ok(())
}
