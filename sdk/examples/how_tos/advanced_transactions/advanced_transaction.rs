// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send a transaction.
//!
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example advanced_transaction`

use iota_sdk::{
    types::block::{
        address::Bech32Address,
        output::{
            unlock_condition::{AddressUnlockCondition, TimelockUnlockCondition},
            BasicOutputBuilder,
        },
        slot::SlotIndex,
    },
    Wallet,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    for var in ["WALLET_DB_PATH", "EXPLORER_URL", "STRONGHOLD_PASSWORD"] {
        std::env::var(var).expect(&format!(".env variable '{var}' is undefined, see .env.example"));
    }

    // Get the wallet we generated with `create_wallet`.
    let wallet = Wallet::builder()
        .with_storage_path(&std::env::var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;

    // May want to ensure the wallet is synced before sending a transaction.
    let balance = wallet.sync(None).await?;

    if balance.base_coin().available() >= 1_000_000 {
        // Set the stronghold password
        wallet
            .set_stronghold_password(std::env::var("STRONGHOLD_PASSWORD").unwrap())
            .await?;

        // TODO better time-based UX ?
        // Create an output with amount 1_000_000 and a timelock of 1000 slots.
        let slot_index = SlotIndex::from(1000);
        let basic_output = BasicOutputBuilder::new_with_amount(1_000_000)
            .add_unlock_condition(AddressUnlockCondition::new(Bech32Address::try_from_str(
                "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
            )?))
            .add_unlock_condition(TimelockUnlockCondition::new(slot_index)?)
            .finish_output()?;

        let transaction = wallet.send_outputs(vec![basic_output], None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        wallet
            .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
            .await?;

        println!(
            "Tx accepted: {}/transactions/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            transaction.transaction_id
        );
    }

    Ok(())
}
