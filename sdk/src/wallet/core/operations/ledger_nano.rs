// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{
        secret::{ledger_nano::LedgerSecretManager, LedgerNanoStatus, SecretManager},
        ClientError,
    },
    wallet::{Wallet, WalletError},
};

impl Wallet<LedgerSecretManager> {
    /// Get the ledger nano status
    pub async fn get_ledger_nano_status(&self) -> Result<LedgerNanoStatus, WalletError> {
        Ok(self.secret_manager.read().await.get_ledger_nano_status().await)
    }
}

impl Wallet {
    /// Get the ledger nano status
    pub async fn get_ledger_nano_status(&self) -> Result<LedgerNanoStatus, WalletError> {
        if let SecretManager::LedgerNano(ledger) = &*self.secret_manager.read().await {
            Ok(ledger.get_ledger_nano_status().await)
        } else {
            Err(ClientError::SecretManagerMismatch.into())
        }
    }
}
