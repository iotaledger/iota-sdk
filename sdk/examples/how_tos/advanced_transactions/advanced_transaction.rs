// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send a transaction.
//! Rename `.env.example` to `.env` first.
//!
//! `cargo run --release --all-features --example advanced_transaction`

use iota_sdk::{
    client::secret::SecretManager,
    types::block::{
        address::Bech32Address,
        output::{
            unlock_condition::{AddressUnlockCondition, TimelockUnlockCondition},
            BasicOutputBuilder,
        },
        slot::SlotIndex,
    },
    wallet::{Result, Wallet},
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    // Create the wallet
    let wallet = Wallet::builder()
        .load_storage::<SecretManager>(std::env::var("WALLET_DB_PATH").unwrap())
        .await?
        .finish()
        .await?;

    // Get the account we generated with `create_account`
    let account = wallet.get_account("Alice").await?;
    // May want to ensure the account is synced before sending a transaction.
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
            .finish_output(account.client().get_token_supply().await?)?;

        let transaction = account.send_outputs(vec![basic_output], None).await?;
        println!("Transaction sent: {}", transaction.transaction_id);

        // Wait for transaction to get included
        let block_id = account
            .reissue_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;
        println!(
            "Block sent: {}/block/{}",
            std::env::var("EXPLORER_URL").unwrap(),
            block_id
        );
    }

    Ok(())
}
