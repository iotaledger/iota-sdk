// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send an nft.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_wallet` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --all-features --example send_nft --release
//! ```

use iota_sdk::wallet::{AddressAndNftId, Result, Wallet};

// The account alias used in this example
const ACCOUNT_ALIAS: &str = "Alice";
// The address to send the tokens to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";
// The wallet database folder
const WALLET_DB_PATH: &str = "./example.walletdb";

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Access the wallet we generated with `--example create_wallet`
    let wallet = Wallet::builder().with_storage_path(WALLET_DB_PATH).finish().await?;
    let account = wallet.get_account(ACCOUNT_ALIAS).await?;

    // May want to ensure the account is synced before sending a transaction.
    let balance = account.sync(None).await?;

    // Get the first nft
    if let Some(nft_id) = balance.nfts().first() {
        // Set the stronghold password
        wallet
            .set_stronghold_password(&std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        let outputs = vec![AddressAndNftId {
            address: RECV_ADDRESS.to_string(),
            nft_id: *nft_id,
        }];

        println!("Sending NFT '{}' to '{}'...", nft_id, RECV_ADDRESS);

        let transaction = account.send_nft(outputs, None).await?;
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
    } else {
        println!("No available NFTs");
    }

    Ok(())
}
