// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::secret::{DowncastSecretManager, LedgerNanoStatus},
    wallet::Wallet,
};

impl Wallet {
    /// Get the ledger nano status
    pub async fn get_ledger_nano_status(&self) -> crate::wallet::Result<LedgerNanoStatus> {
        Ok(self
            .secret_manager
            .read()
            .await
            .as_ledger_nano()?
            .get_ledger_nano_status()
            .await)
    }
}
