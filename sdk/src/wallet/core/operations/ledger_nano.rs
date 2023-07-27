// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::secret::{ledger_nano::LedgerSecretManager, LedgerNanoStatus, SecretManager},
    wallet::Wallet,
};

impl Wallet<LedgerSecretManager> {
    /// Get the ledger nano status
    pub async fn get_ledger_nano_status(&self) -> crate::wallet::Result<LedgerNanoStatus> {
        Ok(self.secret_manager.read().await.get_ledger_nano_status().await)
    }
}

impl Wallet {
    /// Get the ledger nano status
    pub async fn get_ledger_nano_status(&self) -> crate::wallet::Result<LedgerNanoStatus> {
        if let SecretManager::LedgerNano(ledger) = &*self.secret_manager.read().await {
            Ok(ledger.get_ledger_nano_status().await)
        } else {
            Err(crate::client::Error::SecretManagerMismatch.into())
        }
    }
}
