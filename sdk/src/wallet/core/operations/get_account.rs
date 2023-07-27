// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::secret::SecretManage,
    wallet::{
        account::{types::AccountIdentifier, Account},
        Wallet,
    },
};

impl<S: SecretManage> Wallet<S> {
    /// Get an account with an AccountIdentifier
    pub async fn get_account<I: Into<AccountIdentifier> + Send>(
        &self,
        identifier: I,
    ) -> crate::wallet::Result<Account<S>> {
        let account_id = identifier.into();
        let accounts = self.accounts.read().await;

        match &account_id {
            AccountIdentifier::Index(index) => {
                for account in accounts.iter() {
                    let account_details = account.details().await;

                    if account_details.index() == index {
                        return Ok(account.clone());
                    }
                }
            }
            AccountIdentifier::Alias(alias) => {
                for account in accounts.iter() {
                    let account_details = account.details().await;

                    if account_details.alias() == alias {
                        return Ok(account.clone());
                    }
                }
            }
        };

        Err(crate::wallet::Error::AccountNotFound(serde_json::to_string(
            &account_id,
        )?))
    }
}
