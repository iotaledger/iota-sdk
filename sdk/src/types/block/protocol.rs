// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use core::borrow::Borrow;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{prefix::StringPrefix, Packable, PackableExt};

use super::{address::Hrp, slot::SlotIndex};
use crate::types::block::{helper::network_name_to_id, output::RentStructure, ConvertTo, Error, PROTOCOL_VERSION};

/// Defines the parameters of the protocol.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable)]
#[packable(unpack_error = Error)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ProtocolParameters {
    // The version of the protocol running.
    #[cfg_attr(feature = "serde", serde(rename = "version"))]
    pub(crate) protocol_version: u8,
    // The human friendly name of the network.
    #[packable(unpack_error_with = |err| Error::InvalidNetworkName(err.into_item_err()))]
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string_prefix"))]
    pub(crate) network_name: StringPrefix<u8>,
    // The HRP prefix used for Bech32 addresses in the network.
    pub(crate) bech32_hrp: Hrp,
    // The below max depth parameter of the network.
    pub(crate) below_max_depth: u8,
    // The rent structure used by given node/network.
    pub(crate) rent_structure: RentStructure,
    // TokenSupply defines the current token supply on the network.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) token_supply: u64,
    // Genesis timestamp at which the slots start to count.
    #[cfg_attr(feature = "serde", serde(alias = "genesisUnixTimestamp"))]
    pub(crate) genesis_unix_timestamp: u32,
    // Duration of each slot in seconds.
    #[cfg_attr(feature = "serde", serde(alias = "slotDurationInSeconds"))]
    pub(crate) slot_duration_in_seconds: u8,
}

// This implementation is required to make [`ProtocolParameters`] a [`Packable`] visitor.
impl Borrow<()> for ProtocolParameters {
    fn borrow(&self) -> &() {
        &()
    }
}

impl Default for ProtocolParameters {
    fn default() -> Self {
        // PANIC: These values are known to be correct.
        Self::new(
            PROTOCOL_VERSION,
            String::from("iota-core-testnet"),
            "smr",
            15,
            RentStructure::default(),
            1_813_620_509_061_365,
            1582328545,
            10,
        )
        .unwrap()
    }
}

impl ProtocolParameters {
    /// Creates a new [`ProtocolParameters`].
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        protocol_version: u8,
        network_name: String,
        bech32_hrp: impl ConvertTo<Hrp>,
        below_max_depth: u8,
        rent_structure: RentStructure,
        token_supply: u64,
        genesis_unix_timestamp: u32,
        slot_duration_in_seconds: u8,
    ) -> Result<Self, Error> {
        Ok(Self {
            protocol_version,
            network_name: <StringPrefix<u8>>::try_from(network_name).map_err(Error::InvalidStringPrefix)?,
            bech32_hrp: bech32_hrp.convert()?,
            below_max_depth,
            rent_structure,
            token_supply,
            genesis_unix_timestamp,
            slot_duration_in_seconds,
        })
    }

    /// Returns the protocol version of the [`ProtocolParameters`].
    pub fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    /// Returns the network name of the [`ProtocolParameters`].
    pub fn network_name(&self) -> &str {
        &self.network_name
    }

    /// Returns the network ID of the [`ProtocolParameters`].
    pub fn network_id(&self) -> u64 {
        network_name_to_id(&self.network_name)
    }

    /// Returns the bech32 HRP of the [`ProtocolParameters`].
    pub fn bech32_hrp(&self) -> &Hrp {
        &self.bech32_hrp
    }

    /// Returns the below max depth of the [`ProtocolParameters`].
    pub fn below_max_depth(&self) -> u8 {
        self.below_max_depth
    }

    /// Returns the rent structure of the [`ProtocolParameters`].
    pub fn rent_structure(&self) -> &RentStructure {
        &self.rent_structure
    }

    /// Returns the token supply of the [`ProtocolParameters`].
    pub fn token_supply(&self) -> u64 {
        self.token_supply
    }

    pub fn genesis_unix_timestamp(&self) -> u32 {
        self.genesis_unix_timestamp
    }

    pub fn slot_duration_in_seconds(&self) -> u8 {
        self.slot_duration_in_seconds
    }

    pub fn slot_index(&self, timestamp: u64) -> SlotIndex {
        slot_index(
            timestamp,
            self.genesis_unix_timestamp(),
            self.slot_duration_in_seconds(),
        )
    }

    pub fn hash(&self) -> ProtocolParametersHash {
        ProtocolParametersHash::new(Blake2b256::digest(self.pack_to_vec()).into())
    }
}

pub fn slot_index(timestamp: u64, genesis_unix_timestamp: u32, slot_duration_in_seconds: u8) -> SlotIndex {
    (1 + (timestamp - genesis_unix_timestamp as u64) / slot_duration_in_seconds as u64).into()
}

/// Returns a [`ProtocolParameters`] for testing purposes.
#[cfg(any(feature = "test", feature = "rand"))]
pub fn protocol_parameters() -> ProtocolParameters {
    ProtocolParameters::new(
        2,
        String::from("testnet"),
        "rms",
        15,
        crate::types::block::output::RentStructure::new(500, 10, 1),
        1_813_620_509_061_365,
        1582328545,
        10,
    )
    .unwrap()
}

impl_id!(
    pub ProtocolParametersHash,
    32,
    "The hash of the protocol parameters."
);

#[cfg(feature = "serde")]
string_serde_impl!(ProtocolParametersHash);
