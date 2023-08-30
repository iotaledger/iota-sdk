// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`PrivateKeySecretManager`].

use std::ops::Range;

use async_trait::async_trait;
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::bip44::Bip44,
    signatures::{
        ed25519,
        secp256k1_ecdsa::{self, EvmAddress},
    },
};
use zeroize::{Zeroize, Zeroizing};

use super::{GenerateAddressOptions, SecretManage};
use crate::{
    client::{api::PreparedTransactionData, Error},
    types::block::{
        address::Ed25519Address, payload::transaction::TransactionPayload, signature::Ed25519Signature, unlock::Unlocks,
    },
};

/// Secret manager based on a single private key.
pub struct PrivateKeySecretManager(ed25519::SecretKey);

impl std::fmt::Debug for PrivateKeySecretManager {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PrivateKeySecretManager").finish()
    }
}

#[async_trait]
impl SecretManage for PrivateKeySecretManager {
    type Error = Error;

    async fn generate_ed25519_addresses(
        &self,
        _coin_type: u32,
        _account_index: u32,
        _address_indexes: Range<u32>,
        _options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<Ed25519Address>, Self::Error> {
        let public_key = self.0.public_key().to_bytes();

        // Hash the public key to get the address
        let result = Blake2b256::digest(public_key).try_into().map_err(|_e| {
            crate::client::Error::Blake2b256("hashing the public key while generating the address failed.")
        })?;

        crate::client::Result::Ok(vec![Ed25519Address::new(result)])
    }

    async fn generate_evm_addresses(
        &self,
        _coin_type: u32,
        _account_index: u32,
        _address_indexes: Range<u32>,
        _options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<EvmAddress>, Self::Error> {
        panic!()
    }

    async fn sign_ed25519(&self, msg: &[u8], _chain: Bip44) -> Result<Ed25519Signature, Self::Error> {
        let public_key = self.0.public_key();
        let signature = self.0.sign(msg);

        Ok(Ed25519Signature::new(public_key, signature))
    }

    async fn sign_secp256k1_ecdsa(
        &self,
        _msg: &[u8],
        _chain: Bip44,
    ) -> Result<(secp256k1_ecdsa::PublicKey, secp256k1_ecdsa::RecoverableSignature), Self::Error> {
        panic!()
    }

    async fn sign_transaction_essence(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
        time: Option<u32>,
    ) -> Result<Unlocks, Self::Error> {
        super::default_sign_transaction_essence(self, prepared_transaction_data, time).await
    }

    async fn sign_transaction(
        &self,
        prepared_transaction_data: PreparedTransactionData,
    ) -> Result<TransactionPayload, Self::Error> {
        super::default_sign_transaction(self, prepared_transaction_data).await
    }
}

impl PrivateKeySecretManager {
    /// Create a new [`PrivateKeySecretManager`] from a base 58 encoded private key.
    pub fn try_from_b58<T: AsRef<[u8]>>(b58: T) -> Result<Self, Error> {
        let mut bytes = [0u8; ed25519::SecretKey::LENGTH];

        if bs58::decode(b58.as_ref()).onto(&mut bytes).unwrap() != ed25519::SecretKey::LENGTH {
            panic!();
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
