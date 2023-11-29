// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{
        constants::IOTA_COIN_TYPE,
        secret::{DowncastSecretManager, GenerateAddressOptions, SecretManage},
    },
    types::block::address::Ed25519Address,
    wallet::Wallet,
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
    pub async fn generate_ed25519_address<S: 'static + SecretManage>(
        &self,
        secret_manager: &S,
        account_index: u32,
        address_index: u32,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> crate::wallet::Result<Ed25519Address>
    where
        crate::client::Error: From<S::Error>,
    {
        let options = options.into();
        #[cfg(feature = "ledger_nano")]
        {
            if let Ok(ledger_nano) = secret_manager.as_ledger_nano() {
                // If we don't sync, then we want to display the prompt on the ledger with the address. But the user
                // needs to have it visible on the computer first, so we need to generate it without the
                // prompt first
                if options.as_ref().map_or(false, |o| o.ledger_nano_prompt) {
                    #[cfg(feature = "events")]
                    {
                        let changed_options = options.map(|mut options| {
                            // Change options so ledger will not show the prompt the first time
                            options.ledger_nano_prompt = false;
                            options
                        });
                        // Generate without prompt to be able to display it
                        let address = ledger_nano
                            .generate_ed25519_addresses(
                                self.bip_path().await.map(|b| b.coin_type).unwrap_or(IOTA_COIN_TYPE),
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
                    let addresses = ledger_nano
                        .generate_ed25519_addresses(
                            self.bip_path().await.map(|b| b.coin_type).unwrap_or(IOTA_COIN_TYPE),
                            account_index,
                            address_index..address_index + 1,
                            options,
                        )
                        .await?;
                    return Ok(*addresses
                        .first()
                        .ok_or(crate::wallet::Error::MissingParameter("address"))?);
                }
            }
        }
        let addresses = secret_manager
            .generate_ed25519_addresses(
                self.bip_path().await.map(|b| b.coin_type).unwrap_or(IOTA_COIN_TYPE),
                account_index,
                address_index..address_index + 1,
                options,
            )
            .await
            .map_err(crate::client::Error::from)?;

        Ok(*addresses
            .first()
            .ok_or(crate::wallet::Error::MissingParameter("address"))?)
    }
}
