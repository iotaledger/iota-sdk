// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`MnemonicSecretManager`].

use std::ops::Range;

use async_trait::async_trait;
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::slip10::{Chain, Curve, PublicKey, SecretKey, Seed},
    signatures::secp256k1_ecdsa::EvmAddress,
};

use super::{GenerateAddressOptions, SecretManage};
use crate::{
    client::{constants::HD_WALLET_TYPE, Client, Error},
    types::block::{
        address::{Address, Ed25519Address},
        signature::Ed25519Signature,
    },
};

/// Secret manager that uses only a mnemonic.
///
/// Computation are done in-memory. A mnemonic needs to be supplied upon the creation of [`MnemonicSecretManager`].
pub struct MnemonicSecretManager(Seed);

#[async_trait]
impl SecretManage for MnemonicSecretManager {
    type Error = Error;

    async fn generate_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: Option<GenerateAddressOptions>,
    ) -> Result<Vec<Address>, Self::Error> {
        let internal = options.map(|o| o.internal).unwrap_or_default();
        let mut addresses = Vec::new();

        for address_index in address_indexes {
            let chain =
                Chain::from_u32_hardened([HD_WALLET_TYPE, coin_type, account_index, internal as u32, address_index]);

            let PublicKey::Ed25519(public_key) = self.0.derive(Curve::Ed25519, &chain)?.secret_key().public_key() else {unreachable!()};

            // Hash the public key to get the address
            let result = Blake2b256::digest(public_key).try_into().map_err(|_e| {
                crate::client::Error::Blake2b256("hashing the public key while generating the address failed.")
            });

            addresses.push(Address::Ed25519(Ed25519Address::new(result?)));
        }

        Ok(addresses)
    }

    async fn generate_evm_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: Option<GenerateAddressOptions>,
    ) -> Result<Vec<EvmAddress>, Self::Error> {
        let internal = options.map(|o| o.internal).unwrap_or_default();
        let mut addresses = Vec::new();

        for address_index in address_indexes {
            let chain =
                Chain::from_u32_hardened([HD_WALLET_TYPE, coin_type, account_index, internal as u32, address_index]);

            let PublicKey::Secp256k1Ecdsa(public_key) = self.0.derive(Curve::Secp256k1, &chain)?.secret_key().public_key() else {unreachable!()};

            addresses.push(public_key.to_evm_address());
        }

        Ok(addresses)
    }

    async fn sign_ed25519(&self, msg: &[u8], chain: &Chain) -> Result<Ed25519Signature, Self::Error> {
        // Get the private and public key for this Ed25519 address
        let SecretKey::Ed25519(private_key) = &self.0.derive(Curve::Ed25519, chain)?.secret_key() else {unreachable!()};
        let public_key = private_key.public_key();
        let signature = private_key.sign(msg).to_bytes();

        Ok(Ed25519Signature::new(public_key.to_bytes(), signature))
    }
}

impl MnemonicSecretManager {
    /// Create a new [`MnemonicSecretManager`] from a BIP-39 mnemonic in English.
    ///
    /// For more information, see <https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki>.
    pub fn try_from_mnemonic(mnemonic: &str) -> Result<Self, Error> {
        Ok(Self(Client::mnemonic_to_seed(mnemonic)?))
    }

    /// Create a new [`MnemonicSecretManager`] from a hex-encoded raw seed string.
    pub fn try_from_hex_seed(hex: &str) -> Result<Self, Error> {
        let bytes: Vec<u8> = prefix_hex::decode(hex)?;
        Ok(Self(Seed::from_bytes(&bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn address() {
        use crate::client::constants::IOTA_COIN_TYPE;

        let mnemonic = "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally";
        let secret_manager = MnemonicSecretManager::try_from_mnemonic(mnemonic).unwrap();

        let addresses = secret_manager
            .generate_addresses(IOTA_COIN_TYPE, 0, 0..1, None)
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32("atoi"),
            "atoi1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluehe53e"
        );
    }

    #[tokio::test]
    async fn seed_address() {
        use crate::client::constants::IOTA_COIN_TYPE;

        let seed = "0x256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";
        let secret_manager = MnemonicSecretManager::try_from_hex_seed(seed).unwrap();

        let addresses = secret_manager
            .generate_addresses(IOTA_COIN_TYPE, 0, 0..1, None)
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32("atoi"),
            "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r"
        );
    }
}
