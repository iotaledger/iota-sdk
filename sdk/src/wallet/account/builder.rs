// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use tokio::sync::RwLock;

use crate::{
    client::secret::{SecretManage, SecretManager},
    types::block::address::{Address, Bech32Address},
    wallet::{
        account::{types::AccountAddress, Account, AccountDetails},
        Error, Wallet,
    },
};

/// The AccountBuilder
pub struct AccountBuilder {
    addresses: Option<Vec<AccountAddress>>,
    alias: Option<String>,
    bech32_hrp: Option<String>,
    wallet: Wallet,
}

impl AccountBuilder {
    /// Create an IOTA client builder
    pub fn new(wallet: Wallet) -> Self {
        Self {
            addresses: None,
            alias: None,
            bech32_hrp: None,
            wallet,
        }
    }

    /// Set the addresses, should only be used for accounts with an offline counterpart account from which the addresses
    /// were exported
    pub fn with_addresses(mut self, addresses: impl Into<Option<Vec<AccountAddress>>>) -> Self {
        self.addresses = addresses.into();
        self
    }

    /// Set the alias
    pub fn with_alias(mut self, alias: impl Into<Option<String>>) -> Self {
        self.alias = alias.into();
        self
    }

    /// Set the bech32 HRP
    pub fn with_bech32_hrp(mut self, bech32_hrp: impl Into<Option<String>>) -> Self {
        self.bech32_hrp = bech32_hrp.into();
        self
    }

    /// Build the Account and add it to the accounts from Wallet
    /// Also generates the first address of the account and if it's not the first account, the address for the first
    /// account will also be generated and compared, so no accounts get generated with different seeds
    pub async fn finish(&mut self) -> crate::wallet::Result<Account> {
        let mut accounts = self.wallet.accounts.write().await;
        let account_index = accounts.len() as u32;
        // If no alias is provided, the account index will be set as alias
        let account_alias = self.alias.clone().unwrap_or_else(|| account_index.to_string());
        log::debug!(
            "[ACCOUNT BUILDER] creating new account {} with index {}",
            account_alias,
            account_index
        );

        let coin_type = self.wallet.coin_type.load(core::sync::atomic::Ordering::Relaxed);

        // Check that the alias isn't already used for another account and that the coin type is the same for new and
        // existing accounts
        for account in accounts.iter() {
            let account = account.details().await;
            let existing_coin_type = account.coin_type;
            if existing_coin_type != coin_type {
                return Err(Error::InvalidCoinType {
                    new_coin_type: coin_type,
                    existing_coin_type,
                });
            }
            if account.alias().to_lowercase() == account_alias.to_lowercase() {
                return Err(Error::AccountAliasAlreadyExists(account_alias));
            }
        }

        // If addresses are provided we will use them directly without the additional checks, because then we assume
        // that it's for offline signing and the secretManager can't be used
        let addresses = match &self.addresses {
            Some(addresses) => addresses.clone(),
            None => {
                let mut bech32_hrp = self.bech32_hrp.clone();
                if let Some(first_account) = accounts.first() {
                    let first_account_coin_type = *first_account.details().await.coin_type();
                    // Generate the first address of the first account and compare it to the stored address from the
                    // first account to prevent having multiple accounts created with different
                    // seeds
                    let first_account_public_address =
                        get_first_public_address(&self.wallet.secret_manager, first_account_coin_type, 0).await?;
                    let first_account_addresses = first_account.public_addresses().await;

                    if first_account_public_address
                        != first_account_addresses
                            .first()
                            .ok_or(Error::FailedToGetRemainder)?
                            .address
                            .inner
                    {
                        return Err(Error::InvalidMnemonic(
                            "first account address used another seed".to_string(),
                        ));
                    }

                    // Get bech32_hrp from address
                    if let Some(address) = first_account_addresses.first() {
                        if bech32_hrp.is_none() {
                            bech32_hrp = Some(address.address.hrp.clone());
                        }
                    }
                }

                // get bech32_hrp
                let bech32_hrp = {
                    match bech32_hrp {
                        Some(bech32_hrp) => bech32_hrp,
                        None => self.wallet.client().get_bech32_hrp().await?,
                    }
                };

                let first_public_address =
                    get_first_public_address(&self.wallet.secret_manager, coin_type, account_index).await?;

                let first_public_account_address = AccountAddress {
                    address: Bech32Address::new(bech32_hrp, first_public_address)?,
                    key_index: 0,
                    internal: false,
                    used: false,
                };

                vec![first_public_account_address]
            }
        };

        let account = AccountDetails {
            index: account_index,
            coin_type,
            alias: account_alias,
            public_addresses: addresses,
            internal_addresses: Vec::new(),
            addresses_with_unspent_outputs: Vec::new(),
            outputs: HashMap::new(),
            locked_outputs: HashSet::new(),
            unspent_outputs: HashMap::new(),
            transactions: HashMap::new(),
            pending_transactions: HashSet::new(),
            incoming_transactions: HashMap::new(),
            inaccessible_incoming_transactions: HashSet::new(),
            native_token_foundries: HashMap::new(),
        };

        let account = Account::new(account, self.wallet.inner.clone()).await?;
        #[cfg(feature = "storage")]
        account.save(None).await?;
        accounts.push(account.clone());

        Ok(account)
    }
}

/// Generate the first public address of an account
pub(crate) async fn get_first_public_address(
    secret_manager: &RwLock<SecretManager>,
    coin_type: u32,
    account_index: u32,
) -> crate::wallet::Result<Address> {
    Ok(secret_manager
        .read()
        .await
        .generate_addresses(coin_type, account_index, 0..1, None)
        .await?[0])
}
