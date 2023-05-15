// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Range;

use serde::{Deserialize, Serialize};

use super::ADDRESS_GAP_RANGE;
use crate::{
    client::{
        api::types::{Bech32Addresses, RawAddresses},
        constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
        secret::{GenerateAddressOptions, SecretManage, SecretManager},
        Client, Result,
    },
    types::block::address::Address,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAddressesOptions {
    /// Coin type
    pub coin_type: u32,
    /// Account index
    pub account_index: u32,
    /// Range
    pub range: Range<u32>,
    /// Bech32 human readable part
    pub bech32_hrp: String,
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
    pub fn with_bech32_hrp(mut self, bech32_hrp: impl Into<String>) -> Self {
        self.bech32_hrp = bech32_hrp.into();
        self
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
            bech32_hrp: SHIMMER_TESTNET_BECH32_HRP.to_string(),
            options: Default::default(),
        }
    }
}

impl SecretManager {
    /// Get a vector of public address strings
    pub async fn get_addresses(
        &self,
        GetAddressesOptions {
            coin_type,
            account_index,
            range,
            bech32_hrp,
            options,
        }: GetAddressesOptions,
    ) -> Result<Vec<String>> {
        Ok(self
            .generate_addresses(coin_type, account_index, range, options)
            .await?
            .into_iter()
            .map(|a| a.to_bech32(&bech32_hrp))
            .collect())
    }

    /// Get a vector of EVM address strings
    pub async fn get_evm_addresses(
        &self,
        GetAddressesOptions {
            coin_type,
            account_index,
            range,
            bech32_hrp,
            options,
        }: GetAddressesOptions,
    ) -> Result<Vec<String>> {
        Ok(self
            .generate_evm_addresses(coin_type, account_index, range, options)
            .await?
            .into_iter()
            .map(|a| {
                bech32::encode(
                    &bech32_hrp,
                    bech32::ToBase32::to_base32(&a.as_ref()),
                    bech32::Variant::Bech32,
                )
                .unwrap()
            })
            .collect())
    }

    /// Get a vector of public addresses
    pub async fn get_raw_addresses(
        &self,
        GetAddressesOptions {
            coin_type,
            account_index,
            range,
            options,
            ..
        }: GetAddressesOptions,
    ) -> Result<Vec<Address>> {
        self.generate_addresses(
            coin_type,
            account_index,
            range,
            options.map(|mut o| {
                o.internal = false;
                o
            }),
        )
        .await
    }

    /// Get the vector of public and internal addresses bech32 encoded
    pub async fn get_all_addresses(&self, options: GetAddressesOptions) -> Result<Bech32Addresses> {
        let bech32_hrp = options.bech32_hrp.clone();
        let addresses = self.get_all_raw_addresses(options).await?;

        Ok(Bech32Addresses {
            public: addresses.public.into_iter().map(|a| a.to_bech32(&bech32_hrp)).collect(),
            internal: addresses
                .internal
                .into_iter()
                .map(|a| a.to_bech32(&bech32_hrp))
                .collect(),
        })
    }

    /// Get the vector of public and internal addresses
    pub async fn get_all_raw_addresses(
        &self,
        GetAddressesOptions {
            coin_type,
            account_index,
            range,
            options,
            ..
        }: GetAddressesOptions,
    ) -> Result<RawAddresses> {
        let public_addresses = self
            .generate_addresses(
                coin_type,
                account_index,
                range.clone(),
                options.map(|mut o| {
                    o.internal = false;
                    o
                }),
            )
            .await?;

        let internal_addresses = self
            .generate_addresses(
                coin_type,
                account_index,
                range,
                options
                    .map(|mut o| {
                        o.internal = true;
                        o
                    })
                    .or_else(|| Some(GenerateAddressOptions::internal())),
            )
            .await?;

        Ok(RawAddresses {
            public: public_addresses,
            internal: internal_addresses,
        })
    }
}

/// Function to find the index and public (false) or internal (true) type of an Bech32 encoded address
pub async fn search_address(
    secret_manager: &SecretManager,
    bech32_hrp: &str,
    coin_type: u32,
    account_index: u32,
    range: Range<u32>,
    address: &Address,
) -> Result<(u32, bool)> {
    let addresses = secret_manager
        .get_all_raw_addresses(
            GetAddressesOptions::default()
                .with_coin_type(coin_type)
                .with_account_index(account_index)
                .with_range(range.clone()),
        )
        .await?;
    for index in 0..addresses.public.len() {
        if addresses.public[index] == *address {
            return Ok((range.start + index as u32, false));
        }
        if addresses.internal[index] == *address {
            return Ok((range.start + index as u32, true));
        }
    }
    Err(crate::client::Error::InputAddressNotFound {
        address: address.to_bech32(bech32_hrp),
        range: format!("{range:?}"),
    })
}
