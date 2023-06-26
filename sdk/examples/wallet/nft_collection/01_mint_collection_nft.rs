// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint some collection NFTs with issuer feature.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example.
//!
//! You have to provide the ISSUER NFT ID that you created by running the
//! `mint_issuer_nft` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example mint_collection_nft <ISSUER NFT ID>
//! ```

use std::env;

use iota_sdk::{
    types::block::{
        address::{Bech32Address, NftAddress},
        output::NftId,
        payload::transaction::TransactionId,
    },
    wallet::{Account, MintNftParams, Result},
    Wallet,
};

// The NFT collection size
const NFT_COLLECTION_SIZE: usize = 150;
// Mint NFTs in chunks since the transaction size is limited
const NUM_NFTS_MINTED_PER_TRANSACTION: usize = 50;

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let issuer_nft_id = env::args()
        .nth(1)
        .expect("missing example argument: ISSUER NFT ID")
        .parse::<NftId>()?;

    let wallet = Wallet::builder()
        .with_storage_path(&env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    account.sync(None).await?;
    println!("Account synced!");

    // Set the stronghold password
    wallet
        .set_stronghold_password(env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let bech32_hrp = account.client().get_bech32_hrp().await?;
    // Create the metadata with another index for each
    let nft_mint_params = (0..NFT_COLLECTION_SIZE)
        .map(|index| {
            MintNftParams::new()
                .with_immutable_metadata(get_immutable_metadata(index, issuer_nft_id).as_bytes().to_vec())
                // The NFT address from the NFT we minted in mint_issuer_nft example
                .with_issuer(Bech32Address::new(bech32_hrp, NftAddress::new(issuer_nft_id)))
        })
        .collect::<Vec<_>>();

    for nft_mint_params in nft_mint_params.chunks(NUM_NFTS_MINTED_PER_TRANSACTION) {
        println!("Minting {} NFTs...", nft_mint_params.len());
        let transaction = account.mint_nfts(nft_mint_params.to_vec(), None).await?;
        wait_for_inclusion(&transaction.transaction_id, &account).await?;

        // Sync so the new outputs are available again for new transactions
        account.sync(None).await?;
    }

    // After the NFTs are minted, the issuer nft can be sent to the so called "null address"
    // 0x0000000000000000000000000000000000000000000000000000000000000000 (for smr:
    // smr1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqy8f002) or burned, to
    // prevent minting any further NFTs in the future. Sending it to the null address makes it still available to get
    // its metadata.

    Ok(())
}

fn get_immutable_metadata(index: usize, issuer_nft_id: NftId) -> String {
    // Note: we use `serde_json::from_str` to remove all unnecessary whitespace
    serde_json::from_str::<serde_json::Value>(&format!(
        r#"{{
        "standard":"IRC27",
        "version":"v1.0",
        "type":"video/mp4",
        "uri":"ipfs://wrongcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5Ywrong",
        "name":"Shimmer OG NFT #{index}",
        "description":"The Shimmer OG NFT was handed out 1337 times by the IOTA Foundation to celebrate the official launch of the Shimmer Network.",
        "issuerName":"IOTA Foundation",
        "collectionId":"{issuer_nft_id}",
        "collectionName":"Shimmer OG"
    }}"#
    )).unwrap().to_string()
}

async fn wait_for_inclusion(transaction_id: &TransactionId, account: &Account) -> Result<()> {
    println!(
        "Transaction sent: {}/transaction/{}",
        env::var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        env::var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(())
}
