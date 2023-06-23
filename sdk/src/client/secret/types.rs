// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Miscellaneous types for secret managers.

use crypto::keys::slip10::Chain;
use serde::{Deserialize, Serialize};

use crate::types::block::{
    address::Address,
    output::{Output, OutputId, OutputMetadata},
};

/// Stronghold DTO to allow the creation of a Stronghold secret manager from bindings
#[cfg(feature = "stronghold")]
#[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrongholdDto {
    /// The Stronghold password
    pub password: Option<crate::client::Password>,
    /// The timeout for auto key clearing, in seconds
    pub timeout: Option<u64>,
    /// The path for the Stronghold file
    pub snapshot_path: String,
}

#[cfg(feature = "stronghold")]
impl core::fmt::Debug for StrongholdDto {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StrongholdDto")
            .field("timeout", &self.timeout)
            .field("snapshot_path", &self.snapshot_path)
            .finish()
    }
}

/// An account address.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAddress {
    /// The address.
    pub address: Address,
    /// The address key index.
    pub key_index: u32,
    /// Determines if an address is a public or an internal (change) address.
    pub internal: bool,
}

/// Options provided to generate addresses.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GenerateAddressOptions {
    pub internal: bool,
    /// Display the address on ledger devices.
    pub ledger_nano_prompt: bool,
}

impl GenerateAddressOptions {
    pub const fn internal() -> Self {
        Self {
            internal: true,
            ledger_nano_prompt: false,
        }
    }
}

/// The Ledger device status.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LedgerApp {
    /// Opened app name.
    pub(crate) name: String,
    /// Opened app version.
    pub(crate) version: String,
}

impl LedgerApp {
    /// Opened app name.
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Opened app version.
    pub fn version(&self) -> &String {
        &self.version
    }
}

/// Ledger Device Type
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LedgerDeviceType {
    /// Device Type Nano S
    #[serde(alias = "ledgerNanoS")]
    LedgerNanoS,
    /// Device Type Nano X
    #[serde(alias = "ledgerNanoX")]
    LedgerNanoX,
    /// Device Type Nano S Plus
    #[serde(alias = "ledgerNanoSPlus")]
    LedgerNanoSPlus,
}

/// The Ledger device status.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LedgerNanoStatus {
    /// Ledger is available and ready to be used.
    pub(crate) connected: bool,
    /// Ledger is connected and locked, Some(true/false) for IOTA/Shimmer, None for the rest.
    pub(crate) locked: Option<bool>,
    /// Ledger blind signing enabled
    pub(crate) blind_signing_enabled: bool,
    /// Ledger opened app.
    pub(crate) app: Option<LedgerApp>,
    /// Ledger device
    pub(crate) device: Option<LedgerDeviceType>,
    /// Buffer size on device
    pub(crate) buffer_size: Option<usize>,
}

impl LedgerNanoStatus {
    /// Ledger is available and ready to be used.
    pub fn connected(&self) -> bool {
        self.connected
    }
    /// Ledger is connected and locked, Some(true/false) for IOTA/Shimmer, None for the rest.
    pub fn locked(&self) -> Option<bool> {
        self.locked
    }
    /// Ledger blind signing enabled
    pub fn blind_signing_enabled(&self) -> bool {
        self.blind_signing_enabled
    }
    /// Ledger opened app.
    pub fn app(&self) -> Option<&LedgerApp> {
        self.app.as_ref()
    }
    /// Ledger device
    pub fn device(&self) -> Option<LedgerDeviceType> {
        self.device
    }
    /// Buffer size on device
    pub fn buffer_size(&self) -> Option<usize> {
        self.buffer_size
    }
}

/// Data for transaction inputs for signing and ordering of unlock blocks
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputSigningData {
    /// The output
    pub output: Output,
    /// The output metadata
    pub output_metadata: OutputMetadata,
    /// The chain derived from seed, only for ed25519 addresses
    pub chain: Option<Chain>,
}

impl InputSigningData {
    /// Return the [OutputId]
    pub fn output_id(&self) -> &OutputId {
        self.output_metadata.output_id()
    }
}
