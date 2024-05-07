// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;

use crate::{
    client::{secret::SecretManagerConfig, storage::StorageAdapter, stronghold::StrongholdAdapter},
    types::{block::address::Bech32Address, TryFromDto},
    wallet::{
        self,
        core::{WalletLedger, WalletLedgerDto},
        migration::{latest_backup_migration_version, migrate, MIGRATION_VERSION_KEY},
        ClientOptions, Wallet, WalletError,
    },
};

pub(crate) const CLIENT_OPTIONS_KEY: &str = "client_options";
pub(crate) const SECRET_MANAGER_KEY: &str = "secret_manager";
pub(crate) const WALLET_LEDGER_KEY: &str = "wallet_ledger";
pub(crate) const WALLET_ADDRESS_KEY: &str = "wallet_address";
pub(crate) const WALLET_BIP_PATH_KEY: &str = "wallet_bip_path";
pub(crate) const WALLET_ALIAS_KEY: &str = "wallet_alias";

impl<S: 'static + SecretManagerConfig> Wallet<S> {
    pub(crate) async fn write_fields_to_stronghold_snapshot(
        &self,
        stronghold: &StrongholdAdapter,
    ) -> Result<(), WalletError> {
        // Set migration version
        stronghold
            .set(MIGRATION_VERSION_KEY, &latest_backup_migration_version())
            .await?;

        // Store the client options
        let client_options = self.client_options().await;
        stronghold.set(CLIENT_OPTIONS_KEY, &client_options).await?;

        // Store the secret manager
        if let Some(secret_manager_dto) = self.secret_manager.read().await.to_config() {
            stronghold.set(SECRET_MANAGER_KEY, &secret_manager_dto).await?;
        }

        // Store the wallet address
        stronghold.set(WALLET_ADDRESS_KEY, &self.address().await).await?;

        // Store the wallet bip path
        stronghold.set(WALLET_BIP_PATH_KEY, &self.bip_path().await).await?;

        // Store the wallet alias
        if let Some(alias) = self.alias().await {
            stronghold.set(WALLET_ALIAS_KEY, &alias).await?;
        }

        let serialized_wallet_ledger = serde_json::to_value(&WalletLedgerDto::from(&*self.ledger.read().await))?;
        stronghold.set(WALLET_LEDGER_KEY, &serialized_wallet_ledger).await?;

        Ok(())
    }
}

pub(crate) async fn read_fields_from_stronghold_snapshot<S: 'static + SecretManagerConfig>(
    stronghold: &StrongholdAdapter,
) -> Result<
    (
        Bech32Address,
        Option<Bip44>,
        Option<String>,
        Option<ClientOptions>,
        Option<S::Config>,
        Option<WalletLedger>,
    ),
    WalletError,
> {
    log::debug!("[read_fields_from_stronghold_snapshot]");
    migrate(stronghold).await?;

    // Get client_options
    let client_options = stronghold.get(CLIENT_OPTIONS_KEY).await?;

    // Get secret_manager
    let secret_manager = stronghold.get(SECRET_MANAGER_KEY).await?;

    // Get the wallet address
    let wallet_address = stronghold
        .get(WALLET_ADDRESS_KEY)
        .await?
        .ok_or(wallet::WalletError::Backup("missing non-optional wallet address"))?;

    // Get the wallet bip path
    let wallet_bip_path = stronghold.get(WALLET_BIP_PATH_KEY).await?;

    // Get the wallet alias
    let wallet_alias = stronghold.get(WALLET_ALIAS_KEY).await?;

    // Get wallet ledger
    let wallet_ledger = stronghold
        .get::<WalletLedgerDto>(WALLET_LEDGER_KEY)
        .await?
        .map(WalletLedger::try_from_dto)
        .transpose()?;

    Ok((
        wallet_address,
        wallet_bip_path,
        wallet_alias,
        client_options,
        secret_manager,
        wallet_ledger,
    ))
}
