// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Range;

use serde::{Deserialize, Serialize};

use super::ADDRESS_GAP_RANGE;
use crate::{
    client::{
        constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
        secret::{GenerateAddressOptions, SecretManage, SecretManager},
        Client, Result,
    },
    types::block::{
        address::{Address, Bech32Address, Hrp, ToBech32Ext},
        ConvertTo,
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct GetAddressesOptions {
    /// Coin type
    pub coin_type: u32,
    /// Account index
    pub account_index: u32,
    /// Range
    pub range: Range<u32>,
    /// Bech32 human readable part
    pub bech32_hrp: Hrp,
    /// Options
    pub options: Option<GenerateAddressOptions>,
}

impl GetAddressesOptions {
    pub async fn from_client(client: &Client) -> Result<Self> {
        Ok(Self::default().with_bech32_hrp(client.get_bech32_hrp().await?))
    }

    /// Set the coin type
    pub fn with_coin_type(mut self, coin_type: u32) -> Self {
        self.coin_type = coin_type;
        self
    }

    /// Set the account index
    pub fn with_account_index(mut self, account_index: u32) -> Self {
        self.account_index = account_index;
        self
    }

    /// Set range to the builder
    pub fn with_range(mut self, range: Range<u32>) -> Self {
        self.range = range;
        self
    }

    /// Set bech32 human readable part (hrp)
    pub fn with_bech32_hrp(mut self, bech32_hrp: Hrp) -> Self {
        self.bech32_hrp = bech32_hrp;
        self
    }

    /// Set bech32 human readable part (hrp) from something that might be valid
    pub fn try_with_bech32_hrp(mut self, bech32_hrp: impl ConvertTo<Hrp>) -> Result<Self> {
        self.bech32_hrp = bech32_hrp.convert()?;
        Ok(self)
    }

    pub fn internal(mut self) -> Self {
        match &mut self.options {
            Some(o) => {
                o.internal = true;
                self
            }
            None => self.with_options(GenerateAddressOptions::internal()),
        }
    }

    /// Set the metadata for the address generation (used for ledger to display addresses or not)
    pub fn with_options(mut self, options: impl Into<Option<GenerateAddressOptions>>) -> Self {
        self.options = options.into();
        self
    }
}

impl Default for GetAddressesOptions {
    fn default() -> Self {
        Self {
            coin_type: SHIMMER_COIN_TYPE,
            account_index: 0,
            range: 0..ADDRESS_GAP_RANGE,
            bech32_hrp: SHIMMER_TESTNET_BECH32_HRP,
            options: Default::default(),
        }
    }
}

impl SecretManager {
    /// Get a vector of public bech32 addresses
    pub async fn generate_ed25519_addresses(
        &self,
        GetAddressesOptions {
            coin_type,
            account_index,
            range,
            bech32_hrp,
            options,
        }: GetAddressesOptions,
    ) -> Result<Vec<Bech32Address>> {
        Ok(
            SecretManage::generate_ed25519_addresses(self, coin_type, account_index, range, options)
                .await?
                .into_iter()
                .map(|a| a.to_bech32(bech32_hrp))
                .collect(),
        )
    }

    /// Get a vector of EVM address strings
    pub async fn generate_evm_addresses(
        &self,
        GetAddressesOptions {
            coin_type,
            account_index,
            range,
            options,
            ..
        }: GetAddressesOptions,
    ) -> Result<Vec<String>> {
        Ok(
            SecretManage::generate_evm_addresses(self, coin_type, account_index, range, options)
                .await?
                .into_iter()
                .map(|a| prefix_hex::encode(a.as_ref()))
                .collect(),
        )
    }
}

/// Function to find the index and public (false) or internal (true) type of an Bech32 encoded address
pub async fn search_address(
    secret_manager: &SecretManager,
    bech32_hrp: Hrp,
    coin_type: u32,
    account_index: u32,
    range: Range<u32>,
    address: &Address,
) -> Result<(u32, bool)> {
    let opts = GetAddressesOptions::default()
        .with_coin_type(coin_type)
        .with_account_index(account_index)
        .with_range(range.clone());
    let public = secret_manager.generate_ed25519_addresses(opts.clone()).await?;
    let internal = secret_manager.generate_ed25519_addresses(opts.internal()).await?;
    for index in 0..public.len() {
        if public[index].inner == *address {
            return Ok((range.start + index as u32, false));
        }
        if internal[index].inner == *address {
            return Ok((range.start + index as u32, true));
        }
    }
    Err(crate::client::Error::InputAddressNotFound {
        address: address.to_bech32(bech32_hrp).to_string(),
        range: format!("{range:?}"),
    })
}
