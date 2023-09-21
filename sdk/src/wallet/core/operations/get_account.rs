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

        todo!("since there's only one account in the wallet, no need to have AccountIdentifier?");
        // let data = self.data.read().await;

        match &account_id {
            AccountIdentifier::Index(index) => {
                todo!("no need to iter anymore");
                // for account in data.iter() {
                //     let account_details = account.details().await;

                //     if account_details.index() == index {
                //         return Ok(account.clone());
                //     }
                // }
            }
            AccountIdentifier::Alias(alias) => {
                todo!("no need to iter anymore");
                // for account in data.iter() {
                //     let account_details = account.details().await;

                //     if account_details.alias() == alias {
                //         return Ok(account.clone());
                //     }
                // }
            }
        };

        Err(crate::wallet::Error::AccountNotFound(serde_json::to_string(
            &account_id,
        )?))
    }
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    pub async fn get_or_create_account(&self, alias: impl Into<String> + Send) -> crate::wallet::Result<Account<S>> {
        let alias = alias.into();
        match self.get_account(&alias).await {
            Err(crate::wallet::Error::AccountNotFound(_)) => self.create_account().with_alias(alias).finish().await,
            res => res,
        }
    }
}
