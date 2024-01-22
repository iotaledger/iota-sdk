// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`MnemonicSecretManager`].

use async_trait::async_trait;
use crypto::{
    keys::{bip39::Mnemonic, bip44::Bip44, slip10::Seed},
    signatures::{
        ed25519,
        secp256k1_ecdsa::{self, EvmAddress},
    },
};
use zeroize::Zeroizing;

use crate::{
    client::{
        secret::{
            types::EvmSignature, Generate, MultiKeyOptions, PublicKeyOptions, SecretManagerConfig, Sign,
            SignTransaction,
        },
        Client, Error,
    },
    types::block::{address::Ed25519Address, signature::Ed25519Signature},
};

/// Secret manager that uses only a mnemonic.
///
/// Computation are done in-memory. A mnemonic needs to be supplied upon the creation of [`MnemonicSecretManager`].
pub struct MnemonicSecretManager(Seed);

impl std::fmt::Debug for MnemonicSecretManager {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MnemonicSecretManager").finish()
    }
}

#[async_trait]
impl Generate<ed25519::PublicKey> for MnemonicSecretManager {
    type Options = PublicKeyOptions;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<ed25519::PublicKey> {
        let chain = Bip44::new(options.coin_type)
            .with_account(options.account_index)
            .with_address_index(options.address_index)
            .with_change(options.internal as _);

        let public_key = chain
            .derive(&self.0.to_master_key::<ed25519::SecretKey>())
            .secret_key()
            .public_key();

        Ok(public_key)
    }
}

#[async_trait]
impl Generate<Ed25519Address> for MnemonicSecretManager {
    type Options = PublicKeyOptions;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Ed25519Address> {
        let public_key: ed25519::PublicKey = self.generate(options).await?;
        Ok(Ed25519Address::from_public_key_bytes(public_key.to_bytes()))
    }
}

#[async_trait]
impl Generate<Vec<ed25519::PublicKey>> for MnemonicSecretManager {
    type Options = MultiKeyOptions;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Vec<ed25519::PublicKey>> {
        let mut res = Vec::with_capacity(options.address_range.len());
        for address_index in options.address_range.clone() {
            let public_key: ed25519::PublicKey = self
                .generate(
                    &PublicKeyOptions::new(options.coin_type)
                        .with_account_index(options.account_index)
                        .with_internal(options.internal)
                        .with_address_index(address_index),
                )
                .await?;
            res.push(public_key);
        }
        Ok(res)
    }
}

#[async_trait]
impl Generate<Vec<Ed25519Address>> for MnemonicSecretManager {
    type Options = MultiKeyOptions;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Vec<Ed25519Address>> {
        let public_keys: Vec<ed25519::PublicKey> = self.generate(options).await?;
        Ok(public_keys
            .into_iter()
            .map(|k| Ed25519Address::from_public_key_bytes(k.to_bytes()))
            .collect())
    }
}

#[async_trait]
impl Generate<secp256k1_ecdsa::PublicKey> for MnemonicSecretManager {
    type Options = PublicKeyOptions;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<secp256k1_ecdsa::PublicKey> {
        let chain = Bip44::new(options.coin_type)
            .with_account(options.account_index)
            .with_address_index(options.address_index)
            .with_change(options.internal as _);

        let public_key = chain
            .derive(&self.0.to_master_key::<secp256k1_ecdsa::SecretKey>())
            .secret_key()
            .public_key();

        Ok(public_key)
    }
}

#[async_trait]
impl Generate<EvmAddress> for MnemonicSecretManager {
    type Options = PublicKeyOptions;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<EvmAddress> {
        let public_key: secp256k1_ecdsa::PublicKey = self.generate(options).await?;
        Ok(public_key.evm_address())
    }
}

#[async_trait]
impl Generate<Vec<secp256k1_ecdsa::PublicKey>> for MnemonicSecretManager {
    type Options = MultiKeyOptions;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Vec<secp256k1_ecdsa::PublicKey>> {
        let mut res = Vec::with_capacity(options.address_range.len());
        for address_index in options.address_range.clone() {
            res.push(
                Generate::<secp256k1_ecdsa::PublicKey>::generate(
                    self,
                    &PublicKeyOptions::new(options.coin_type)
                        .with_account_index(options.account_index)
                        .with_internal(options.internal)
                        .with_address_index(address_index),
                )
                .await?,
            );
        }
        Ok(res)
    }
}

#[async_trait]
impl Generate<Vec<EvmAddress>> for MnemonicSecretManager {
    type Options = MultiKeyOptions;

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Vec<EvmAddress>> {
        let public_keys: Vec<secp256k1_ecdsa::PublicKey> = self.generate(options).await?;
        Ok(public_keys.into_iter().map(|k| k.evm_address()).collect())
    }
}

#[async_trait]
impl Sign<Ed25519Signature> for MnemonicSecretManager {
    type Options = Bip44;

    async fn sign(&self, msg: &[u8], chain: &Self::Options) -> crate::client::Result<Ed25519Signature> {
        // Get the private and public key for this Ed25519 address
        let private_key = chain.derive(&self.0.to_master_key::<ed25519::SecretKey>()).secret_key();
        let public_key = private_key.public_key();
        let signature = private_key.sign(msg);

        Ok(Ed25519Signature::new(public_key, signature))
    }
}

#[async_trait]
impl Sign<EvmSignature> for MnemonicSecretManager {
    type Options = Bip44;

    async fn sign(&self, msg: &[u8], chain: &Self::Options) -> crate::client::Result<EvmSignature> {
        // Get the private and public key for this secp256k1_ecdsa key
        let private_key = chain
            .derive(&self.0.to_master_key::<secp256k1_ecdsa::SecretKey>())
            .secret_key();
        let public_key = private_key.public_key();
        let signature = private_key.try_sign_keccak256(msg)?;

        Ok(EvmSignature { public_key, signature })
    }
}

impl SignTransaction for MnemonicSecretManager {}

impl MnemonicSecretManager {
    /// Create a new [`MnemonicSecretManager`] from a BIP-39 mnemonic in English.
    ///
    /// For more information, see <https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki>.
    pub fn try_from_mnemonic(mnemonic: impl Into<Mnemonic>) -> Result<Self, Error> {
        Ok(Self(Client::mnemonic_to_seed(mnemonic.into())?.into()))
    }

    /// Create a new [`MnemonicSecretManager`] from a hex-encoded raw seed string.
    pub fn try_from_hex_seed(hex: impl Into<Zeroizing<String>>) -> Result<Self, Error> {
        let hex = hex.into();
        let bytes = Zeroizing::new(prefix_hex::decode::<Vec<u8>>(hex.as_str())?);
        let seed = Seed::from_bytes(bytes.as_ref());
        Ok(Self(seed))
    }

    /// Generate a random mnemonic to use for the secret manager.
    pub fn generate_random() -> Result<Self, Error> {
        Self::try_from_mnemonic(Client::generate_mnemonic()?)
    }
}

impl SecretManagerConfig for MnemonicSecretManager {
    type Config = String;

    fn to_config(&self) -> Option<Self::Config> {
        None
    }

    fn from_config(config: &Self::Config) -> crate::client::Result<Self>
    where
        Self: Sized,
    {
        Self::try_from_mnemonic(config.as_str())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::types::block::address::ToBech32Ext;

    #[tokio::test]
    async fn address() {
        use crate::client::constants::IOTA_COIN_TYPE;

        let mnemonic = "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally";
        let secret_manager = MnemonicSecretManager::try_from_mnemonic(mnemonic.to_owned()).unwrap();

        let options = PublicKeyOptions::new(IOTA_COIN_TYPE);

        let address: Ed25519Address = secret_manager.generate(&options).await.unwrap();

        assert_eq!(
            address.to_bech32_unchecked("atoi"),
            "atoi1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluehe53e"
        );
    }

    #[tokio::test]
    async fn seed_address() {
        use crate::client::constants::IOTA_COIN_TYPE;

        let seed = "0x256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2".to_owned();
        let secret_manager = MnemonicSecretManager::try_from_hex_seed(seed).unwrap();

        let options = PublicKeyOptions::new(IOTA_COIN_TYPE);

        let address: Ed25519Address = secret_manager.generate(&options).await.unwrap();

        assert_eq!(
            address.to_bech32_unchecked("atoi"),
            "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r"
        );
    }
}
