// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint an NFT in two different ways.
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
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";
// The owner address of the first NFT we'll mint
const NFT1_OWNER_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";
// The metadata of the first minted NFT
const NFT1_METADATA: &str = "some NFT metadata";
// The immutable metadata of the first minted NFT
const NFT1_IMMUTABLE_METADATA: &str = "some NFT immutable metadata";
// The tag of the first minted NFT
const NFT1_TAG: &str = "some NFT tag";
// The base coin amount we sent with the second NFT
const NFT2_AMOUNT: u64 = 1_000_000;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder().with_storage_path(WALLET_DB_PATH).finish().await?;
    let account = wallet.get_account(ACCOUNT_ALIAS).await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;
    let nfts_before = balance.nfts();

    // We send from the first address in the account.
    let sender_address = account.addresses().await?[0].address().clone();

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Build an NFT using `NftOptions` and use the `mint_nfts` API
    let nft_options = vec![NftOptions {
        address: Some(NFT1_OWNER_ADDRESS.to_string()),
        sender: Some(sender_address.to_string()),
        metadata: Some(NFT1_METADATA.as_bytes().to_vec()),
        tag: Some(NFT1_TAG.as_bytes().to_vec()),
        issuer: Some(sender_address.to_string()),
        immutable_metadata: Some(NFT1_IMMUTABLE_METADATA.as_bytes().to_vec()),
    }];

    println!("Preparing NFT 1 minting transaction ...");

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
    println!("Minted NFT 1");

    // Build an NFT manually by using the `NftOutputBuilder`
    let token_supply = account.client().get_token_supply().await?;
    let outputs = vec![
        // address of the owner of the NFT
        NftOutputBuilder::new_with_amount(NFT2_AMOUNT, NftId::null())
            .add_unlock_condition(AddressUnlockCondition::new(*sender_address.as_ref()))
            .add_feature(SenderFeature::new(*sender_address.as_ref()))
            .add_immutable_feature(IssuerFeature::new(*sender_address.as_ref()))
            .finish_output(token_supply)?,
    ];

    println!("Preparing NFT 2 minting transaction ...");

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
    println!("Minted NFT 2");

    // Ensure the account is synced after minting.
    let balance = account.sync(None).await?;
    let nfts_after = balance.nfts();
    println!("New owned NFTs:");
    nfts_after.iter().for_each(|nft_id| {
        if !nfts_before.contains(nft_id) {
            println!("- {nft_id}");
        }
    });

    Ok(())
}
