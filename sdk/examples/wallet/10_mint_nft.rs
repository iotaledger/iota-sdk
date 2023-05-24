// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint an NFT in two different ways.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example mint_nft
//! ```

use std::env::var;

use iota_sdk::{
    types::block::output::{
        feature::{IssuerFeature, SenderFeature},
        unlock_condition::AddressUnlockCondition,
        NftId, NftOutputBuilder,
    },
    wallet::{MintNftParams, Result, Wallet},
};

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

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account(&var("ACCOUNT_ALIAS_1").unwrap()).await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;
    let nfts_before = balance.nfts();

    // We send from the first address in the account.
    let sender_address = *account.addresses().await?[0].address();

    // Set the stronghold password
    wallet
        .set_stronghold_password(&var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Build an NFT using `MintNftParams` and use the `mint_nfts` API
    let nft_params = vec![MintNftParams {
        address: Some(NFT1_OWNER_ADDRESS.parse()?),
        sender: Some(sender_address),
        metadata: Some(NFT1_METADATA.as_bytes().to_vec()),
        tag: Some(NFT1_TAG.as_bytes().to_vec()),
        issuer: Some(sender_address),
        immutable_metadata: Some(NFT1_IMMUTABLE_METADATA.as_bytes().to_vec()),
    }];

    println!("Sending minting transaction for NFT 1...");

    let transaction = account.mint_nfts(nft_params, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        var("EXPLORER_URL").unwrap(),
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

    println!("Sending minting transaction for NFT 2...");

    let transaction = account.send(outputs, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        var("EXPLORER_URL").unwrap(),
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
