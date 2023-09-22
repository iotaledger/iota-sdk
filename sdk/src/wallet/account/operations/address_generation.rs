// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "ledger_nano")]
use crate::client::secret::{ledger_nano::LedgerSecretManager, DowncastSecretManager};
use crate::{
    client::secret::{GenerateAddressOptions, SecretManage},
    types::block::address::Bech32Address,
    wallet::{account::types::address::Bip44Address, Wallet},
};
#[cfg(all(feature = "events", feature = "ledger_nano"))]
use crate::{
    types::block::address::ToBech32Ext,
    wallet::events::types::{AddressData, WalletEvent},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    // TODO: remove

    // /// Generate addresses and stores them in the account
    // /// ```ignore
    // /// let public_addresses = account.generate_ed25519_addresses(2, None).await?;
    // /// // internal addresses are used for remainder outputs, if the RemainderValueStrategy for transactions is set
    // to ChangeAddress /// let internal_addresses = account
    // ///     .generate_ed25519_addresses(
    // ///         1,
    // ///         Some(GenerateAddressOptions {
    // ///             internal: true,
    // ///             ..Default::default()
    // ///         }),
    // ///     )
    // ///     .await?;
    // /// ```
    // pub async fn generate_ed25519_addresses(
    //     &self,
    //     amount: u32,
    //     options: impl Into<Option<GenerateAddressOptions>> + Send,
    // ) -> crate::wallet::Result<Vec<Bip44Address>> { let options = options.into().unwrap_or_default(); log::debug!(
    //   "[ADDRESS GENERATION] generating {amount} addresses, internal: {}", options.internal ); if amount == 0 { return
    //   Ok(Vec::new()); }

    //     let wallet_data = self.data().await;

    //     // get the highest index for the public or internal addresses
    //     let highest_current_index_plus_one = if options.internal {
    //         wallet_data.internal_addresses.len() as u32
    //     } else {
    //         wallet_data.public_addresses.len() as u32
    //     };

    //     // get bech32_hrp
    //     let bech32_hrp = {
    //         match wallet_data.public_addresses.first() {
    //             Some(address) => address.address.hrp,
    //             None => self.client().get_bech32_hrp().await?,
    //         }
    //     };

    //     let address_range = highest_current_index_plus_one..highest_current_index_plus_one + amount;

    //     // If we don't sync, then we want to display the prompt on the ledger with the address. But the user
    //     // needs to have it visible on the computer first, so we need to generate it without the
    //     // prompt first
    //     #[cfg(feature = "ledger_nano")]
    //     let addresses = {
    //         use crate::wallet::account::SecretManager;
    //         let secret_manager = self.inner.secret_manager.read().await;
    //         if secret_manager
    //             .downcast::<LedgerSecretManager>()
    //             .or_else(|| {
    //                 secret_manager.downcast::<SecretManager>().and_then(|s| {
    //                     if let SecretManager::LedgerNano(n) = s {
    //                         Some(n)
    //                     } else {
    //                         None
    //                     }
    //                 })
    //             })
    //             .is_some()
    //         {
    //             #[cfg(feature = "events")]
    //             let changed_options = {
    //                 // Change options so ledger will not show the prompt the first time
    //                 let mut changed_options = options;
    //                 changed_options.ledger_nano_prompt = false;
    //                 changed_options
    //             };
    //             let mut addresses = Vec::new();

    //             for address_index in address_range {
    //                 #[cfg(feature = "events")]
    //                 {
    //                     // Generate without prompt to be able to display it
    //                     let address = self
    //                         .inner
    //                         .secret_manager
    //                         .read()
    //                         .await
    //                         .generate_ed25519_addresses(
    //                             wallet_data.coin_type(),
    //                             todo!("wallet_data.index"),
    //                             address_index..address_index + 1,
    //                             Some(changed_options),
    //                         )
    //                         .await?;
    //                     self.emit(
    //                         todo!("wallet_data.index"),
    //                         WalletEvent::LedgerAddressGeneration(AddressData {
    //                             address: address[0].to_bech32(bech32_hrp),
    //                         }),
    //                     )
    //                     .await;
    //                 }
    //                 // Generate with prompt so the user can verify
    //                 let address = self
    //                     .inner
    //                     .secret_manager
    //                     .read()
    //                     .await
    //                     .generate_ed25519_addresses(
    //                         wallet_data.coin_type(),
    //                         todo!("wallet_data.index"),
    //                         address_index..address_index + 1,
    //                         Some(options),
    //                     )
    //                     .await?;
    //                 addresses.push(address[0]);
    //             }
    //             addresses
    //         } else {
    //             self.inner
    //                 .secret_manager
    //                 .read()
    //                 .await
    //                 .generate_ed25519_addresses(
    //                     wallet_data.coin_type(),
    //                     todo!("wallet_data.index"),
    //                     address_range,
    //                     Some(options),
    //                 )
    //                 .await?
    //         }
    //     };

    //     #[cfg(not(feature = "ledger_nano"))]
    //     let addresses = self
    //         .wallet
    //         .secret_manager
    //         .read()
    //         .await
    //         .generate_ed25519_addresses(
    //             account_details.coin_type,
    //             account_details.index,
    //             address_range,
    //             Some(options),
    //         )
    //         .await?;

    //     drop(wallet_data);

    //     let generate_addresses: Vec<Bip44Address> = addresses
    //         .into_iter()
    //         .enumerate()
    //         .map(|(index, address)| Bip44Address {
    //             address: Bech32Address::new(bech32_hrp, address),
    //             key_index: highest_current_index_plus_one + index as u32,
    //             internal: options.internal,
    //         })
    //         .collect();

    //     self.update_wallet_addresses(options.internal, generate_addresses.clone())
    //         .await?;

    //     Ok(generate_addresses)
    // }

    // TODO: remove
    // /// Generate an internal address and store in the account, internal addresses are used for remainder outputs
    // pub(crate) async fn generate_remainder_address(&self) -> crate::wallet::Result<Bip44Address> {
    //     let result = self
    //         .generate_ed25519_addresses(1, Some(GenerateAddressOptions::internal()))
    //         .await?
    //         .first()
    //         .ok_or(crate::wallet::Error::FailedToGetRemainder)?
    //         .clone();

    //     Ok(result)
    // }
}
