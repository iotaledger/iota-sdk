// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint some collection NFTs with issuer feature.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example and  that you have created an Issuer NFT ID
//! by running the `mint_issuer_nft` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example mint_collection_nft
//! ```

use std::{env::var, str::FromStr};

use iota_sdk::{
    types::block::{
        address::{Bech32Address, NftAddress},
        output::NftId,
        payload::transaction::TransactionId,
    },
    wallet::{Account, MintNftParams, Result, Wallet},
};

// !!! Replace with the NFT address from the NFT we minted in `mint_issuer_nft` example !!!
// const ISSUER_NFT_ID: &str = "0x13c490ac052e575cffd40e170c2d46c6029b8b68cdf0e899b34cde93d2a7b28a";
const ISSUER_NFT_ID: &str = "0xe009d73b326b5e3c3430734695421d34fc0f820005181c987962d31daf25db82";
// The NFT collection size
const NFT_COLLECTION_SIZE: usize = 150;
// Mint NFTs in chunks since the transaction size is limited
const NUM_NFTS_MINTED_PER_TRANSACTION: usize = 50;
// The location of the NFT.
const NFT_URI: &str = "ipfs://wrongcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5Ywrong";
// The name of the NFT
const NFT_NAME: &str = "Shimmer OG NFT";
// The description of the NFT
const NFT_DESCRIPTION: &str = "The Shimmer OG NFT was handed out 1337 times by the IOTA Foundation to celebrate the official launch of the Shimmer Network.";
// The issuer of the NFT
const NFT_ISSUER_NAME: &str = "IOTA Foundation";
// The name of the NFT collection
const NFT_COLLECTION_NAME: &str = "Shimmer OG";

#[tokio::main]
async fn main() -> Result<()> {
    let issuer_nft_id = if ISSUER_NFT_ID == "0x13c490ac052e575cffd40e170c2d46c6029b8b68cdf0e899b34cde93d2a7b28a" {
        panic!("You need to change the ISSUER_NFT_ID constant before you can run this example successfully!");
    } else {
        NftId::from_str(ISSUER_NFT_ID)?
    };

    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account(&var("ACCOUNT_ALIAS_1").unwrap()).await?;

    account.sync(None).await?;
    println!("Account synced!");

    // Set the stronghold password
    wallet
        .set_stronghold_password(&var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Create the metadata with another index for each
    let bech32_hrp = account.client().get_bech32_hrp().await?;
    let mut index = 0;
    let nft_mint_params = std::iter::repeat_with(|| {
        let params = MintNftParams {
            address: None,
            immutable_metadata: Some(get_immutable_metadata(index, issuer_nft_id).as_bytes().to_vec()),
            // The NFT address from the NFT we minted in mint_issuer_nft example
            issuer: Some(Bech32Address::new(bech32_hrp, NftAddress::new(issuer_nft_id))),
            metadata: None,
            sender: None,
            tag: None,
        };
        index += 1;
        params
    })
    .take(NFT_COLLECTION_SIZE)
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
    format!(
        "{{\"standard\":\"IRC27\",\"version\":\"v1.0\",\"type\":\"video/mp4\",\"uri\":\"{NFT_URI}\",\"name\":\"{NFT_NAME} #{index}\",\"description\":\"{NFT_DESCRIPTION}\",\"issuerName\":\"{NFT_ISSUER_NAME}\",\"collectionId\":\"{issuer_nft_id}\",\"collectionName\":\"{NFT_COLLECTION_NAME}\" }}"
    )
}

async fn wait_for_inclusion(transaction_id: &TransactionId, account: &Account) -> Result<()> {
    println!(
        "Transaction sent: {}/transaction/{}",
        var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(())
}
