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

use super::Client;
use crate::{
    client::ClientError,
    types::block::{
        address::{Address, Bech32Address, Ed25519Address, Hrp, ToBech32Ext},
        payload::TaggedDataPayload,
        Block, BlockId,
    },
    utils::ConvertTo,
};

/// Transforms a prefix hex encoded public key to a bech32 encoded address
pub fn hex_public_key_to_bech32_address(
    hex: &str,
    bech32_hrp: impl ConvertTo<Hrp>,
) -> Result<Bech32Address, ClientError> {
    let public_key: [u8; Ed25519Address::LENGTH] = prefix_hex::decode(hex)?;
    let address = Ed25519Address::new(Blake2b256::digest(public_key).into());

    Ok(Address::Ed25519(address).try_to_bech32(bech32_hrp)?)
}

/// Generates a new mnemonic.
pub fn generate_mnemonic() -> Result<Mnemonic, ClientError> {
    let mut entropy = [0u8; 32];
    utils::rand::fill(&mut entropy)?;
    let mnemonic = wordlist::encode(&entropy, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| ClientError::InvalidMnemonic(format!("{e:?}")))?;
    entropy.zeroize();
    Ok(mnemonic)
}

/// Returns a hex encoded seed for a mnemonic.
pub fn mnemonic_to_hex_seed(mnemonic: impl Borrow<MnemonicRef>) -> Result<String, ClientError> {
    Ok(prefix_hex::encode(mnemonic_to_seed(mnemonic)?.as_ref()))
}

/// Returns a seed for a mnemonic.
pub fn mnemonic_to_seed(mnemonic: impl Borrow<MnemonicRef>) -> Result<Seed, ClientError> {
    // first we check if the mnemonic is valid to give meaningful errors
    verify_mnemonic(mnemonic.borrow())?;
    Ok(crypto::keys::bip39::mnemonic_to_seed(
        mnemonic.borrow(),
        &Passphrase::default(),
    ))
}

/// Verifies that a &str is a valid mnemonic.
pub fn verify_mnemonic(mnemonic: impl Borrow<MnemonicRef>) -> Result<(), ClientError> {
    crypto::keys::bip39::wordlist::verify(mnemonic.borrow(), &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| ClientError::InvalidMnemonic(format!("{e:?}")))?;
    Ok(())
}

/// Requests funds from a faucet
pub async fn request_funds_from_faucet(url: &str, bech32_address: &Bech32Address) -> Result<String, ClientError> {
    let mut map = HashMap::new();
    map.insert("address", bech32_address.to_string());

    let client = reqwest::Client::new();
    let faucet_response = client
        .post(url)
        .json(&map)
        .send()
        .await
        .map_err(|err| ClientError::Node(err.into()))?
        .text()
        .await
        .map_err(|err| ClientError::Node(err.into()))?;
    Ok(faucet_response)
}

impl Client {
    /// Converts an address to its bech32 representation
    pub async fn address_to_bech32(
        &self,
        address: Address,
        bech32_hrp: Option<impl ConvertTo<Hrp>>,
    ) -> Result<Bech32Address, ClientError> {
        match bech32_hrp {
            Some(hrp) => Ok(address.to_bech32(hrp.convert()?)),
            None => Ok(address.to_bech32(self.get_bech32_hrp().await?)),
        }
    }

    /// Transforms a hex encoded public key to a bech32 encoded address
    pub async fn hex_public_key_to_bech32_address(
        &self,
        hex: &str,
        bech32_hrp: Option<impl ConvertTo<Hrp>>,
    ) -> Result<Bech32Address, ClientError> {
        match bech32_hrp {
            Some(hrp) => Ok(hex_public_key_to_bech32_address(hex, hrp)?),
            None => Ok(hex_public_key_to_bech32_address(hex, self.get_bech32_hrp().await?)?),
        }
    }

    /// Generates a new mnemonic.
    pub fn generate_mnemonic() -> Result<Mnemonic, ClientError> {
        generate_mnemonic()
    }

    /// Returns a seed for a mnemonic.
    pub fn mnemonic_to_seed(mnemonic: impl Borrow<MnemonicRef>) -> Result<Seed, ClientError> {
        mnemonic_to_seed(mnemonic)
    }

    /// Returns a hex encoded seed for a mnemonic.
    pub fn mnemonic_to_hex_seed(mnemonic: impl Borrow<MnemonicRef>) -> Result<String, ClientError> {
        mnemonic_to_hex_seed(mnemonic)
    }

    /// UTF-8 encodes the `tag` of a given TaggedDataPayload.
    pub fn tag_to_utf8(payload: &TaggedDataPayload) -> Result<String, ClientError> {
        String::from_utf8(payload.tag().to_vec())
            .map_err(|_| ClientError::TaggedData("found invalid UTF-8".to_string()))
    }

    /// UTF-8 encodes the `data` of a given TaggedDataPayload.
    pub fn data_to_utf8(payload: &TaggedDataPayload) -> Result<String, ClientError> {
        String::from_utf8(payload.data().to_vec())
            .map_err(|_| ClientError::TaggedData("found invalid UTF-8".to_string()))
    }

    /// UTF-8 encodes both the `tag` and `data` of a given TaggedDataPayload.
    pub fn tagged_data_to_utf8(payload: &TaggedDataPayload) -> Result<(String, String), ClientError> {
        Ok((Self::tag_to_utf8(payload)?, Self::data_to_utf8(payload)?))
    }

    pub async fn block_id(&self, block: &Block) -> Result<BlockId, ClientError> {
        Ok(block.id(&self.get_protocol_parameters().await?))
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

pub fn unix_timestamp_now() -> core::time::Duration {
    instant::SystemTime::now()
        .duration_since(instant::SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
}
