// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use crypto::keys::bip39::Mnemonic;

use crate::{
    client::{secret::SecretManager, stronghold::StrongholdAdapter, utils::Password, ClientError},
    wallet::{Wallet, WalletError},
};

impl Wallet {
    /// Sets the Stronghold password
    pub async fn set_stronghold_password(&self, password: impl Into<Password> + Send) -> Result<(), WalletError> {
        let password = password.into();

        if let SecretManager::Stronghold(stronghold) = &mut *self.secret_manager.write().await {
            stronghold.set_password(password).await?;
            Ok(())
        } else {
            Err(ClientError::SecretManagerMismatch.into())
        }
    }

    /// Change the Stronghold password to another one and also re-encrypt the values in the loaded snapshot with it.
    pub async fn change_stronghold_password(
        &self,
        current_password: impl Into<Password> + Send,
        new_password: impl Into<Password> + Send,
    ) -> Result<(), WalletError> {
        let current_password = current_password.into();
        let new_password = new_password.into();

        if let SecretManager::Stronghold(stronghold) = &mut *self.secret_manager.write().await {
            stronghold.set_password(current_password).await?;
            stronghold.change_password(new_password).await?;
            Ok(())
        } else {
            Err(ClientError::SecretManagerMismatch.into())
        }
    }

    /// Sets the Stronghold password clear interval
    pub async fn set_stronghold_password_clear_interval(&self, timeout: Option<Duration>) -> Result<(), WalletError> {
        if let SecretManager::Stronghold(stronghold) = &mut *self.secret_manager.write().await {
            stronghold.set_timeout(timeout).await;
            Ok(())
        } else {
            Err(ClientError::SecretManagerMismatch.into())
        }
    }

    /// Stores a mnemonic into the Stronghold vault
    pub async fn store_mnemonic(&self, mnemonic: Mnemonic) -> Result<(), WalletError> {
        if let SecretManager::Stronghold(stronghold) = &mut *self.secret_manager.write().await {
            stronghold.store_mnemonic(mnemonic).await?;
            Ok(())
        } else {
            Err(ClientError::SecretManagerMismatch.into())
        }
    }

    /// Clears the Stronghold password from memory.
    pub async fn clear_stronghold_password(&self) -> Result<(), WalletError> {
        log::debug!("[clear_stronghold_password]");
        if let SecretManager::Stronghold(stronghold) = &mut *self.secret_manager.write().await {
            stronghold.clear_key().await;
            Ok(())
        } else {
            Err(ClientError::SecretManagerMismatch.into())
        }
    }

    /// Checks if the Stronghold password is available.
    pub async fn is_stronghold_password_available(&self) -> Result<bool, WalletError> {
        log::debug!("[is_stronghold_password_available]");
        if let SecretManager::Stronghold(stronghold) = &*self.secret_manager.write().await {
            Ok(stronghold.is_key_available().await)
        } else {
            Err(ClientError::SecretManagerMismatch.into())
        }
    }
}

impl Wallet<StrongholdAdapter> {
    /// Sets the Stronghold password
    pub async fn set_stronghold_password(&self, password: impl Into<Password> + Send) -> Result<(), WalletError> {
        Ok(self.secret_manager.write().await.set_password(password).await?)
    }

    /// Change the Stronghold password to another one and also re-encrypt the values in the loaded snapshot with it.
    pub async fn change_stronghold_password(
        &self,
        current_password: impl Into<Password> + Send,
        new_password: impl Into<Password> + Send,
    ) -> Result<(), WalletError> {
        let stronghold = &mut *self.secret_manager.write().await;
        stronghold.set_password(current_password).await?;
        stronghold.change_password(new_password).await?;
        Ok(())
    }

    /// Sets the Stronghold password clear interval
    pub async fn set_stronghold_password_clear_interval(&self, timeout: Option<Duration>) -> Result<(), WalletError> {
        self.secret_manager.write().await.set_timeout(timeout).await;
        Ok(())
    }

    /// Stores a mnemonic into the Stronghold vault
    pub async fn store_mnemonic(&self, mnemonic: Mnemonic) -> Result<(), WalletError> {
        Ok(self.secret_manager.write().await.store_mnemonic(mnemonic).await?)
    }

    /// Clears the Stronghold password from memory.
    pub async fn clear_stronghold_password(&self) -> Result<(), WalletError> {
        log::debug!("[clear_stronghold_password]");
        self.secret_manager.write().await.clear_key().await;
        Ok(())
    }

    /// Checks if the Stronghold password is available.
    pub async fn is_stronghold_password_available(&self) -> Result<bool, WalletError> {
        log::debug!("[is_stronghold_password_available]");
        Ok(self.secret_manager.write().await.is_key_available().await)
    }
}
