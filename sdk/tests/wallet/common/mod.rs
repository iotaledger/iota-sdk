// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod constants;

use crypto::keys::{bip39::Mnemonic, bip44::Bip44};
use iota_sdk::{
    client::{
        constants::SHIMMER_COIN_TYPE,
        request_funds_from_faucet,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
        Client,
    },
    wallet::{ClientOptions, Wallet},
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
/// A Wallet
#[allow(dead_code, unused_variables)]
pub(crate) async fn make_wallet(
    storage_path: &str,
    mnemonic: Option<Mnemonic>,
    node: Option<&str>,
) -> Result<Wallet, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::new().with_node(node.unwrap_or(NODE_LOCAL))?;
    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(mnemonic.unwrap_or_else(|| Client::generate_mnemonic().unwrap()))?;

    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::builder()
        .with_secret_manager(SecretManager::Mnemonic(secret_manager))
        .with_client_options(client_options)
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE));

    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }

    Ok(wallet_builder.finish().await?)
}

#[allow(dead_code, unused_variables)]
#[cfg(feature = "ledger_nano")]
pub(crate) async fn make_ledger_nano_wallet(
    storage_path: &str,
    node: Option<&str>,
) -> Result<Wallet, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::new().with_node(node.unwrap_or(NODE_LOCAL))?;
    let mut secret_manager = iota_sdk::client::secret::ledger_nano::LedgerSecretManager::new(true);
    secret_manager.non_interactive = true;

    #[allow(unused_mut)]
    let mut wallet_builder = Wallet::builder()
        .with_secret_manager(SecretManager::LedgerNano(secret_manager))
        .with_client_options(client_options)
        .with_bip_path(Bip44::new(SHIMMER_COIN_TYPE));
    #[cfg(feature = "storage")]
    {
        wallet_builder = wallet_builder.with_storage_path(storage_path);
    }

    Ok(wallet_builder.finish().await?)
}

/// Request funds from the faucet and sync the wallet.
#[allow(dead_code)]
pub(crate) async fn request_funds(wallet: &Wallet) -> Result<(), Box<dyn std::error::Error>> {
    request_funds_from_faucet(FAUCET_URL, &wallet.address().await).await?;

    // Continue only after funds are received
    for _ in 0..30 {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let balance = wallet.sync(None).await?;
        if balance.base_coin().available() > 0 {
            return Ok(());
        }
    }
    panic!("Faucet no longer wants to hand over coins");
}

#[allow(dead_code)]
pub(crate) fn setup(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_dir_all(path).ok();
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn tear_down(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_dir_all(path).ok();
    Ok(())
}
