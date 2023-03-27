// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`PlaceholderSecretManager`].

use std::ops::Range;

use async_trait::async_trait;
use crypto::keys::slip10::Chain;

use super::{GenerateAddressOptions, SecretManage, SecretManageExt};
use crate::{
    client::secret::PreparedTransactionData,
    types::block::{address::Address, signature::Ed25519Signature, unlock::Unlocks},
};

/// Secret manager that is only useful to prevent accidental address generation in a wallet
/// that has an offline counterpart for address generation and signing.
pub struct PlaceholderSecretManager;

#[async_trait]
impl SecretManage for PlaceholderSecretManager {
    async fn generate_addresses(
        &self,
        _coin_type: u32,
        _account_index: u32,
        _address_indexes: Range<u32>,
        _internal: bool,
        _: Option<GenerateAddressOptions>,
    ) -> crate::client::Result<Vec<Address>> {
        return Err(crate::client::Error::PlaceholderSecretManager);
    }

    async fn sign_ed25519(&self, _msg: &[u8], _chain: &Chain) -> crate::client::Result<Ed25519Signature> {
        return Err(crate::client::Error::PlaceholderSecretManager);
    }
}

#[async_trait]
impl SecretManageExt for PlaceholderSecretManager {
    async fn sign_transaction_essence(
        &self,
        _prepared_transaction_data: &PreparedTransactionData,
        _time: Option<u32>,
    ) -> crate::client::Result<Unlocks> {
        return Err(crate::client::Error::PlaceholderSecretManager);
    }
}
