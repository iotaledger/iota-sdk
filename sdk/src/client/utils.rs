// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Utility functions for IOTA

use core::borrow::Borrow;
use std::collections::HashMap;

use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::bip39::{wordlist, Mnemonic, MnemonicRef, Passphrase, Seed},
    utils,
};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use super::{Client, ClientInner};
use crate::{
    client::{Error, Result},
    types::block::{
        address::{Address, Bech32Address, Ed25519Address, Hrp, ToBech32Ext},
        output::{AliasId, NftId},
        payload::TaggedDataPayload,
        ConvertTo,
    },
};

/// Transforms bech32 to hex
pub fn bech32_to_hex(bech32: impl ConvertTo<Bech32Address>) -> Result<String> {
    Ok(match bech32.convert()?.inner() {
        Address::Ed25519(ed) => ed.to_string(),
        Address::Alias(alias) => alias.to_string(),
        Address::Nft(nft) => nft.to_string(),
    })
}

/// Transforms a hex encoded address to a bech32 encoded address
pub fn hex_to_bech32(hex: &str, bech32_hrp: impl ConvertTo<Hrp>) -> Result<Bech32Address> {
    let address: Ed25519Address = hex.parse::<Ed25519Address>()?;
    Ok(Address::Ed25519(address).try_to_bech32(bech32_hrp)?)
}

/// Transforms a prefix hex encoded public key to a bech32 encoded address
pub fn hex_public_key_to_bech32_address(hex: &str, bech32_hrp: impl ConvertTo<Hrp>) -> Result<Bech32Address> {
    let public_key: [u8; Ed25519Address::LENGTH] = prefix_hex::decode(hex)?;

    let address = Blake2b256::digest(public_key)
        .try_into()
        .map_err(|_e| Error::Blake2b256("hashing the public key failed."))?;
    let address: Ed25519Address = Ed25519Address::new(address);
    Ok(Address::Ed25519(address).try_to_bech32(bech32_hrp)?)
}

/// Generates a new mnemonic.
pub fn generate_mnemonic() -> Result<Mnemonic> {
    let mut entropy = [0u8; 32];
    utils::rand::fill(&mut entropy)?;
    let mnemonic = wordlist::encode(&entropy, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| crate::client::Error::InvalidMnemonic(format!("{e:?}")))?;
    entropy.zeroize();
    Ok(mnemonic)
}

/// Returns a hex encoded seed for a mnemonic.
pub fn mnemonic_to_hex_seed(mnemonic: impl Borrow<MnemonicRef>) -> Result<String> {
    Ok(prefix_hex::encode(mnemonic_to_seed(mnemonic)?.as_ref()))
}

/// Returns a seed for a mnemonic.
pub fn mnemonic_to_seed(mnemonic: impl Borrow<MnemonicRef>) -> Result<Seed> {
    // first we check if the mnemonic is valid to give meaningful errors
    verify_mnemonic(mnemonic.borrow())?;
    Ok(crypto::keys::bip39::mnemonic_to_seed(
        mnemonic.borrow(),
        &Passphrase::default(),
    ))
}

/// Verifies that a &str is a valid mnemonic.
pub fn verify_mnemonic(mnemonic: impl Borrow<MnemonicRef>) -> Result<()> {
    crypto::keys::bip39::wordlist::verify(mnemonic.borrow(), &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| crate::client::Error::InvalidMnemonic(format!("{e:?}")))?;
    Ok(())
}

/// Requests funds from a faucet
pub async fn request_funds_from_faucet(url: &str, bech32_address: &Bech32Address) -> Result<String> {
    let mut map = HashMap::new();
    map.insert("address", bech32_address.to_string());

    let client = reqwest::Client::new();
    let faucet_response = client
        .post(url)
        .json(&map)
        .send()
        .await
        .map_err(|err| Error::Node(err.into()))?
        .text()
        .await
        .map_err(|err| Error::Node(err.into()))?;
    Ok(faucet_response)
}

impl ClientInner {
    /// Transforms a hex encoded address to a bech32 encoded address
    pub async fn hex_to_bech32(
        &self,
        hex: &str,
        bech32_hrp: Option<impl ConvertTo<Hrp>>,
    ) -> crate::client::Result<Bech32Address> {
        match bech32_hrp {
            Some(hrp) => Ok(hex_to_bech32(hex, hrp)?),
            None => Ok(hex_to_bech32(hex, self.get_bech32_hrp().await?)?),
        }
    }

    /// Transforms an alias id to a bech32 encoded address
    pub async fn alias_id_to_bech32(
        &self,
        alias_id: AliasId,
        bech32_hrp: Option<impl ConvertTo<Hrp>>,
    ) -> crate::client::Result<Bech32Address> {
        match bech32_hrp {
            Some(hrp) => Ok(alias_id.to_bech32(hrp.convert()?)),
            None => Ok(alias_id.to_bech32(self.get_bech32_hrp().await?)),
        }
    }

    /// Transforms an nft id to a bech32 encoded address
    pub async fn nft_id_to_bech32(
        &self,
        nft_id: NftId,
        bech32_hrp: Option<impl ConvertTo<Hrp>>,
    ) -> crate::client::Result<Bech32Address> {
        match bech32_hrp {
            Some(hrp) => Ok(nft_id.to_bech32(hrp.convert()?)),
            None => Ok(nft_id.to_bech32(self.get_bech32_hrp().await?)),
        }
    }

    /// Transforms a hex encoded public key to a bech32 encoded address
    pub async fn hex_public_key_to_bech32_address(
        &self,
        hex: &str,
        bech32_hrp: Option<impl ConvertTo<Hrp>>,
    ) -> crate::client::Result<Bech32Address> {
        match bech32_hrp {
            Some(hrp) => Ok(hex_public_key_to_bech32_address(hex, hrp)?),
            None => Ok(hex_public_key_to_bech32_address(hex, self.get_bech32_hrp().await?)?),
        }
    }
}

impl Client {
    /// Transforms bech32 to hex
    pub fn bech32_to_hex(bech32: impl ConvertTo<Bech32Address>) -> crate::client::Result<String> {
        bech32_to_hex(bech32)
    }

    /// Generates a new mnemonic.
    pub fn generate_mnemonic() -> Result<Mnemonic> {
        generate_mnemonic()
    }

    /// Returns a seed for a mnemonic.
    pub fn mnemonic_to_seed(mnemonic: impl Borrow<MnemonicRef>) -> Result<Seed> {
        mnemonic_to_seed(mnemonic)
    }

    /// Returns a hex encoded seed for a mnemonic.
    pub fn mnemonic_to_hex_seed(mnemonic: impl Borrow<MnemonicRef>) -> Result<String> {
        mnemonic_to_hex_seed(mnemonic)
    }

    /// UTF-8 encodes the `tag` of a given TaggedDataPayload.
    pub fn tag_to_utf8(payload: &TaggedDataPayload) -> Result<String> {
        String::from_utf8(payload.tag().to_vec()).map_err(|_| Error::TaggedData("found invalid UTF-8".to_string()))
    }

    /// UTF-8 encodes the `data` of a given TaggedDataPayload.
    pub fn data_to_utf8(payload: &TaggedDataPayload) -> Result<String> {
        String::from_utf8(payload.data().to_vec()).map_err(|_| Error::TaggedData("found invalid UTF-8".to_string()))
    }

    /// UTF-8 encodes both the `tag` and `data` of a given TaggedDataPayload.
    pub fn tagged_data_to_utf8(payload: &TaggedDataPayload) -> Result<(String, String)> {
        Ok((Self::tag_to_utf8(payload)?, Self::data_to_utf8(payload)?))
    }
}

/// A password wrapper that takes care of zeroing the memory when being dropped.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Zeroize, ZeroizeOnDrop, derive_more::From)]
pub struct Password(String);

impl Password {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
