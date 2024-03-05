// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{
        secret::{GenerateAddressOptions, SecretManage, SecretManager},
        ClientError,
    },
    types::block::address::Ed25519Address,
    wallet::{Wallet, WalletError},
};
#[cfg(all(feature = "events", feature = "ledger_nano"))]
use crate::{
    types::block::address::ToBech32Ext,
    wallet::events::types::{AddressData, WalletEvent},
};

impl Wallet {
    /// Generate an address without storing it
    /// ```ignore
    /// let public_addresses = wallet
    ///     .generate_ed25519_address(None)
    ///     .await?;
    /// ```
    pub async fn generate_ed25519_address(
        &self,
        account_index: u32,
        address_index: u32,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Ed25519Address, WalletError> {
        // TODO: not sure yet whether we also should allow this method to generate addresses for different bip
        // paths.
        let coin_type = self.bip_path().await.ok_or(WalletError::MissingBipPath)?.coin_type;

        let address = match &*self.secret_manager.read().await {
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(ledger_nano) => {
                // If we don't sync, then we want to display the prompt on the ledger with the address. But the user
                // needs to have it visible on the computer first, so we need to generate it without the
                // prompt first
                let options = options.into();
                #[cfg(feature = "events")]
                if options.as_ref().map_or(false, |o| o.ledger_nano_prompt) {
                    let changed_options = options.map(|mut options| {
                        // Change options so ledger will not show the prompt the first time
                        options.ledger_nano_prompt = false;
                        options
                    });
                    // Generate without prompt to be able to display it
                    let address = ledger_nano
                        .generate_ed25519_addresses(
                            coin_type,
                            account_index,
                            address_index..address_index + 1,
                            changed_options,
                        )
                        .await?;

                    let bech32_hrp = self.bech32_hrp().await;

                    self.emit(WalletEvent::LedgerAddressGeneration(AddressData {
                        address: address[0].to_bech32(bech32_hrp),
                    }))
                    .await;
                }
                // Generate with prompt so the user can verify
                ledger_nano
                    .generate_ed25519_addresses(coin_type, account_index, address_index..address_index + 1, options)
                    .await?
            }
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(stronghold) => {
                stronghold
                    .generate_ed25519_addresses(coin_type, account_index, address_index..address_index + 1, options)
                    .await?
            }
            SecretManager::Mnemonic(mnemonic) => {
                mnemonic
                    .generate_ed25519_addresses(coin_type, account_index, address_index..address_index + 1, options)
                    .await?
            }
            #[cfg(feature = "private_key_secret_manager")]
            SecretManager::PrivateKey(private_key) => {
                private_key
                    .generate_ed25519_addresses(coin_type, account_index, address_index..address_index + 1, options)
                    .await?
            }
            SecretManager::Placeholder => return Err(ClientError::PlaceholderSecretManager.into()),
        };

        Ok(*address.first().ok_or(WalletError::MissingParameter("address"))?)
    }
}
