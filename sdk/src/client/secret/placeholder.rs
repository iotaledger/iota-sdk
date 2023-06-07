// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`PlaceholderSecretManager`].

use std::ops::Range;

use async_trait::async_trait;
use crypto::{
    keys::slip10::Chain,
    signatures::secp256k1_ecdsa::{self, EvmAddress},
};

use super::{GenerateAddressOptions, SecretManage, SignTransactionEssence};
use crate::{
    client::{secret::PreparedTransactionData, Error},
    types::block::{address::Ed25519Address, signature::Ed25519Signature, unlock::Unlocks},
};

/// Secret manager that is only useful to prevent accidental address generation in a wallet
/// that has an offline counterpart for address generation and signing.
pub struct PlaceholderSecretManager;

#[async_trait]
impl SecretManage for PlaceholderSecretManager {
    type Error = Error;

    async fn generate_ed25519_addresses(
        &self,
        _coin_type: u32,
        _account_index: u32,
        _address_indexes: Range<u32>,
        _options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<Ed25519Address>, Self::Error> {
        Err(Error::PlaceholderSecretManager)
    }

    async fn generate_evm_addresses(
        &self,
        _coin_type: u32,
        _account_index: u32,
        _address_indexes: Range<u32>,
        _options: impl Into<Option<GenerateAddressOptions>> + Send,
    ) -> Result<Vec<EvmAddress>, Self::Error> {
        Err(Error::PlaceholderSecretManager)
    }

    async fn sign_ed25519(&self, _msg: &[u8], _chain: &Chain) -> Result<Ed25519Signature, Self::Error> {
        Err(Error::PlaceholderSecretManager)
    }

    async fn sign_evm(
        &self,
        _msg: &[u8],
        _chain: &Chain,
    ) -> Result<(secp256k1_ecdsa::PublicKey, secp256k1_ecdsa::Signature), Self::Error> {
        Err(Error::PlaceholderSecretManager)
    }
}

#[async_trait]
impl SignTransactionEssence for PlaceholderSecretManager {
    async fn sign_transaction_essence(
        &self,
        _prepared_transaction_data: &PreparedTransactionData,
        _time: Option<u32>,
    ) -> Result<Unlocks, <Self as SecretManage>::Error> {
        Err(Error::PlaceholderSecretManager)
    }
}
