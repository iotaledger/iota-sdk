// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint the 150 nfts with issuer feature.
//! Rename `.env.example` to `.env` and run 01_create_wallet.rs before.
//!
//! `cargo run --example mint_collection_nft --release`

use std::str::FromStr;

use iota_sdk::{
    types::block::{
        address::{Address, NftAddress},
        output::NftId,
    },
    wallet::{NftOptions, Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let nft_collection_size = 15;
    // Set this to the NFT id from the mint_issuer_nft example
    let issuer_nft_id = NftId::from_str("0x13c490ac052e575cffd40e170c2d46c6029b8b68cdf0e899b34cde93d2a7b28a")?;

    // Create the wallet
    let wallet = Wallet::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = wallet.get_account("Alice").await?;

    account.sync(None).await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let bech32_hrp = account.client().get_bech32_hrp().await?;
    let mut nft_options = Vec::new();

    // Create the metadata with another index for each
    for index in 0..nft_collection_size {
        nft_options.push(NftOptions {
            address: None,
            immutable_metadata: Some(format!("{{\"standard\":\"IRC27\",\"version\":\"v1.0\",\"type\":\"video/mp4\",\"uri\":\"ipfs://wrongcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5Ywrong\",\"name\":\"Shimmer OG NFT #{index}\",\"description\":\"The Shimmer OG NFT was handed out 1337 times by the IOTA Foundation to celebrate the official launch of the Shimmer Network.\",\"issuerName\":\"IOTA Foundation\",\"collectionId\":\"{issuer_nft_id}\",\"collectionName\":\"Shimmer OG\" }}").as_bytes().to_vec()),
            // The NFT address from the NFT we minted in mint_issuer_nft example
            issuer: Some(Address::Nft(NftAddress::new(issuer_nft_id)).to_bech32(bech32_hrp.clone())),
            metadata: None,
            sender: None,
            tag: None,
        });
    }

    // Mint nfts in chunks, since the transaction size is limited
    for nfts in nft_options.chunks(50) {
        let transaction = account.mint_nfts(nfts.to_vec(), None).await?;

        println!(
            "Block with chunk of NFTs mint sent: {}/transaction/{}",
            &std::env::var("EXPLORER_URL").unwrap(),
            transaction.transaction_id
        );
        // Try to get the transaction confirmed
        account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;
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
