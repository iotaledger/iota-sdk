// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod work_score;

use alloc::string::String;
use core::borrow::Borrow;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use getset::{CopyGetters, Getters};
use packable::{prefix::StringPrefix, Packable, PackableExt};

pub use self::work_score::{WorkScore, WorkScoreParameters};
use crate::{
    types::block::{
        address::Hrp,
        helper::network_name_to_id,
        mana::{ManaParameters, RewardsParameters},
        output::StorageScoreParameters,
        slot::{EpochIndex, SlotIndex},
        Error, PROTOCOL_VERSION,
    },
    utils::ConvertTo,
};

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
    /// The storage score parameters used by given node/network.
    pub(crate) storage_score_parameters: StorageScoreParameters,
    /// The work score parameters used by the node/network.
    pub(crate) work_score_parameters: WorkScoreParameters,
    /// The parameters used for mana calculations.
    #[getset(skip)]
    pub(crate) mana_parameters: ManaParameters,
    /// TokenSupply defines the current token supply on the network.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) token_supply: u64,
    /// Defines the slot of the genesis.
    pub(crate) genesis_slot: u32,
    /// Genesis timestamp at which the slots start to count.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) genesis_unix_timestamp: u64,
    /// Duration of each slot in seconds.
    pub(crate) slot_duration_in_seconds: u8,
    /// The number of slots in an epoch expressed as an exponent of 2.
    pub(crate) slots_per_epoch_exponent: u8,
    /// The unbonding period in epochs before an account can stop staking.
    pub(crate) staking_unbonding_period: u32,
    /// The number of validation blocks that each validator should issue each slot.
    pub(crate) validation_blocks_per_slot: u8,
    /// The number of epochs worth of Mana that a node is punished with for each additional validation block it issues.
    pub(crate) punishment_epochs: u32,
    /// Used by tip-selection to determine if a block is eligible by evaluating issuing times.
    pub(crate) liveness_threshold_lower_bound: u16,
    /// Used by tip-selection to determine if a block is eligible by evaluating issuing times.
    pub(crate) liveness_threshold_upper_bound: u16,
    /// Minimum age relative to the accepted tangle time slot index that a slot can be committed.
    pub(crate) min_committable_age: u32,
    /// Maximum age for a slot commitment to be included in a block relative to the slot index of the block issuing
    /// time.
    pub(crate) max_committable_age: u32,
    /// Epoch Nearing Threshold is used by the epoch orchestrator to detect the slot that should trigger a new
    /// committee selection for the next and upcoming epoch.
    pub(crate) epoch_nearing_threshold: u32,
    /// Parameters used to calculate the Reference Mana Cost (RMC).
    pub(crate) congestion_control_parameters: CongestionControlParameters,
    /// Defines the parameters used to signal a protocol parameters upgrade.
    pub(crate) version_signaling_parameters: VersionSignalingParameters,
    /// Defines the parameters used for reward calculation.
    pub(crate) rewards_parameters: RewardsParameters,
    /// Defines the target size of the committee. If there's fewer candidates the actual committee size could be
    /// smaller in a given epoch.
    pub(crate) target_committee_size: u8,
    /// Defines the number of heavier slots that a chain needs to be ahead of the current chain to be considered for
    /// switching.
    pub(crate) chain_switching_threshold: u8,
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
            storage_score_parameters: Default::default(),
            work_score_parameters: Default::default(),
            token_supply: 1_813_620_509_061_365,
            genesis_slot: 0,
            genesis_unix_timestamp: 1582328545,
            slot_duration_in_seconds: 10,
            epoch_nearing_threshold: 20,
            slots_per_epoch_exponent: Default::default(),
            mana_parameters: Default::default(),
            staking_unbonding_period: 10,
            validation_blocks_per_slot: 10,
            punishment_epochs: 9,
            liveness_threshold_lower_bound: 15,
            liveness_threshold_upper_bound: 30,
            min_committable_age: 10,
            max_committable_age: 20,
            congestion_control_parameters: Default::default(),
            version_signaling_parameters: Default::default(),
            rewards_parameters: Default::default(),
            target_committee_size: 32,
            chain_switching_threshold: 3,
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
        storage_score_parameters: StorageScoreParameters,
        token_supply: u64,
        genesis_unix_timestamp: u64,
        slot_duration_in_seconds: u8,
        epoch_nearing_threshold: u32,
    ) -> Result<Self, Error> {
        Ok(Self {
            version,
            network_name: <StringPrefix<u8>>::try_from(network_name.into()).map_err(Error::InvalidStringPrefix)?,
            bech32_hrp: bech32_hrp.convert()?,
            storage_score_parameters,
            token_supply,
            genesis_unix_timestamp,
            slot_duration_in_seconds,
            epoch_nearing_threshold,
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
    pub fn mana_parameters(&self) -> &ManaParameters {
        &self.mana_parameters
    }

    /// Returns the slots per epoch of the [`ProtocolParameters`].
    pub fn slots_per_epoch(&self) -> u32 {
        2_u32.pow(self.slots_per_epoch_exponent() as u32)
    }

    /// Gets a [`SlotIndex`] from a unix timestamp in seconds.
    pub fn slot_index(&self, timestamp: u64) -> SlotIndex {
        SlotIndex::from_timestamp(
            timestamp,
            self.genesis_slot(),
            self.genesis_unix_timestamp(),
            self.slot_duration_in_seconds(),
        )
    }

    /// Gets the first [`SlotIndex`] of a given [`EpochIndex`].
    pub fn first_slot_of(&self, epoch_index: impl Into<EpochIndex>) -> SlotIndex {
        epoch_index.into().first_slot_index(self.slots_per_epoch_exponent())
    }

    /// Gets the last [`SlotIndex`] of a given [`EpochIndex`].
    pub fn last_slot_of(&self, epoch_index: impl Into<EpochIndex>) -> SlotIndex {
        epoch_index.into().last_slot_index(self.slots_per_epoch_exponent())
    }

    /// Gets the [`EpochIndex`] of a given [`SlotIndex`].
    pub fn epoch_index_of(&self, slot_index: impl Into<SlotIndex>) -> EpochIndex {
        EpochIndex::from_slot_index(slot_index.into(), self.slots_per_epoch_exponent())
    }

    /// Returns the hash of the [`ProtocolParameters`].
    pub fn hash(&self) -> ProtocolParametersHash {
        ProtocolParametersHash::new(Blake2b256::digest(self.pack_to_vec()).into())
    }

    /// Returns the [`CommittableAgeRange`].
    pub fn committable_age_range(&self) -> CommittableAgeRange {
        CommittableAgeRange {
            min: self.min_committable_age(),
            max: self.max_committable_age(),
        }
    }
}

/// Defines the age in which a block can be issued.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
pub struct CommittableAgeRange {
    /// Minimum age relative to the accepted tangle time slot index that a slot can be committed.
    pub min: u32,
    /// Maximum age for a slot commitment to be included in a block relative to the slot index of the block issuing
    /// time.
    pub max: u32,
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
        crate::types::block::output::StorageScoreParameters::new(500, 1, 10, 1, 1, 1),
        1_813_620_509_061_365,
        1582328545,
        10,
        20,
    )
    .unwrap()
}

crate::impl_id!(
    /// The hash of a [`ProtocolParameters`].
    pub ProtocolParametersHash {
        pub const LENGTH: usize = 32;
    }
);
