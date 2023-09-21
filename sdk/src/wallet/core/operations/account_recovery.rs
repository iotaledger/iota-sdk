// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use instant::Instant;

use crate::{
    client::secret::SecretManage,
    wallet::{account::SyncOptions, task, Account, Wallet},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
{
    /// Find accounts with unspent outputs.
    ///
    /// Arguments:
    ///
    /// * `account_start_index`: The index of the first account to search for.
    /// * `account_gap_limit`: The number of accounts to search for, after the last account with unspent outputs.
    /// * `address_gap_limit`: The number of addresses to search for, after the last address with unspent outputs, in
    ///   each account.
    /// * `sync_options`: Optional parameter to specify the sync options. The `address_start_index` and `force_syncing`
    ///   fields will be overwritten to skip existing addresses.
    ///
    /// Returns:
    ///
    /// A vector of Account
    pub async fn recover_accounts(
        &self,
        account_start_index: u32,
        account_gap_limit: u32,
        address_gap_limit: u32,
        sync_options: Option<SyncOptions>,
    ) -> crate::wallet::Result<Vec<Account<S>>> {
        // log::debug!("[recover_accounts]");
        // let start_time = Instant::now();
        // let mut max_account_index_to_keep = None;

        // // Search for addresses in current accounts
        // for account in self.data.read().await.iter() {
        //     // If the gap limit is 0, there is no need to search for funds
        //     if address_gap_limit > 0 {
        //         account
        //             .search_addresses_with_outputs(address_gap_limit, sync_options.clone())
        //             .await?;
        //     }
        //     let account_index = *account.details().await.index();
        //     match max_account_index_to_keep {
        //         Some(max_account_index) => {
        //             if account_index > max_account_index {
        //                 max_account_index_to_keep = Some(account_index);
        //             }
        //         }
        //         None => max_account_index_to_keep = Some(account_index),
        //     }
        // }

        // // Create accounts below account_start_index, because we don't want to have gaps in the accounts, but we also
        // // don't want to sync them
        // for _ in max_account_index_to_keep.unwrap_or(0)..account_start_index {
        //     // Don't return possible errors here, because we could then still have empty accounts
        //     let _ = self.create_account().finish().await;
        // }

        // // Don't return possible errors here already, because we would then still have empty accounts
        // let new_accounts_discovery_result = self
        //     .search_new_accounts(
        //         account_gap_limit,
        //         address_gap_limit,
        //         &mut max_account_index_to_keep,
        //         sync_options.clone(),
        //     )
        //     .await;

        // // remove accounts without outputs
        // let mut new_accounts = Vec::new();
        // let mut accounts = self.data.write().await;

        // for account in accounts.iter() {
        //     let account_index = *account.details().await.index();
        //     let mut keep_account = false;

        //     if let Some(max_account_index_to_keep) = max_account_index_to_keep {
        //         if account_index <= max_account_index_to_keep {
        //             new_accounts.push((account_index, account.clone()));
        //             keep_account = true;
        //         }
        //     }

        //     if !keep_account {
        //         // accounts are stored during syncing, delete the empty accounts again
        //         #[cfg(feature = "storage")]
        //         {
        //             log::debug!("[recover_accounts] delete empty account {}", account_index);
        //             self.storage_manager.write().await.remove_account(account_index).await?;
        //         }
        //     }
        // }
        // new_accounts.sort_by_key(|(index, _acc)| *index);
        // *accounts = new_accounts.into_iter().map(|(_, acc)| acc).collect();
        // drop(accounts);

        // // Handle result after cleaning up the empty accounts
        // new_accounts_discovery_result?;

        // log::debug!("[recover_accounts] finished in {:?}", start_time.elapsed());
        // Ok(self.data.read().await.clone())
        todo!("recover the single account");
    }
}
