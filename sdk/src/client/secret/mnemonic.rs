// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`MnemonicSecretManager`].

use std::ops::Range;

use async_trait::async_trait;
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::slip10::{Chain, Seed},
    signatures::{
        ed25519,
        secp256k1_ecdsa::{self, EvmAddress},
    },
};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use super::{GenerateAddressOptions, SecretManage};
use crate::{
    client::{constants::HD_WALLET_TYPE, Client, Error},
    types::block::{address::Ed25519Address, signature::Ed25519Signature},
};

/// Secret manager that uses only a mnemonic.
///
/// Computation are done in-memory. A mnemonic needs to be supplied upon the creation of [`MnemonicSecretManager`].
pub struct MnemonicSecretManager(Seed);

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
                let chain = Chain::from_u32_hardened([
                    HD_WALLET_TYPE,
                    coin_type,
                    account_index,
                    internal as u32,
                    address_index,
                ]);

                let public_key = self
                    .0
                    .derive::<ed25519::SecretKey>(&chain)?
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
                let chain = Chain::from_u32_hardened([HD_WALLET_TYPE, coin_type, account_index])
                    .join(Chain::from_u32([internal as u32, address_index]));

                let public_key = self
                    .0
                    .derive::<secp256k1_ecdsa::SecretKey>(&chain)?
                    .secret_key()
                    .public_key();

                crate::client::Result::Ok(public_key.to_evm_address())
            })
            .collect::<Result<_, _>>()?)
    }

    async fn sign_ed25519(&self, msg: &[u8], chain: &Chain) -> Result<Ed25519Signature, Self::Error> {
        // Get the private and public key for this Ed25519 address
        let private_key = self.0.derive::<ed25519::SecretKey>(chain)?.secret_key();
        let public_key = private_key.public_key().to_bytes();
        let signature = private_key.sign(msg).to_bytes();

        Ok(Ed25519Signature::new(public_key, signature))
    }

    async fn sign_evm(
        &self,
        msg: &[u8],
        chain: &Chain,
    ) -> Result<(secp256k1_ecdsa::PublicKey, secp256k1_ecdsa::Signature), Self::Error> {
        // Get the private and public key for this Evm address
        let private_key = self.0.derive::<secp256k1_ecdsa::SecretKey>(chain)?.secret_key();
        let public_key = private_key.public_key();
        let signature = private_key.sign(msg);

        Ok((public_key, signature))
    }
}

impl MnemonicSecretManager {
    /// Create a new [`MnemonicSecretManager`] from a BIP-39 mnemonic in English.
    ///
    /// For more information, see <https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki>.
    pub fn try_from_mnemonic(mnemonic: String) -> Result<Self, Error> {
        let mnemonic = Mnemonic::try_from(mnemonic)?;
        Ok(Self(Client::mnemonic_to_seed(&mnemonic)))
    }

    /// Create a new [`MnemonicSecretManager`] from a hex-encoded raw seed string.
    pub fn try_from_hex_seed(mut hex: String) -> Result<Self, Error> {
        let mut bytes: Vec<u8> = prefix_hex::decode(hex.as_str())?;
        let seed = Seed::from_bytes(&bytes);
        hex.zeroize();
        bytes.zeroize();
        Ok(Self(seed))
    }
}

impl From<Mnemonic> for MnemonicSecretManager {
    fn from(m: Mnemonic) -> Self {
        Self(Client::mnemonic_to_seed(&m))
    }
}

/// A mnemonic (space separated list of words) that allows to create a seed from.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Mnemonic(String);

impl Mnemonic {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for Mnemonic {
    type Error = Error;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        // trim because empty spaces could create a different seed https://github.com/iotaledger/crypto.rs/issues/125
        let trimmed = value.trim();
        // first we check if the mnemonic is valid to give meaningful errors
        if let Err(err) = crypto::keys::bip39::wordlist::verify(trimmed, &crypto::keys::bip39::wordlist::ENGLISH) {
            value.zeroize();
            Err(crate::client::Error::InvalidMnemonic(format!("{err:?}")))
        } else {
            let mnemonic = trimmed.to_string();
            value.zeroize();
            Ok(Self(mnemonic))
        }
    }
}

pub trait MnemonicLike: Send {
    fn to_mnemonic(self) -> Result<Mnemonic, Error>;
}

impl MnemonicLike for Mnemonic {
    fn to_mnemonic(self) -> Result<Mnemonic, Error> {
        Ok(self)
    }
}

impl MnemonicLike for String {
    fn to_mnemonic(self) -> Result<Mnemonic, Error> {
        Mnemonic::try_from(self)
    }
}

impl MnemonicLike for Vec<String> {
    fn to_mnemonic(mut self) -> Result<Mnemonic, Error> {
        let m = self.join(" ");
        self.zeroize();
        Mnemonic::try_from(m)
    }
}

impl MnemonicLike for [&'static str; 24] {
    fn to_mnemonic(self) -> Result<Mnemonic, Error> {
        let m = self.join(" ");
        Mnemonic::try_from(m)
    }
}

// that's only necessary to use it in `assert!` macros
impl core::fmt::Debug for Mnemonic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "<mnemonic>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::block::address::ToBech32Ext;

    #[tokio::test]
    async fn address() {
        use crate::client::constants::IOTA_COIN_TYPE;

        let mnemonic = "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally".to_owned();
        let secret_manager = MnemonicSecretManager::try_from_mnemonic(mnemonic).unwrap();

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

    #[test]
    fn mnemonic_like() {
        // Mnemonic from a space-separated word list stored in a `String`
        let mnemonic1 = "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally".to_owned().to_mnemonic().unwrap();
        // Mnemonic from a word list stored in a `Vec<String>`
        let mnemonic2 = [
            "giant", "dynamic", "museum", "toddler", "six", "deny", "defense", "ostrich", "bomb", "access", "mercy",
            "blood", "explain", "muscle", "shoot", "shallow", "glad", "autumn", "author", "calm", "heavy", "hawk",
            "abuse", "rally",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>()
        .to_mnemonic()
        .unwrap();
        // Mnemonic from a word list stored in a `[&'static str; 24]`
        let mnemonic3 = [
            "giant", "dynamic", "museum", "toddler", "six", "deny", "defense", "ostrich", "bomb", "access", "mercy",
            "blood", "explain", "muscle", "shoot", "shallow", "glad", "autumn", "author", "calm", "heavy", "hawk",
            "abuse", "rally",
        ]
        .to_mnemonic()
        .unwrap();

        assert_eq!(mnemonic1, mnemonic2);
        assert_eq!(mnemonic1, mnemonic3);
        assert_eq!(mnemonic2, mnemonic3);

        // Different mnemonic
        let mnemonic4 = [
            "endorse", "answer", "radar", "about", "source", "reunion", "marriage", "tag", "sausage", "weekend",
            "frost", "daring", "base", "attack", "because", "joke", "dream", "slender", "leisure", "group", "reason",
            "prepare", "broken", "river",
        ]
        .to_mnemonic()
        .unwrap();

        assert_ne!(mnemonic1, mnemonic4);
        assert_ne!(mnemonic2, mnemonic4);
        assert_ne!(mnemonic3, mnemonic4);

        // Incorrect mnemonic
        assert!(
            [
                "dynamic", "giant", "museum", "toddler", "six", "deny", "defense", "ostrich", "bomb", "access",
                "mercy", "blood", "explain", "muscle", "shoot", "shallow", "glad", "autumn", "author", "calm", "heavy",
                "hawk", "abuse", "rally"
            ]
            .to_mnemonic()
            .is_err()
        );
    }
}
