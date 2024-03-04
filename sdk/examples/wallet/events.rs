// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we'll demonstrate how to listen to wallet events by sending some amount of base coins to an
//! address.
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example events
//! ```

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
    },
    crypto::keys::bip44::Bip44,
    types::block::{
        address::Address,
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder},
    },
    wallet::{ClientOptions, Wallet},
};

// The amount of base coins we'll send
const SEND_AMOUNT: u64 = 1_000_000;
// The address we'll be sending coins to
const RECV_ADDRESS: &str = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["NODE_URL", "MNEMONIC", "WALLET_DB_PATH", "EXPLORER_URL"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    let client_options = ClientOptions::new().with_node(&std::env::var("NODE_URL").unwrap())?;

    let secret_manager = MnemonicSecretManager::try_from_mnemonic(std::env::var("MNEMONIC").unwrap())?;

    let wallet = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .with_client_options(client_options)
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE))
        .finish()
        .await?;

    wallet
        .listen([], move |event| {
            println!("RECEIVED AN EVENT:\n{:?}", event);
        })
        .await;

    let balance = wallet.sync(None).await?;
    println!("Balance BEFORE:\n{:#?}", balance.base_coin());

    // send transaction
    let outputs = [BasicOutputBuilder::new_with_amount(SEND_AMOUNT)
        .add_unlock_condition(AddressUnlockCondition::new(Address::try_from_bech32(RECV_ADDRESS)?))
        .finish_output()?];

    let transaction = wallet.send_outputs(outputs, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;

    println!(
        "Tx accepted: {}/transactions/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        transaction.transaction_id
    );

    let balance = wallet.sync(None).await?;
    println!("Balance AFTER:\n{:#?}", balance.base_coin());

    Ok(())
}
