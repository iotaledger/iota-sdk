// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint a native token.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --example mint_nft --release`

use iota_sdk::{
    types::block::output::{
        feature::{IssuerFeature, SenderFeature},
        unlock_condition::AddressUnlockCondition,
        NftId, NftOutputBuilder,
    },
    wallet::{MintNftParams, Result},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let nft_options = [MintNftParams::new()
        .try_with_address("rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu")?
        .try_with_sender("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")?
        .with_metadata(b"some NFT metadata".to_vec())
        .with_tag(b"some NFT tag".to_vec())
        .try_with_issuer("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")?
        .with_immutable_metadata(b"some NFT immutable metadata".to_vec())];

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
    let account_address = &account.addresses().await?[0];
    let sender_address = account_address.address();
    let token_supply = account.client().get_token_supply().await?;
    let outputs = [NftOutputBuilder::new_with_amount(1_000_000, NftId::null())
        .add_unlock_condition(AddressUnlockCondition::new(sender_address))
        .add_feature(SenderFeature::new(sender_address))
        .add_immutable_feature(IssuerFeature::new(sender_address))
        .finish_output(token_supply)?];

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
