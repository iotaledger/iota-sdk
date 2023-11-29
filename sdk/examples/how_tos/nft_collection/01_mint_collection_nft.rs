// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint some collection NFTs with issuer feature.
//!
//! Make sure that `STRONGHOLD_SNAPSHOT_PATH` and `WALLET_DB_PATH` already exist by
//! running the `./how_tos/accounts_and_addresses/create_wallet.rs` example.
//!
//! You have to provide the ISSUER_NFT_ID that was created by first running the
//! `mint_issuer_nft` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example mint_collection_nft <ISSUER_NFT_ID>
//! ```

use crypto::keys::bip39::Mnemonic;
use iota_sdk::{
    client::secret::stronghold::StrongholdSecretManager,
    types::block::{
        address::{Bech32Address, NftAddress},
        output::{feature::Irc27Metadata, NftId},
        payload::signed_transaction::TransactionId,
    },
    wallet::{MintNftParams, Result},
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

    let issuer_nft_id = std::env::args()
        .nth(1)
        .expect("missing example argument: ISSUER_NFT_ID")
        .parse::<NftId>()?;

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

    let bech32_hrp = wallet.client().get_bech32_hrp().await?;
    let issuer = Bech32Address::new(bech32_hrp, NftAddress::new(issuer_nft_id));

    // Create the metadata with another index for each
    let nft_mint_params = (0..NFT_COLLECTION_SIZE)
        .map(|index| {
            MintNftParams::new()
                .with_immutable_metadata(get_immutable_metadata(index).to_bytes())
                // The NFT address from the NFT we minted in mint_issuer_nft example
                .with_issuer(issuer.clone())
        })
        .collect::<Vec<_>>();

    for (index, nft_mint_params) in nft_mint_params.chunks(NUM_NFTS_MINTED_PER_TRANSACTION).enumerate() {
        println!(
            "Minting {} NFTs... ({}/{})",
            nft_mint_params.len(),
            index * NUM_NFTS_MINTED_PER_TRANSACTION + nft_mint_params.len(),
            NFT_COLLECTION_SIZE
        );
        let transaction = wallet
            .mint_nfts(&secret_manager, nft_mint_params.to_vec(), None)
            .await?;
        wait_for_inclusion(&wallet, &secret_manager, &transaction.transaction_id).await?;

        // Sync so the new outputs are available again for new transactions
        wallet.sync(&secret_manager, None).await?;
    }

    // After the NFTs are minted, the issuer nft can be sent to the so called "null address"
    // 0x0000000000000000000000000000000000000000000000000000000000000000 (for smr:
    // smr1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqy8f002) or burned, to
    // prevent minting any further NFTs in the future. Sending it to the null address makes it still available to get
    // its metadata.

    Ok(())
}

fn get_immutable_metadata(index: usize) -> Irc27Metadata {
    Irc27Metadata::new(
        "video/mp4",
        "https://ipfs.io/ipfs/QmPoYcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5YjLgiT"
            .parse()
            .unwrap(),
        format!("Shimmer OG NFT #{index}"),
    )
    .with_description(
        "The Shimmer OG NFT was handed out 1337 times by the IOTA Foundation \
        to celebrate the official launch of the Shimmer Network.",
    )
    .with_issuer_name("IOTA Foundation")
    .with_collection_name("Shimmer OG")
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
