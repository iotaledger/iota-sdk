// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint the issuer NFT for the NFT collection.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_wallet.rs` example!
//!
//! Make sure that the wallet already has some funds by running the
//! `./how_tos/simple_transaction/request_funds.rs` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example mint_issuer_nft
//! ```

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::secret::{stronghold::StrongholdSecretManager, SecretManager},
    types::block::{
        output::{NftId, Output, OutputId},
        payload::signed_transaction::TransactionId,
    },
    wallet::{MintNftParams, Result},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Setup Stronghold secret_manager
    let secret_manager = StrongholdSecretManager::builder()
        .password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .build(std::env::var("STRONGHOLD_SNAPSHOT_PATH").unwrap())?;

    // Only required the first time, can also be generated with `manager.generate_mnemonic()?`
    let mnemonic = Mnemonic::from(std::env::var("MNEMONIC").unwrap());

    // The mnemonic only needs to be stored the first time
    secret_manager.store_mnemonic(mnemonic).await?;

    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish(&secret_manager)
        .await?;

    wallet.sync(&secret_manager, None).await?;
    println!("Wallet synced!");

    // Issue the minting transaction and wait for its inclusion
    println!("Sending NFT minting transaction...");
    let nft_mint_params = [MintNftParams::new()
        .with_immutable_metadata(b"This NFT will be the issuer from the awesome NFT collection".to_vec())];
    let transaction = dbg!(wallet.mint_nfts(&secret_manager, nft_mint_params, None).await)?;

    wait_for_inclusion(&wallet, &secret_manager, &transaction.transaction_id).await?;

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

async fn wait_for_inclusion(
    wallet: &Wallet,
    secret_manager: &StrongholdSecretManager,
    transaction_id: &TransactionId,
) -> Result<()> {
    println!(
        "Transaction sent: {}/transaction/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    // Wait for transaction to get included
    let block_id = wallet
        .reissue_transaction_until_included(secret_manager, transaction_id, None, None)
        .await?;
    println!(
        "Block included: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(())
}
