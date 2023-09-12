// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::wallet::Wallet;

impl Wallet {
    /// Checks if there is no missing account for example indexes [0, 1, 3] should panic (for now, later return error,
    /// automatically fix?) Also checks for each account if there is a gap in an address list and no address is
    /// duplicated
    pub async fn verify_integrity(&self) -> crate::wallet::Result<()> {
        log::debug!("[verify_integrity]");

        let accounts = self.accounts.read().await;

        // check that no account is missing and they're ordered
        // check that no address is missing and they're ordered
        for (account_index, account) in accounts.iter().enumerate() {
            assert_eq!(accounts[account_index].details().await.index(), &(account_index as u32));

            let account = account.details().await;
            let public_addresses = account.public_addresses();
            for (index, public_address) in public_addresses.iter().enumerate() {
                assert_eq!(public_address.key_index, index as u32);
            }
            let internal_addresses = account.internal_addresses();
            for (index, internal_address) in internal_addresses.iter().enumerate() {
                assert_eq!(internal_address.key_index, index as u32);
            }
        }

        Ok(())
    }
}
