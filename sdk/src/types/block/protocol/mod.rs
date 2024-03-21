// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod error;
#[cfg(feature = "protocol_parameters_samples")]
mod samples;
mod work_score;

use core::borrow::Borrow;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use getset::{CopyGetters, Getters};
#[cfg(feature = "std")]
use instant::Duration;
use packable::{prefix::StringPrefix, Packable, PackableExt};
#[cfg(feature = "protocol_parameters_samples")]
pub use samples::{iota_mainnet_protocol_parameters, shimmer_mainnet_protocol_parameters};

pub use self::{
    error::ProtocolParametersError,
    work_score::{WorkScore, WorkScoreParameters},
};
use crate::types::block::{
    address::Hrp,
    helper::network_name_to_id,
    mana::{ManaParameters, RewardsParameters},
    output::{StorageScore, StorageScoreParameters},
    slot::{EpochIndex, SlotCommitmentId, SlotIndex},
};

/// Defines the parameters of the protocol at a particular version.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, Getters, CopyGetters)]
#[packable(unpack_error = ProtocolParametersError)]
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
    #[packable(unpack_error_with = |err| ProtocolParametersError::NetworkName(err.into_item_err()))]
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

impl ProtocolParameters {
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
        epoch_index
            .into()
            .first_slot_index(self.genesis_slot, self.slots_per_epoch_exponent())
    }

    /// Gets the last [`SlotIndex`] of a given [`EpochIndex`].
    pub fn last_slot_of(&self, epoch_index: impl Into<EpochIndex>) -> SlotIndex {
        epoch_index
            .into()
            .last_slot_index(self.genesis_slot, self.slots_per_epoch_exponent())
    }

    /// Calculates the number of slots before the next epoch.
    pub fn slots_before_next_epoch(&self, slot_index: impl Into<SlotIndex>) -> u32 {
        let slot_index = slot_index.into();

        if slot_index.0 < self.genesis_slot() {
            0
        } else {
            self.genesis_slot() + self.first_slot_of(self.epoch_index_of(slot_index) + 1).0 - slot_index.0
        }
    }

    /// Calculates the number of slots since the start of the epoch.
    pub fn slots_since_epoch_start(&self, slot_index: impl Into<SlotIndex>) -> u32 {
        let slot_index = slot_index.into();

        if slot_index.0 < self.genesis_slot() {
            0
        } else {
            self.genesis_slot() + slot_index.0 - self.first_slot_of(self.epoch_index_of(slot_index)).0
        }
    }

    #[cfg(feature = "std")]
    /// Calculates the number of slots in a duration.
    pub fn slots_in_duration(&self, duration: Duration) -> u32 {
        (duration.as_secs() / self.slot_duration_in_seconds() as u64) as u32
    }

    #[cfg(feature = "std")]
    /// Calculates the [`Duration`] of a number of slots.
    pub fn duration_of_slots(&self, slots: u32) -> Duration {
        Duration::from_secs((slots * self.slot_duration_in_seconds() as u32) as u64)
    }

    /// Gets the [`EpochIndex`] of a given [`SlotIndex`].
    pub fn epoch_index_of(&self, slot_index: impl Into<SlotIndex>) -> EpochIndex {
        EpochIndex::from_slot_index(slot_index, self.genesis_slot, self.slots_per_epoch_exponent())
    }

    /// Calculates the duration of an epoch in seconds.
    pub fn epoch_duration_in_seconds(&self) -> u64 {
        self.slot_duration_in_seconds() as u64 * self.slots_per_epoch() as u64
    }

    /// Calculates the number of epochs per year.
    pub fn epochs_per_year(&self) -> f64 {
        (365_u64 * 24 * 60 * 60) as f64 / self.epoch_duration_in_seconds() as f64
    }

    /// Calculates the decay per epoch based on the annual decay factor and number of epochs in a year.
    #[cfg(feature = "std")]
    pub fn decay_per_epoch(&self) -> f64 {
        self.mana_parameters()
            .annual_decay_factor()
            .powf(self.epochs_per_year().recip())
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

    /// Calculates the past bounded slot for the given slot of the SlotCommitment.
    /// Given any slot index of a commitment input, the result of this function is a slot index
    /// that is at least equal to the slot of the block in which it was issued, or higher.
    /// That means no commitment input can be chosen such that the index lies behind the slot index of the block,
    /// hence the past is bounded.
    pub fn past_bounded_slot(&self, slot_commitment_id: SlotCommitmentId) -> SlotIndex {
        slot_commitment_id.past_bounded_slot(self.max_committable_age())
    }

    /// Calculates the past bounded epoch for the given slot of the SlotCommitment.
    pub fn past_bounded_epoch(&self, slot_commitment_id: SlotCommitmentId) -> EpochIndex {
        self.epoch_index_of(self.past_bounded_slot(slot_commitment_id))
    }

    /// Calculates the future bounded slot for the given slot of the SlotCommitment.
    /// Given any slot index of a commitment input, the result of this function is a slot index
    /// that is at most equal to the slot of the block in which it was issued, or lower.
    /// That means no commitment input can be chosen such that the index lies ahead of the slot index of the block,
    /// hence the future is bounded.
    pub fn future_bounded_slot(&self, slot_commitment_id: SlotCommitmentId) -> SlotIndex {
        slot_commitment_id.future_bounded_slot(self.min_committable_age())
    }

    /// Calculates the future bounded epoch for the given slot of the SlotCommitment.
    pub fn future_bounded_epoch(&self, slot_commitment_id: SlotCommitmentId) -> EpochIndex {
        self.epoch_index_of(self.future_bounded_slot(slot_commitment_id))
    }

    /// Returns the slot at the end of which the validator and delegator registration ends and the voting power
    /// for the epoch is calculated.
    pub fn registration_slot(&self, epoch_index: EpochIndex) -> SlotIndex {
        epoch_index.registration_slot(
            self.genesis_slot(),
            self.slots_per_epoch_exponent(),
            self.epoch_nearing_threshold(),
        )
    }

    /// Gets the start epoch for a delegation with the given slot commitment id.
    pub fn delegation_start_epoch(&self, slot_commitment_id: SlotCommitmentId) -> EpochIndex {
        let past_bounded_slot = self.past_bounded_slot(slot_commitment_id);
        let past_bounded_epoch = self.epoch_index_of(past_bounded_slot);

        let registration_slot = self.registration_slot(past_bounded_epoch + 1);

        if past_bounded_slot <= registration_slot {
            past_bounded_epoch + 1
        } else {
            past_bounded_epoch + 2
        }
    }

    /// Gets the end epoch for a delegation with the given slot commitment id
    pub fn delegation_end_epoch(&self, slot_commitment_id: SlotCommitmentId) -> EpochIndex {
        let future_bounded_slot = self.future_bounded_slot(slot_commitment_id);
        let future_bounded_epoch = self.epoch_index_of(future_bounded_slot);

        let registration_slot = self.registration_slot(future_bounded_epoch + 1);

        if future_bounded_slot <= registration_slot {
            future_bounded_epoch
        } else {
            future_bounded_epoch + 1
        }
    }

    /// Get the storage score of a value.
    pub fn storage_score(&self, value: &impl StorageScore) -> u64 {
        value.storage_score(self.storage_score_parameters())
    }

    /// Get the work score of a value.
    pub fn work_score(&self, value: &impl WorkScore) -> u32 {
        value.work_score(self.work_score_parameters())
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
#[packable(unpack_error = ProtocolParametersError)]
#[getset(get_copy = "pub")]
pub struct CongestionControlParameters {
    /// Minimum value of the reference Mana cost.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) min_reference_mana_cost: u64,
    /// Increase step size of the RMC.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) increase: u64,
    /// Decrease step size of the RMC.
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) decrease: u64,
    /// Threshold for increasing the RMC.
    pub(crate) increase_threshold: u32,
    /// Threshold for decreasing the RMC.
    pub(crate) decrease_threshold: u32,
    /// Rate at which the scheduler runs (in workscore units per second).
    pub(crate) scheduler_rate: u32,
    /// Maximum size of the buffer in the scheduler.
    pub(crate) max_buffer_size: u32,
    /// Maximum number of blocks in the validation buffer.
    pub(crate) max_validation_buffer_size: u32,
}

/// Defines the parameters used to signal a protocol parameters upgrade.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[packable(unpack_error = ProtocolParametersError)]
#[getset(get_copy = "pub")]
pub struct VersionSignalingParameters {
    /// The size of the window in epochs that is used to find which version of protocol parameters was
    /// most signaled, from `current_epoch - window_size` to `current_epoch`.
    pub(crate) window_size: u8,
    /// The number of supporters required for a version to win within a `window_size`.
    pub(crate) window_target_ratio: u8,
    /// The offset in epochs required to activate the new version of protocol parameters.
    pub(crate) activation_offset: u8,
}

crate::impl_id!(
    /// The hash of a [`ProtocolParameters`].
    pub ProtocolParametersHash {
        pub const LENGTH: usize = 32;
    }
);
