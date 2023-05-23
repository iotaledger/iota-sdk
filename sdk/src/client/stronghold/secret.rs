// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The [SecretManage] implementation for [StrongholdAdapter].

use std::ops::Range;

use async_trait::async_trait;
use crypto::hashes::{blake2b::Blake2b256, Digest};
use iota_stronghold::{
    procedures::{self, Chain, KeyType, Slip10DeriveInput},
    Location,
};

use super::{
    common::{DERIVE_OUTPUT_RECORD_PATH, PRIVATE_DATA_CLIENT_PATH, SECRET_VAULT_PATH, SEED_RECORD_PATH},
    StrongholdAdapter,
};
use crate::{
    client::{
        constants::HD_WALLET_TYPE,
        secret::{types::Mnemonic, GenerateAddressOptions, SecretManage},
        stronghold::Error,
    },
    types::block::{
        address::{Address, Ed25519Address},
        signature::Ed25519Signature,
    },
};

#[async_trait]
impl SecretManage for StrongholdAdapter {
    type Error = Error;

    async fn generate_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: Option<GenerateAddressOptions>,
    ) -> Result<Vec<Address>, Self::Error> {
        // Prevent the method from being invoked when the key has been cleared from the memory. Do note that Stronghold
        // only asks for a key for reading / writing a snapshot, so without our cached key this method is invocable, but
        // it doesn't make sense when it comes to our user (signing transactions / generating addresses without a key).
        // Thus, we put an extra guard here to prevent this methods from being invoked when our cached key has
        // been cleared.
        if !self.is_key_available().await {
            return Err(Error::KeyCleared);
        }

        // Stronghold arguments.
        let seed_location = Slip10DeriveInput::Seed(Location::generic(SECRET_VAULT_PATH, SEED_RECORD_PATH));

        // Addresses to return.
        let mut addresses = Vec::new();
        let internal = options.map(|o| o.internal).unwrap_or_default();

        for address_index in address_indexes {
            let bip_path = vec![HD_WALLET_TYPE, coin_type, account_index, internal as u32, address_index];
            let chain = Chain::from_u32_hardened(bip_path);

            let derive_location = Location::generic(
                SECRET_VAULT_PATH,
                [
                    DERIVE_OUTPUT_RECORD_PATH,
                    &chain.segments().iter().flat_map(|seg| seg.bs()).collect::<Vec<u8>>(),
                ]
                .concat(),
            );

            // Derive a SLIP-10 private key in the vault.
            self.slip10_derive(chain, seed_location.clone(), derive_location.clone())
                .await?;

            // Get the Ed25519 public key from the derived SLIP-10 private key in the vault.
            let public_key = self.ed25519_public_key(derive_location.clone()).await?;

            // Cleanup location afterwards
            self.stronghold
                .lock()
                .await
                .get_client(PRIVATE_DATA_CLIENT_PATH)?
                .vault(SECRET_VAULT_PATH)
                .delete_secret(derive_location.record_path())?;

            // Hash the public key to get the address.
            let hash = Blake2b256::digest(public_key);

            // Convert the hash into [Address].
            let address = Address::Ed25519(Ed25519Address::new(hash.into()));

            // Collect it.
            addresses.push(address);
        }

        Ok(addresses)
    }

    async fn sign_ed25519(&self, msg: &[u8], chain: &Chain) -> Result<Ed25519Signature, Self::Error> {
        // Prevent the method from being invoked when the key has been cleared from the memory. Do note that Stronghold
        // only asks for a key for reading / writing a snapshot, so without our cached key this method is invocable, but
        // it doesn't make sense when it comes to our user (signing transactions / generating addresses without a key).
        // Thus, we put an extra guard here to prevent this methods from being invoked when our cached key has
        // been cleared.
        if !self.is_key_available().await {
            return Err(Error::KeyCleared);
        }

        // Stronghold arguments.
        let seed_location = Slip10DeriveInput::Seed(Location::generic(SECRET_VAULT_PATH, SEED_RECORD_PATH));

        let derive_location = Location::generic(
            SECRET_VAULT_PATH,
            [
                DERIVE_OUTPUT_RECORD_PATH,
                &chain.segments().iter().flat_map(|seg| seg.bs()).collect::<Vec<u8>>(),
            ]
            .concat(),
        );

        // Derive a SLIP-10 private key in the vault.
        self.slip10_derive(chain.clone(), seed_location, derive_location.clone())
            .await?;

        // Get the Ed25519 public key from the derived SLIP-10 private key in the vault.
        let public_key = self.ed25519_public_key(derive_location.clone()).await?;
        let signature = self.ed25519_sign(derive_location.clone(), msg).await?;

        // Cleanup location afterwards
        self.stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .vault(SECRET_VAULT_PATH)
            .delete_secret(derive_location.record_path())?;

        Ok(Ed25519Signature::new(public_key, signature))
    }
}

/// Private methods for the secret manager implementation.
impl StrongholdAdapter {
    /// Execute [Procedure::BIP39Recover] in Stronghold to put a mnemonic into the Stronghold vault.
    async fn bip39_recover(
        &self,
        mnemonic: &Mnemonic,
        passphrase: Option<String>,
        output: Location,
    ) -> Result<(), Error> {
        self.stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .execute_procedure(procedures::BIP39Recover {
                mnemonic: mnemonic.as_str().to_owned(),
                passphrase,
                output,
            })?;

        Ok(())
    }

    /// Execute [Procedure::SLIP10Derive] in Stronghold to derive a SLIP-10 private key in the Stronghold vault.
    async fn slip10_derive(&self, chain: Chain, input: Slip10DeriveInput, output: Location) -> Result<(), Error> {
        if let Err(err) = self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .execute_procedure(procedures::Slip10Derive { chain, input, output })
        {
            match err {
                iota_stronghold::procedures::ProcedureError::Engine(ref e) => {
                    // Custom error for missing vault error: https://github.com/iotaledger/stronghold.rs/blob/7f0a2e0637394595e953f9071fa74b1d160f51ec/client/src/types/error.rs#L170
                    if e.to_string().contains("does not exist") {
                        // Actually the seed, derived from the mnemonic, is not stored.
                        return Err(Error::MnemonicMissing);
                    } else {
                        return Err(err.into());
                    }
                }
                _ => {
                    return Err(err.into());
                }
            }
        };

        Ok(())
    }

    /// Execute [Procedure::Ed25519PublicKey] in Stronghold to get an Ed25519 public key from the SLIP-10 private key
    /// located in `private_key`.
    async fn ed25519_public_key(&self, private_key: Location) -> Result<[u8; 32], Error> {
        Ok(self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .execute_procedure(procedures::PublicKey {
                ty: KeyType::Ed25519,
                private_key,
            })?)
    }

    /// Execute [Procedure::Ed25519Sign] in Stronghold to sign `msg` with `private_key` stored in the Stronghold vault.
    async fn ed25519_sign(&self, private_key: Location, msg: &[u8]) -> Result<[u8; 64], Error> {
        Ok(self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .execute_procedure(procedures::Ed25519Sign {
                private_key,
                msg: msg.to_vec(),
            })?)
    }

    /// Store a mnemonic into the Stronghold vault.
    pub async fn store_mnemonic<T>(&self, mnemonic: T) -> Result<(), Error>
    where
        T: TryInto<Mnemonic, Error = crate::client::stronghold::Error> + Send,
    {
        self.store_mnemonic_t(mnemonic.try_into()?).await?;
        Ok(())
    }

    /// Store a mnemonic into the Stronghold vault.
    pub async fn store_mnemonic_t(&self, mnemonic: Mnemonic) -> Result<(), Error> {
        // The key needs to be supplied first.
        if self.key_provider.lock().await.is_none() {
            return Err(Error::KeyCleared);
        };

        // Stronghold arguments.
        let output = Location::generic(SECRET_VAULT_PATH, SEED_RECORD_PATH);

        // We need to check if there has been a mnemonic stored in Stronghold or not to prevent overwriting it.
        if self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .record_exists(&output)?
        {
            return Err(Error::MnemonicAlreadyStored);
        }

        // Execute the BIP-39 recovery procedure to put it into the vault (in memory).
        self.bip39_recover(&mnemonic, None, output).await?;

        // Persist Stronghold to the disk
        self.write_stronghold_snapshot(None).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::client::constants::IOTA_COIN_TYPE;

    #[tokio::test]
    async fn test_address_generation() {
        let stronghold_path = "test_address_generation.stronghold";
        // Remove potential old stronghold file
        std::fs::remove_file(stronghold_path).ok();
        let mnemonic = String::from(
            "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally",
        );
        let stronghold_adapter = StrongholdAdapter::builder()
            .password("drowssap")
            .build(stronghold_path)
            .unwrap();

        stronghold_adapter.store_mnemonic(mnemonic).await.unwrap();

        // The snapshot should have been on the disk now.
        assert!(Path::new(stronghold_path).exists());

        let addresses = stronghold_adapter
            .generate_addresses(IOTA_COIN_TYPE, 0, 0..1, None)
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32("atoi"),
            "atoi1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluehe53e".to_string()
        );

        // Remove garbage after test, but don't care about the result
        std::fs::remove_file(stronghold_path).ok();
    }

    #[tokio::test]
    async fn test_key_cleared() {
        let stronghold_path = "test_key_cleared.stronghold";
        // Remove potential old stronghold file
        std::fs::remove_file(stronghold_path).ok();
        let mnemonic = String::from(
            "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally",
        );
        let stronghold_adapter = StrongholdAdapter::builder()
            .password("drowssap")
            .build(stronghold_path)
            .unwrap();

        stronghold_adapter.store_mnemonic(mnemonic).await.unwrap();

        // The snapshot should have been on the disk now.
        assert!(Path::new(stronghold_path).exists());

        stronghold_adapter.clear_key().await;

        // Address generation returns an error when the key is cleared.
        assert!(
            stronghold_adapter
                .generate_addresses(IOTA_COIN_TYPE, 0, 0..1, None,)
                .await
                .is_err()
        );

        stronghold_adapter.set_password("drowssap").await.unwrap();

        // After setting the correct password it works again.
        let addresses = stronghold_adapter
            .generate_addresses(IOTA_COIN_TYPE, 0, 0..1, None)
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32("atoi"),
            "atoi1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluehe53e".to_string()
        );

        // Remove garbage after test, but don't care about the result
        std::fs::remove_file(stronghold_path).ok();
    }
}
