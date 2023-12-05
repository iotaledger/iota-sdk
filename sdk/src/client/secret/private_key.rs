// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`PrivateKeySecretManager`].

use async_trait::async_trait;
use crypto::signatures::ed25519;
use zeroize::{Zeroize, Zeroizing};

use crate::{
    client::{
        secret::{Generate, Sign, SignTransaction},
        Error,
    },
    types::block::{address::Ed25519Address, signature::Ed25519Signature},
};

/// Secret manager based on a single private key.
pub struct PrivateKeySecretManager(ed25519::SecretKey);

impl std::fmt::Debug for PrivateKeySecretManager {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrivateKeySecretManager").finish()
    }
}

#[async_trait]
impl Generate<ed25519::PublicKey> for PrivateKeySecretManager {
    type Options = ();

    async fn generate(&self, _options: &Self::Options) -> crate::client::Result<ed25519::PublicKey> {
        crate::client::Result::Ok(self.0.public_key())
    }
}

#[async_trait]
impl Generate<Ed25519Address> for PrivateKeySecretManager {
    type Options = ();

    async fn generate(&self, options: &Self::Options) -> crate::client::Result<Ed25519Address> {
        let public_key: ed25519::PublicKey = self.generate(options).await?;
        Ok(Ed25519Address::from_public_key_bytes(public_key.to_bytes()))
    }
}

#[async_trait]
impl Sign<Ed25519Signature> for PrivateKeySecretManager {
    type Options = ();

    async fn sign(&self, msg: &[u8], _options: &Self::Options) -> crate::client::Result<Ed25519Signature> {
        let public_key = self.0.public_key();
        let signature = self.0.sign(msg);

        Ok(Ed25519Signature::new(public_key, signature))
    }
}

impl SignTransaction for PrivateKeySecretManager {}

impl PrivateKeySecretManager {
    /// Create a new [`PrivateKeySecretManager`] from a base 58 encoded private key.
    pub fn try_from_b58<T: AsRef<[u8]>>(b58: T) -> Result<Self, Error> {
        let mut bytes = [0u8; ed25519::SecretKey::LENGTH];

        // TODO replace with a more fitting variant.
        if bs58::decode(b58.as_ref())
            .onto(&mut bytes)
            .map_err(|_| crypto::Error::PrivateKeyError)?
            != ed25519::SecretKey::LENGTH
        {
            // TODO replace with a more fitting variant.
            return Err(crypto::Error::PrivateKeyError.into());
        }

        let private_key = Self(ed25519::SecretKey::from_bytes(&bytes));

        bytes.zeroize();

        Ok(private_key)
    }

    /// Create a new [`PrivateKeySecretManager`] from an hex encoded private key.
    pub fn try_from_hex(hex: impl Into<Zeroizing<String>>) -> Result<Self, Error> {
        let mut bytes = prefix_hex::decode(hex.into())?;

        let private_key = Self(ed25519::SecretKey::from_bytes(&bytes));

        bytes.zeroize();

        Ok(private_key)
    }
}
