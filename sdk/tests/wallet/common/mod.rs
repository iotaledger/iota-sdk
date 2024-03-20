// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod constants;

use crypto::keys::{bip39::Mnemonic, bip44::Bip44};
use iota_sdk::{
    client::{
        api::transaction_builder::TransactionBuilderError,
        constants::SHIMMER_COIN_TYPE,
        request_funds_from_faucet,
        secret::{mnemonic::MnemonicSecretManager, SecretManager},
        Client, ClientError,
    },
    types::block::{
        output::{feature::BlockIssuerKeySource, AccountId},
        protocol::iota_mainnet_protocol_parameters,
    },
    wallet::{ClientOptions, SendParams, SyncOptions, Wallet, WalletError},
};

pub use self::constants::{DEFAULT_MNEMONIC, FAUCET_URL, NODE_LOCAL, NODE_OTHER};

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
    mnemonic: impl Into<Option<Mnemonic>>,
    node: Option<&str>,
) -> Result<Wallet, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::new()
        .with_node(node.unwrap_or(NODE_LOCAL))?
        .with_protocol_parameters(iota_mainnet_protocol_parameters().clone());
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(
        mnemonic.into().unwrap_or_else(|| Client::generate_mnemonic().unwrap()),
    )?;

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

/// Create an implicit account creation address, request funds from the faucet to it, transition it to an account and
/// wait until enough mana is generated to send a transaction.
#[allow(dead_code)]
pub(crate) async fn request_funds(wallet: &Wallet) -> Result<(), Box<dyn std::error::Error>> {
    request_funds_from_faucet(FAUCET_URL, &wallet.implicit_account_creation_address().await?).await?;
    request_funds_from_faucet(FAUCET_URL, &wallet.address().await).await?;

    // Continue only after funds are received
    let mut attempts = 0;
    let implicit_account = loop {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        wallet
            .sync(Some(SyncOptions {
                sync_implicit_accounts: true,
                ..Default::default()
            }))
            .await?;
        if let Some(account) = wallet.ledger().await.implicit_accounts().next() {
            break account.clone();
        }
        attempts += 1;
        if attempts == 30 {
            panic!("Faucet no longer wants to hand over coins");
        }
    };

    let mut tries = 0;
    while let Err(ClientError::Node(iota_sdk::client::node_api::error::Error::NotFound(_))) = wallet
        .client()
        .get_account_congestion(&AccountId::from(&implicit_account.output_id), None)
        .await
    {
        tries += 1;
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        if tries > 100 {
            panic!("Can't get account for implicit account");
        }
    }

    let transaction = wallet
        .implicit_account_transition(
            &implicit_account.output_id,
            BlockIssuerKeySource::ImplicitAccountAddress,
        )
        .await?;

    wallet
        .wait_for_transaction_acceptance(&transaction.transaction_id, None, None)
        .await?;

    wallet.sync(None).await?;

    // Is this better than using the congestion endpoint?
    // prepare a big tx and wait the time it takes until enough mana is generated
    #[allow(unused_variables)]
    if let Err(WalletError::Client(ClientError::TransactionBuilder(TransactionBuilderError::InsufficientMana {
        found,
        required,
        slots_remaining,
    }))) = wallet
        .prepare_send(vec![SendParams::new(1_000_000, wallet.address().await)?; 10], None)
        .await
    {
        tokio::time::sleep(std::time::Duration::from_secs(
            slots_remaining as u64
                * wallet
                    .client()
                    .get_protocol_parameters()
                    .await?
                    .slot_duration_in_seconds() as u64,
        ))
        .await;
    }

    Ok(())
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
