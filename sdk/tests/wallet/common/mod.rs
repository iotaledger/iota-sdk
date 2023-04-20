// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::redundant_pub_crate)]

mod constants;

use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        request_funds_from_faucet,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
        Client,
    },
    wallet::{Account, ClientOptions, Result, Wallet},
};

pub use self::constants::*;

/// It creates a new wallet with a mnemonic secret manager, a client options object,
/// SHIMMER_COIN_TYPE, and a storage path
///
/// Arguments:
///
/// * `storage_path`: The path to the directory where the wallet will store its data.
/// * `mnemonic`: The mnemonic phrase that you want to use to generate the account. Defaults to a random one.
/// * `node`: The node to connect to. Defaults to `constants::NODE_LOCAL`
///
/// Returns:
///
/// An Wallet
#[allow(dead_code, unused_variables)]
pub(crate) async fn make_wallet(storage_path: &str, mnemonic: Option<&str>, node: Option<&str>) -> Result<Wallet> {
    let client_options = ClientOptions::new().with_node(node.unwrap_or(NODE_LOCAL))?;
    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(mnemonic.unwrap_or(&Client::generate_mnemonic().unwrap()))?;

    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_coin_type(SHIMMER_COIN_TYPE);
    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }

    wallet_builder.finish().await
}

/// Create `amount` new accounts, request funds from the faucet and sync the accounts afterwards until the faucet output
/// is available. Returns the new accounts.
#[allow(dead_code)]
pub(crate) async fn create_accounts_with_funds(wallet: &Wallet, amount: usize) -> Result<Vec<Account>> {
    let mut new_accounts = Vec::new();
    'accounts: for _ in 0..amount {
        let account = wallet.create_account().finish().await?;
        request_funds_from_faucet(FAUCET_URL, &account.addresses().await?[0].address().to_string()).await?;

        // Continue only after funds are received
        for _ in 0..30 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            let balance = account.sync(None).await?;
            if balance.base_coin().available() > 0 {
                new_accounts.push(account);
                continue 'accounts;
            }
        }
        panic!("Faucet no longer wants to hand over coins");
    }

    Ok(new_accounts)
}

#[allow(dead_code)]
pub(crate) fn setup(path: &str) -> Result<()> {
    std::fs::remove_dir_all(path).ok();
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn tear_down(path: &str) -> Result<()> {
    std::fs::remove_dir_all(path).ok();
    Ok(())
}
