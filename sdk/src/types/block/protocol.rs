// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use core::borrow::Borrow;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use getset::{CopyGetters, Getters};
use packable::{prefix::StringPrefix, Packable, PackableExt};

use super::{
    address::Hrp,
    mana::ManaStructure,
    slot::{EpochIndex, SlotIndex},
};
use crate::types::block::{helper::network_name_to_id, output::RentStructure, ConvertTo, Error, PROTOCOL_VERSION};

/// Defines the parameters of the protocol at a particular version.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, Getters, CopyGetters)]
#[packable(unpack_error = Error)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[getset(get_copy = "pub")]
pub struct ProtocolParameters {
    /// The layout type.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub(crate) kind: u8,
    /// The version of the protocol running.
    pub(crate) version: u8,
    /// The human friendly name of the network.
    #[packable(unpack_error_with = |err| Error::InvalidNetworkName(err.into_item_err()))]
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string_prefix"))]
    #[getset(skip)]
    pub(crate) network_name: StringPrefix<u8>,
    /// The HRP prefix used for Bech32 addresses in the network.
    pub(crate) bech32_hrp: Hrp,
    /// The rent structure used by given node/network.
    pub(crate) rent_structure: RentStructure,
    /// The work score structure used by the node/network.
    pub(crate) work_score_structure: WorkScoreStructure,
    /// TokenSupply defines the current token supply on the network.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) token_supply: u64,
    /// Genesis timestamp at which the slots start to count.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) genesis_unix_timestamp: u64,
    /// Duration of each slot in seconds.
    pub(crate) slot_duration_in_seconds: u8,
    /// The number of slots in an epoch expressed as an exponent of 2.
    pub(crate) slots_per_epoch_exponent: u8,
    /// The parameters used for mana calculations.
    #[getset(skip)]
    pub(crate) mana_structure: ManaStructure,
    /// The unbonding period in epochs before an account can stop staking.
    pub(crate) staking_unbonding_period: EpochIndex,
    /// The number of validation blocks that each validator should issue each slot.
    pub(crate) validation_blocks_per_slot: u16,
    /// The number of epochs worth of Mana that a node is punished with for each additional validation block it issues.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) punishment_epochs: u64,
    /// The slot index used by tip-selection to determine if a block is eligible by evaluating issuing times
    /// and commitments in its past-cone against accepted tangle time and last committed slot respectively.
    pub(crate) liveness_threshold: SlotIndex,
    /// Minimum age relative to the accepted tangle time slot index that a slot can be committed.
    pub(crate) min_committable_age: SlotIndex,
    /// Maximum age for a slot commitment to be included in a block relative to the slot index of the block issuing
    /// time.
    pub(crate) max_committable_age: SlotIndex,
    /// The slot index used by the epoch orchestrator to detect the slot that should trigger a new
    /// committee selection for the next and upcoming epoch.
    pub(crate) epoch_nearing_threshold: SlotIndex,
    /// Parameters used to calculate the Reference Mana Cost (RMC).
    pub(crate) congestion_control_parameters: CongestionControlParameters,
    /// Defines the parameters used to signal a protocol parameters upgrade.
    pub(crate) version_signaling: VersionSignalingParameters,
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
            kind: 0,
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
            mana_structure: Default::default(),
            staking_unbonding_period: 10.into(),
            validation_blocks_per_slot: 10,
            punishment_epochs: 9,
            liveness_threshold: 5.into(),
            min_committable_age: 10.into(),
            max_committable_age: 20.into(),
            congestion_control_parameters: Default::default(),
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
        genesis_unix_timestamp: u64,
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

    /// Returns the parameters used for mana calculations.
    pub fn mana_structure(&self) -> &ManaStructure {
        &self.mana_structure
    }

    /// Returns the slots per epoch of the [`ProtocolParameters`].
    pub fn slots_per_epoch(&self) -> u64 {
        2_u64.pow(self.slots_per_epoch_exponent() as u32)
    }

    /// Gets a [`SlotIndex`] from a unix timestamp.
    pub fn slot_index(&self, timestamp: u64) -> SlotIndex {
        SlotIndex::from_timestamp(
            timestamp,
            self.genesis_unix_timestamp(),
            self.slot_duration_in_seconds(),
        )
    }

    /// Returns the hash of the [`ProtocolParameters`].
    pub fn hash(&self) -> ProtocolParametersHash {
        ProtocolParametersHash::new(Blake2b256::digest(self.pack_to_vec()).into())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
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
    min_strong_parents_threshold: u8,
}

impl Default for WorkScoreStructure {
    fn default() -> Self {
        Self {
            data_byte: 0,
            block: 100,
            missing_parent: 500,
            input: 20,
            context_input: 20,
            output: 20,
            native_token: 20,
            staking: 100,
            block_issuer: 100,
            allotment: 100,
            signature_ed25519: 200,
            min_strong_parents_threshold: 4,
        }
    }
}

/// Defines the parameters used to calculate the Reference Mana Cost (RMC).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[packable(unpack_error = Error)]
#[getset(get_copy = "pub")]
pub struct CongestionControlParameters {
    /// Minimum value of the reference Mana cost.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    min_reference_mana_cost: u64,
    /// Increase step size of the RMC.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    increase: u64,
    /// Decrease step size of the RMC.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    decrease: u64,
    /// Threshold for increasing the RMC.
    increase_threshold: u32,
    /// Threshold for decreasing the RMC.
    decrease_threshold: u32,
    /// Rate at which the scheduler runs (in workscore units per second).
    scheduler_rate: u32,
    /// Minimum amount of Mana that an account must have to schedule a block.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    min_mana: u64,
    /// Maximum size of the buffer in the scheduler.
    max_buffer_size: u32,
    /// Maximum number of blocks in the validation buffer.
    max_validation_buffer_size: u32,
}

impl Default for CongestionControlParameters {
    fn default() -> Self {
        Self {
            min_reference_mana_cost: 500,
            increase: 500,
            decrease: 500,
            increase_threshold: 800000,
            decrease_threshold: 500000,
            scheduler_rate: 100000,
            min_mana: 1,
            max_buffer_size: 3276800,
            max_validation_buffer_size: 100,
        }
    }
}

/// Defines the parameters used to signal a protocol parameters upgrade.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[packable(unpack_error = Error)]
#[getset(get_copy = "pub")]
pub struct VersionSignalingParameters {
    /// The size of the window in epochs that is used to find which version of protocol parameters was
    /// most signaled, from `current_epoch - window_size` to `current_epoch`.
    window_size: u8,
    /// The number of supporters required for a version to win within a `window_size`.
    window_target_ratio: u8,
    /// The offset in epochs required to activate the new version of protocol parameters.
    activation_offset: u8,
}

impl Default for VersionSignalingParameters {
    fn default() -> Self {
        Self {
            window_size: 7,
            window_target_ratio: 5,
            activation_offset: 7,
        }
    }
}

/// Returns a [`ProtocolParameters`] for testing purposes.
#[cfg(any(feature = "test", feature = "rand"))]
pub fn protocol_parameters() -> ProtocolParameters {
    ProtocolParameters::new(
        2,
        "testnet",
        "rms",
        crate::types::block::output::RentStructure::new(500, 1, 10, 1, 1, 1),
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
