// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The [SecretManage] implementation for [StrongholdAdapter].

use core::borrow::Borrow;
use std::ops::Range;

use async_trait::async_trait;
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::{
        bip39::{Mnemonic, MnemonicRef},
        bip44::Bip44,
        slip10::Segment,
    },
    signatures::{
        ed25519,
        secp256k1_ecdsa::{self, EvmAddress},
    },
};
use instant::Duration;
use iota_stronghold::{
    procedures::{self, Curve, KeyType, Slip10DeriveInput},
    Location,
};

use super::{
    common::{DERIVE_OUTPUT_RECORD_PATH, PRIVATE_DATA_CLIENT_PATH, SECRET_VAULT_PATH, SEED_RECORD_PATH},
    StrongholdAdapter,
};
use crate::{
    client::{
        api::PreparedTransactionData,
        secret::{types::StrongholdDto, GenerateAddressOptions, SecretManage, SecretManagerConfig},
        stronghold::Error,
    },
    types::block::{address::Ed25519Address, payload::Payload, signature::Ed25519Signature, unlock::Unlocks},
};

#[async_trait]
impl SecretManage for StrongholdAdapter {
    type Error = crate::client::Error;

    async fn generate_ed25519_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<Ed25519Address>, Self::Error> {
        // Prevent the method from being invoked when the key has been cleared from the memory. Do note that Stronghold
        // only asks for a key for reading / writing a snapshot, so without our cached key this method is invocable, but
        // it doesn't make sense when it comes to our user (signing transactions / generating addresses without a key).
        // Thus, we put an extra guard here to prevent this methods from being invoked when our cached key has
        // been cleared.
        if !self.is_key_available().await {
            return Err(Error::KeyCleared.into());
        }

        // Stronghold arguments.
        let seed_location = Slip10DeriveInput::Seed(Location::generic(SECRET_VAULT_PATH, SEED_RECORD_PATH));

        // Addresses to return.
        let mut addresses = Vec::new();
        let internal = options.into().map(|o| o.internal).unwrap_or_default();

        for address_index in address_indexes {
            let chain = Bip44::new()
                .with_coin_type(coin_type)
                .with_account(account_index)
                .with_change(internal as _)
                .with_address_index(address_index);

            let derive_location = Location::generic(
                SECRET_VAULT_PATH,
                [
                    DERIVE_OUTPUT_RECORD_PATH,
                    &chain
                        .to_chain::<ed25519::SecretKey>()
                        .into_iter()
                        .flat_map(|seg| seg.ser32())
                        .collect::<Vec<u8>>(),
                ]
                .concat(),
            );

            // Derive a SLIP-10 private key in the vault.
            self.slip10_derive(Curve::Ed25519, chain, seed_location.clone(), derive_location.clone())
                .await?;

            // Get the Ed25519 public key from the derived SLIP-10 private key in the vault.
            let public_key = self.ed25519_public_key(derive_location.clone()).await?;

            // Cleanup location afterwards
            self.stronghold
                .lock()
                .await
                .get_client(PRIVATE_DATA_CLIENT_PATH)
                .map_err(Error::from)?
                .vault(SECRET_VAULT_PATH)
                .delete_secret(derive_location.record_path())
                .map_err(Error::from)?;

            // Hash the public key to get the address.
            let hash = Blake2b256::digest(public_key);

            // Convert the hash into [Address].
            let address = Ed25519Address::new(hash.into());

            // Collect it.
            addresses.push(address);
        }

        Ok(addresses)
    }

    async fn generate_evm_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<EvmAddress>, Self::Error> {
        // Prevent the method from being invoked when the key has been cleared from the memory. Do note that Stronghold
        // only asks for a key for reading / writing a snapshot, so without our cached key this method is invocable, but
        // it doesn't make sense when it comes to our user (signing transactions / generating addresses without a key).
        // Thus, we put an extra guard here to prevent this methods from being invoked when our cached key has
        // been cleared.
        if !self.is_key_available().await {
            return Err(Error::KeyCleared.into());
        }

        // Stronghold arguments.
        let seed_location = Slip10DeriveInput::Seed(Location::generic(SECRET_VAULT_PATH, SEED_RECORD_PATH));

        // Addresses to return.
        let mut addresses = Vec::new();
        let internal = options.into().map(|o| o.internal).unwrap_or_default();

        for address_index in address_indexes {
            let chain = Bip44::new()
                .with_coin_type(coin_type)
                .with_account(account_index)
                .with_change(internal as _)
                .with_address_index(address_index);

            let derive_location = Location::generic(
                SECRET_VAULT_PATH,
                [
                    DERIVE_OUTPUT_RECORD_PATH,
                    &chain
                        .to_chain::<secp256k1_ecdsa::SecretKey>()
                        .into_iter()
                        .flat_map(|seg| seg.ser32())
                        .collect::<Vec<u8>>(),
                ]
                .concat(),
            );

            // Derive a SLIP-10 private key in the vault.
            self.slip10_derive(Curve::Secp256k1, chain, seed_location.clone(), derive_location.clone())
                .await?;

            // Get the Secp256k1 public key from the derived SLIP-10 private key in the vault.
            let public_key = self.secp256k1_ecdsa_public_key(derive_location.clone()).await?;

            // Cleanup location afterwards
            self.stronghold
                .lock()
                .await
                .get_client(PRIVATE_DATA_CLIENT_PATH)
                .map_err(Error::from)?
                .vault(SECRET_VAULT_PATH)
                .delete_secret(derive_location.record_path())
                .map_err(Error::from)?;

            // Collect it.
            addresses.push(public_key.evm_address());
        }

        Ok(addresses)
    }

    async fn sign_ed25519(&self, msg: &[u8], chain: Bip44) -> Result<Ed25519Signature, Self::Error> {
        // Prevent the method from being invoked when the key has been cleared from the memory. Do note that Stronghold
        // only asks for a key for reading / writing a snapshot, so without our cached key this method is invocable, but
        // it doesn't make sense when it comes to our user (signing transactions / generating addresses without a key).
        // Thus, we put an extra guard here to prevent this methods from being invoked when our cached key has
        // been cleared.
        if !self.is_key_available().await {
            return Err(Error::KeyCleared.into());
        }

        // Stronghold arguments.
        let seed_location = Slip10DeriveInput::Seed(Location::generic(SECRET_VAULT_PATH, SEED_RECORD_PATH));

        let derive_location = Location::generic(
            SECRET_VAULT_PATH,
            [
                DERIVE_OUTPUT_RECORD_PATH,
                &chain
                    .to_chain::<ed25519::SecretKey>()
                    .into_iter()
                    .flat_map(|seg| seg.ser32())
                    .collect::<Vec<u8>>(),
            ]
            .concat(),
        );

        // Derive a SLIP-10 private key in the vault.
        self.slip10_derive(Curve::Ed25519, chain, seed_location, derive_location.clone())
            .await?;

        // Get the Ed25519 public key from the derived SLIP-10 private key in the vault.
        let public_key = self.ed25519_public_key(derive_location.clone()).await?;
        let signature = self.ed25519_sign(derive_location.clone(), msg).await?;

        // Cleanup location afterwards
        self.stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)
            .map_err(Error::from)?
            .vault(SECRET_VAULT_PATH)
            .delete_secret(derive_location.record_path())
            .map_err(Error::from)?;

        Ok(Ed25519Signature::new(public_key, signature))
    }

    async fn sign_secp256k1_ecdsa(
        &self,
        msg: &[u8],
        chain: Bip44,
    ) -> Result<(secp256k1_ecdsa::PublicKey, secp256k1_ecdsa::RecoverableSignature), Self::Error> {
        // Prevent the method from being invoked when the key has been cleared from the memory. Do note that Stronghold
        // only asks for a key for reading / writing a snapshot, so without our cached key this method is invocable, but
        // it doesn't make sense when it comes to our user (signing transactions / generating addresses without a key).
        // Thus, we put an extra guard here to prevent this methods from being invoked when our cached key has
        // been cleared.
        if !self.is_key_available().await {
            return Err(Error::KeyCleared.into());
        }

        // Stronghold arguments.
        let seed_location = Slip10DeriveInput::Seed(Location::generic(SECRET_VAULT_PATH, SEED_RECORD_PATH));

        let derive_location = Location::generic(
            SECRET_VAULT_PATH,
            [
                DERIVE_OUTPUT_RECORD_PATH,
                &chain
                    .to_chain::<secp256k1_ecdsa::SecretKey>()
                    .into_iter()
                    .flat_map(|seg| seg.ser32())
                    .collect::<Vec<u8>>(),
            ]
            .concat(),
        );

        // Derive a SLIP-10 private key in the vault.
        self.slip10_derive(Curve::Secp256k1, chain, seed_location, derive_location.clone())
            .await?;

        // Get the public key from the derived SLIP-10 private key in the vault.
        let public_key = self.secp256k1_ecdsa_public_key(derive_location.clone()).await?;
        let signature = self.secp256k1_ecdsa_sign(derive_location.clone(), msg).await?;

        // Cleanup location afterwards
        self.stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)
            .map_err(Error::from)?
            .vault(SECRET_VAULT_PATH)
            .delete_secret(derive_location.record_path())
            .map_err(Error::from)?;

        Ok((public_key, signature))
    }

    async fn sign_transaction_essence(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
        time: Option<u32>,
    ) -> Result<Unlocks, Self::Error> {
        crate::client::secret::default_sign_transaction_essence(self, prepared_transaction_data, time).await
    }

    async fn sign_transaction(
        &self,
        prepared_transaction_data: PreparedTransactionData,
    ) -> Result<Payload, Self::Error> {
        crate::client::secret::default_sign_transaction(self, prepared_transaction_data).await
    }
}

impl SecretManagerConfig for StrongholdAdapter {
    type Config = StrongholdDto;

    fn to_config(&self) -> Option<Self::Config> {
        Some(Self::Config {
            password: None,
            timeout: self.get_timeout().map(|duration| duration.as_secs()),
            snapshot_path: self.snapshot_path.clone().into_os_string().to_string_lossy().into(),
        })
    }

    fn from_config(config: &Self::Config) -> Result<Self, Self::Error> {
        let mut builder = Self::builder();

        if let Some(password) = &config.password {
            builder = builder.password(password.clone());
        }

        if let Some(timeout) = &config.timeout {
            builder = builder.timeout(Duration::from_secs(*timeout));
        }

        Ok(builder.build(&config.snapshot_path)?)
    }
}

/// Private methods for the secret manager implementation.
impl StrongholdAdapter {
    /// Execute [Procedure::BIP39Recover] in Stronghold to put a mnemonic into the Stronghold vault.
    async fn bip39_recover(
        &self,
        mnemonic: Mnemonic,
        passphrase: Option<String>,
        output: Location,
    ) -> Result<(), Error> {
        self.stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .execute_procedure(procedures::BIP39Recover {
                mnemonic,
                passphrase: passphrase.map(Into::into).unwrap_or_default(),
                output,
            })?;

        Ok(())
    }

    /// Execute [Procedure::SLIP10Derive] in Stronghold to derive a SLIP-10 private key in the Stronghold vault.
    async fn slip10_derive(
        &self,
        curve: Curve,
        chain: Bip44,
        input: Slip10DeriveInput,
        output: Location,
    ) -> Result<(), Error> {
        let chain = match curve {
            Curve::Ed25519 => chain
                .to_chain::<ed25519::SecretKey>()
                .into_iter()
                .map(Into::into)
                .collect(),
            Curve::Secp256k1 => chain.to_chain::<secp256k1_ecdsa::SecretKey>().to_vec(),
        };
        if let Err(err) = self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .execute_procedure(procedures::Slip10Derive {
                curve,
                chain,
                input,
                output,
            })
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

    /// Execute [Procedure::PublicKey] in Stronghold to get an Ed25519 public key from the SLIP-10
    /// private key located in `private_key`.
    async fn ed25519_public_key(&self, private_key: Location) -> Result<ed25519::PublicKey, Error> {
        Ok(ed25519::PublicKey::try_from_bytes(
            self.stronghold
                .lock()
                .await
                .get_client(PRIVATE_DATA_CLIENT_PATH)?
                .execute_procedure(procedures::PublicKey {
                    ty: KeyType::Ed25519,
                    private_key,
                })?
                .try_into()
                .unwrap(),
        )?)
    }

    /// Execute [Procedure::Ed25519Sign] in Stronghold to sign `msg` with `private_key` stored in the Stronghold vault.
    async fn ed25519_sign(&self, private_key: Location, msg: &[u8]) -> Result<ed25519::Signature, Error> {
        Ok(ed25519::Signature::from_bytes(
            self.stronghold
                .lock()
                .await
                .get_client(PRIVATE_DATA_CLIENT_PATH)?
                .execute_procedure(procedures::Ed25519Sign {
                    private_key,
                    msg: msg.to_vec(),
                })?,
        ))
    }

    /// Execute [Procedure::Secp256k1EcdsaSign] in Stronghold to sign `msg` with `private_key` stored in the Stronghold
    /// vault.
    async fn secp256k1_ecdsa_sign(
        &self,
        private_key: Location,
        msg: &[u8],
    ) -> Result<secp256k1_ecdsa::RecoverableSignature, Error> {
        Ok(secp256k1_ecdsa::RecoverableSignature::try_from_bytes(
            &self
                .stronghold
                .lock()
                .await
                .get_client(PRIVATE_DATA_CLIENT_PATH)?
                .execute_procedure(procedures::Secp256k1EcdsaSign {
                    private_key,
                    msg: msg.to_vec(),
                    flavor: procedures::Secp256k1EcdsaFlavor::Keccak256,
                })?,
        )?)
    }

    /// Execute [Procedure::PublicKey] in Stronghold to get a Secp256k1Ecdsa public key from the
    /// SLIP-10 private key located in `private_key`.
    async fn secp256k1_ecdsa_public_key(&self, private_key: Location) -> Result<secp256k1_ecdsa::PublicKey, Error> {
        let bytes = self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .execute_procedure(procedures::PublicKey {
                ty: KeyType::Secp256k1Ecdsa,
                private_key,
            })?;
        Ok(secp256k1_ecdsa::PublicKey::try_from_slice(&bytes)?)
    }

    /// Store a mnemonic into the Stronghold vault.
    pub async fn store_mnemonic(&self, mnemonic: impl Borrow<MnemonicRef> + Send) -> Result<(), Error> {
        // The key needs to be supplied first.
        if self.key_provider.lock().await.is_none() {
            return Err(Error::KeyCleared);
        };

        // Stronghold arguments.
        let output = Location::generic(SECRET_VAULT_PATH, SEED_RECORD_PATH);

        // Trim the mnemonic, in case it hasn't been, as otherwise the restored seed would be wrong.
        let trimmed_mnemonic = Mnemonic::from(mnemonic.borrow().trim().to_owned());

        // Check if the mnemonic is valid.
        crypto::keys::bip39::wordlist::verify(&trimmed_mnemonic, &crypto::keys::bip39::wordlist::ENGLISH)
            .map_err(|e| Error::InvalidMnemonic(format!("{e:?}")))?;

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
        self.bip39_recover(trimmed_mnemonic, None, output).await?;

        // Persist Stronghold to the disk
        self.write_stronghold_snapshot(None).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::{
        client::constants::{ETHER_COIN_TYPE, IOTA_COIN_TYPE},
        types::block::address::ToBech32Ext,
    };

    #[tokio::test]
    async fn test_ed25519_address_generation() {
        let stronghold_path = "test_ed25519_address_generation.stronghold";
        // Remove potential old stronghold file
        std::fs::remove_file(stronghold_path).ok();
        let mnemonic = Mnemonic::from(
            "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally".to_owned(),
        );
        let stronghold_adapter = StrongholdAdapter::builder()
            .password("drowssap".to_owned())
            .build(stronghold_path)
            .unwrap();

        stronghold_adapter.store_mnemonic(mnemonic).await.unwrap();

        // The snapshot should have been on the disk now.
        assert!(Path::new(stronghold_path).exists());

        let addresses = stronghold_adapter
            .generate_ed25519_addresses(IOTA_COIN_TYPE, 0, 0..1, None)
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32_unchecked("atoi"),
            "atoi1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluehe53e"
        );

        // Remove garbage after test, but don't care about the result
        std::fs::remove_file(stronghold_path).ok();
    }

    #[tokio::test]
    async fn test_evm_address_generation() {
        let stronghold_path = "test_evm_address_generation.stronghold";
        // Remove potential old stronghold file
        std::fs::remove_file(stronghold_path).ok();
        let mnemonic = Mnemonic::from(
            "endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river".to_owned(),
        );
        let stronghold_adapter = StrongholdAdapter::builder()
            .password("drowssap".to_owned())
            .build(stronghold_path)
            .unwrap();

        stronghold_adapter.store_mnemonic(mnemonic).await.unwrap();

        // The snapshot should have been on the disk now.
        assert!(Path::new(stronghold_path).exists());

        let addresses = stronghold_adapter
            .generate_evm_addresses(ETHER_COIN_TYPE, 0, 0..1, None)
            .await
            .unwrap();

        assert_eq!(
            prefix_hex::encode(addresses[0].as_ref()),
            "0xcaefde2b487ded55688765964320ff390cd87828"
        );

        // Remove garbage after test, but don't care about the result
        std::fs::remove_file(stronghold_path).ok();
    }

    #[tokio::test]
    async fn test_key_cleared() {
        let stronghold_path = "test_key_cleared.stronghold";
        // Remove potential old stronghold file
        std::fs::remove_file(stronghold_path).ok();
        let mnemonic = Mnemonic::from(
            "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally".to_owned(),
        );
        let stronghold_adapter = StrongholdAdapter::builder()
            .password("drowssap".to_owned())
            .build(stronghold_path)
            .unwrap();

        stronghold_adapter.store_mnemonic(mnemonic).await.unwrap();

        // The snapshot should have been on the disk now.
        assert!(Path::new(stronghold_path).exists());

        stronghold_adapter.clear_key().await;

        // Address generation returns an error when the key is cleared.
        assert!(
            stronghold_adapter
                .generate_ed25519_addresses(IOTA_COIN_TYPE, 0, 0..1, None,)
                .await
                .is_err()
        );

        stronghold_adapter.set_password("drowssap".to_owned()).await.unwrap();

        // After setting the correct password it works again.
        let addresses = stronghold_adapter
            .generate_ed25519_addresses(IOTA_COIN_TYPE, 0, 0..1, None)
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32_unchecked("atoi"),
            "atoi1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluehe53e"
        );

        // Remove garbage after test, but don't care about the result
        std::fs::remove_file(stronghold_path).ok();
    }
}
