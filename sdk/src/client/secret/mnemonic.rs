// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`MnemonicSecretManager`].

use std::ops::Range;

use async_trait::async_trait;
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::{bip39::Mnemonic, bip44::Bip44, slip10::Seed},
    signatures::{
        ed25519,
        secp256k1_ecdsa::{self, EvmAddress},
    },
};
use zeroize::Zeroizing;

use super::{GenerateAddressOptions, SecretManage};
use crate::{
    client::{api::PreparedTransactionData, Client, Error},
    types::block::{address::Ed25519Address, payload::Payload, signature::Ed25519Signature, unlock::Unlocks},
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
impl SecretManage for MnemonicSecretManager {
    type Error = Error;

    async fn generate_ed25519_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<Ed25519Address>, Self::Error> {
        let internal = options.into().map(|o| o.internal).unwrap_or_default();

        Ok(address_indexes
            .map(|address_index| {
                let chain = Bip44::new()
                    .with_coin_type(coin_type)
                    .with_account(account_index)
                    .with_change(internal as _)
                    .with_address_index(address_index);

                let public_key = chain
                    .derive(&self.0.to_master_key::<ed25519::SecretKey>())
                    .secret_key()
                    .public_key()
                    .to_bytes();

                // Hash the public key to get the address
                let result = Blake2b256::digest(public_key).try_into().map_err(|_e| {
                    crate::client::Error::Blake2b256("hashing the public key while generating the address failed.")
                })?;

                crate::client::Result::Ok(Ed25519Address::new(result))
            })
            .collect::<Result<_, _>>()?)
    }

    async fn generate_evm_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<EvmAddress>, Self::Error> {
        let internal = options.into().map(|o| o.internal).unwrap_or_default();

        Ok(address_indexes
            .map(|address_index| {
                let chain = Bip44::new()
                    .with_coin_type(coin_type)
                    .with_account(account_index)
                    .with_change(internal as _)
                    .with_address_index(address_index);

                let public_key = chain
                    .derive(&self.0.to_master_key::<secp256k1_ecdsa::SecretKey>())
                    .secret_key()
                    .public_key();

                crate::client::Result::Ok(public_key.evm_address())
            })
            .collect::<Result<_, _>>()?)
    }

    async fn sign_ed25519(&self, msg: &[u8], chain: Bip44) -> Result<Ed25519Signature, Self::Error> {
        // Get the private and public key for this Ed25519 address
        let private_key = chain.derive(&self.0.to_master_key::<ed25519::SecretKey>()).secret_key();
        let public_key = private_key.public_key();
        let signature = private_key.sign(msg);

        Ok(Ed25519Signature::new(public_key, signature))
    }

    async fn sign_secp256k1_ecdsa(
        &self,
        msg: &[u8],
        chain: Bip44,
    ) -> Result<(secp256k1_ecdsa::PublicKey, secp256k1_ecdsa::RecoverableSignature), Self::Error> {
        // Get the private and public key for this secp256k1_ecdsa key
        let private_key = chain
            .derive(&self.0.to_master_key::<secp256k1_ecdsa::SecretKey>())
            .secret_key();
        let public_key = private_key.public_key();
        let signature = private_key.try_sign_keccak256(msg)?;

        Ok((public_key, signature))
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
    ) -> Result<Payload, Self::Error> {
        super::default_sign_transaction(self, prepared_transaction_data).await
    }
}

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::block::address::ToBech32Ext;

    #[tokio::test]
    async fn address() {
        use crate::client::constants::IOTA_COIN_TYPE;

        let mnemonic = "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally";
        let secret_manager = MnemonicSecretManager::try_from_mnemonic(mnemonic.to_owned()).unwrap();

        let addresses = secret_manager
            .generate_ed25519_addresses(IOTA_COIN_TYPE, 0, 0..1, None)
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32_unchecked("atoi"),
            "atoi1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluehe53e"
        );
    }

    #[tokio::test]
    async fn seed_address() {
        use crate::client::constants::IOTA_COIN_TYPE;

        let seed = "0x256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2".to_owned();
        let secret_manager = MnemonicSecretManager::try_from_hex_seed(seed).unwrap();

        let addresses = secret_manager
            .generate_ed25519_addresses(IOTA_COIN_TYPE, 0, 0..1, None)
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32_unchecked("atoi"),
            "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r"
        );
    }
}
