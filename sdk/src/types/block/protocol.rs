// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use core::borrow::Borrow;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use getset::{CopyGetters, Getters};
use packable::{
    prefix::{BoxedSlicePrefix, StringPrefix},
    Packable, PackableExt,
};

use super::{
    address::Hrp,
    slot::{EpochIndex, SlotIndex},
};
use crate::types::block::{
    error::UnpackPrefixOptionErrorExt, helper::network_name_to_id, output::RentStructure, ConvertTo, Error,
    PROTOCOL_VERSION,
};

// TODO: The API spec lists this field as optional, but is it really? And if so, what would the default be?
pub const DEFAULT_SLOTS_PER_EPOCH_EXPONENT: u32 = 10;

/// Defines the parameters of the protocol.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, Getters, CopyGetters)]
#[packable(unpack_error = Error)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ProtocolParameters {
    // The version of the protocol running.
    #[getset(get_copy = "pub")]
    version: u8,
    // The human friendly name of the network.
    #[packable(unpack_error_with = |err| Error::InvalidNetworkName(err.into_item_err()))]
    #[serde(with = "crate::utils::serde::string_prefix")]
    #[getset(skip)]
    network_name: StringPrefix<u8>,
    // The HRP prefix used for Bech32 addresses in the network.
    #[getset(get_copy = "pub")]
    bech32_hrp: Hrp,
    // The rent structure used by given node/network.
    #[getset(get = "pub")]
    rent_structure: RentStructure,
    // The work score structure used by the node/network.
    #[getset(get = "pub")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    work_score_structure: Option<WorkScoreStructure>,
    // TokenSupply defines the current token supply on the network.
    #[serde(with = "crate::utils::serde::string")]
    #[getset(get_copy = "pub")]
    token_supply: u64,
    // Genesis timestamp at which the slots start to count.
    #[serde(with = "crate::utils::serde::string")]
    #[getset(get_copy = "pub")]
    genesis_unix_timestamp: u32,
    // Duration of each slot in seconds.
    #[getset(get_copy = "pub")]
    slot_duration_in_seconds: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(skip)]
    slots_per_epoch_exponent: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    mana_generation_rate: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    mana_generation_rate_exponent: Option<u32>,
    #[packable(unpack_error_with = |err| Error::InvalidManaDecayFactors(err.into_opt_error()))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(skip)]
    mana_decay_factors: Option<BoxedSlicePrefix<u32, u8>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    mana_decay_factors_exponent: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    mana_decay_factor_epochs_sum: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    mana_decay_factor_epochs_sum_exponent: Option<u32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::utils::serde::option_string"
    )]
    #[getset(get_copy = "pub")]
    staking_unbonding_period: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    eviction_age: Option<SlotIndex>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    liveness_threshold: Option<SlotIndex>,
    epoch_nearing_threshold: SlotIndex,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub")]
    version_signaling: Option<VersionSignalingParameters>,
}

// This implementation is required to make [`ProtocolParameters`] a [`Packable`] visitor.
impl Borrow<()> for ProtocolParameters {
    fn borrow(&self) -> &() {
        &()
    }
}

impl Default for ProtocolParameters {
    fn default() -> Self {
        Self {
            version: PROTOCOL_VERSION,
            // Unwrap: Known to be valid
            network_name: String::from("iota-core-testnet").try_into().unwrap(),
            bech32_hrp: Hrp::from_str_unchecked("smr"),
            rent_structure: Default::default(),
            work_score_structure: Default::default(),
            token_supply: 1_813_620_509_061_365,
            genesis_unix_timestamp: 1582328545,
            slot_duration_in_seconds: 10,
            epoch_nearing_threshold: 20.into(),
            slots_per_epoch_exponent: Default::default(),
            mana_generation_rate: Default::default(),
            mana_generation_rate_exponent: Default::default(),
            mana_decay_factors: Default::default(),
            mana_decay_factors_exponent: Default::default(),
            mana_decay_factor_epochs_sum: Default::default(),
            mana_decay_factor_epochs_sum_exponent: Default::default(),
            staking_unbonding_period: Default::default(),
            eviction_age: Default::default(),
            liveness_threshold: Default::default(),
            version_signaling: Default::default(),
        }
    }
}

impl ProtocolParameters {
    /// Creates a new [`ProtocolParameters`].
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        version: u8,
        network_name: impl Into<String>,
        bech32_hrp: impl ConvertTo<Hrp>,
        rent_structure: RentStructure,
        token_supply: u64,
        genesis_unix_timestamp: u32,
        slot_duration_in_seconds: u8,
        epoch_nearing_threshold: impl Into<SlotIndex>,
    ) -> Result<Self, Error> {
        Ok(Self {
            version,
            network_name: <StringPrefix<u8>>::try_from(network_name.into()).map_err(Error::InvalidStringPrefix)?,
            bech32_hrp: bech32_hrp.convert()?,
            rent_structure,
            token_supply,
            genesis_unix_timestamp,
            slot_duration_in_seconds,
            epoch_nearing_threshold: epoch_nearing_threshold.into(),
            ..Default::default()
        })
    }

    /// Returns the network name of the [`ProtocolParameters`].
    pub fn network_name(&self) -> &str {
        &self.network_name
    }

    /// Returns the network ID of the [`ProtocolParameters`].
    pub fn network_id(&self) -> u64 {
        network_name_to_id(&self.network_name)
    }

    /// Returns the mana decay factors slice of the [`ProtocolParameters`].
    pub fn mana_decay_factors(&self) -> Option<&[u32]> {
        self.mana_decay_factors.as_ref().map(|slice| slice.as_ref())
    }

    /// Returns the epoch nearing threshold of the [`ProtocolParameters`].
    pub fn epoch_nearing_threshold(&self) -> SlotIndex {
        self.epoch_nearing_threshold
    }

    pub fn slots_per_epoch_exponent(&self) -> u32 {
        self.slots_per_epoch_exponent
            .unwrap_or(DEFAULT_SLOTS_PER_EPOCH_EXPONENT)
    }

    pub fn slot_index(&self, timestamp: u64) -> SlotIndex {
        calc_slot_index(
            timestamp,
            self.genesis_unix_timestamp(),
            self.slot_duration_in_seconds(),
        )
    }

    pub fn epoch_index(&self, timestamp: u64) -> EpochIndex {
        self.slot_index(timestamp)
            .to_epoch_index(self.slots_per_epoch_exponent())
    }

    pub fn hash(&self) -> ProtocolParametersHash {
        ProtocolParametersHash::new(Blake2b256::digest(self.pack_to_vec()).into())
    }
}

pub fn calc_slot_index(timestamp: u64, genesis_unix_timestamp: u32, slot_duration_in_seconds: u8) -> SlotIndex {
    (1 + (timestamp - genesis_unix_timestamp as u64) / slot_duration_in_seconds as u64).into()
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[packable(unpack_error = Error)]
#[getset(get_copy = "pub")]
pub struct WorkScoreStructure {
    /// Modifier for network traffic per byte.
    data_byte: u32,
    /// Modifier for work done to process a block.
    block: u32,
    /// Modifier for slashing when there are insufficient strong tips.
    missing_parent: u32,
    /// Modifier for loading UTXOs and performing mana calculations.
    input: u32,
    /// Modifier for loading and checking the context input.
    context_input: u32,
    /// Modifier for storing UTXOs.
    output: u32,
    /// Modifier for calculations using native tokens.
    native_token: u32,
    /// Modifier for storing staking features.
    staking: u32,
    /// Modifier for storing block issuer features.
    block_issuer: u32,
    /// Modifier for accessing the account-based ledger to transform mana to Block Issuance Credits.
    allotment: u32,
    /// Modifier for the block signature check.
    signature_ed25519: u32,
    /// The minimum count of strong parents in a basic block.
    min_strong_parents_threshold: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[packable(unpack_error = Error)]
#[getset(get_copy = "pub")]
pub struct VersionSignalingParameters {
    window_size: u32,
    window_target_ratio: u32,
    activation_offset: u32,
}

/// Returns a [`ProtocolParameters`] for testing purposes.
#[cfg(any(feature = "test", feature = "rand"))]
pub fn protocol_parameters() -> ProtocolParameters {
    ProtocolParameters::new(
        2,
        "testnet",
        "rms",
        crate::types::block::output::RentStructure::new(500, 10, 1),
        1_813_620_509_061_365,
        1582328545,
        10,
        20,
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
