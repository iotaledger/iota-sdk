// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::CopyGetters;
use packable::{prefix::BoxedSlicePrefix, Packable};

use crate::types::block::{
    protocol::ProtocolParameters,
    slot::{EpochIndex, SlotIndex},
    Error,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[packable(unpack_error = Error)]
#[getset(get_copy = "pub")]
pub struct ManaParameters {
    /// The number of bits used to represent Mana.
    pub(crate) bits_count: u8,
    /// The amount of potential Mana generated by 1 microIOTA in 1 slot multiplied by 2^generation_rate_exponent.
    pub(crate) generation_rate: u8,
    /// The scaling of `generation_rate` expressed as an exponent of 2.
    /// The actual generation rate of Mana is given by generation_rate * 2^(-generation_rate_exponent).
    pub(crate) generation_rate_exponent: u8,
    /// A lookup table of epoch index diff to mana decay factor.
    /// The actual decay factor is given by decay_factors\[epoch_diff\] * 2^(-decay_factors_exponent).
    #[packable(unpack_error_with = |_| Error::InvalidManaDecayFactors)]
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::boxed_slice_prefix"))]
    #[getset(skip)]
    pub(crate) decay_factors: BoxedSlicePrefix<u32, u16>,
    /// The scaling of `decay_factors` expressed as an exponent of 2.
    pub(crate) decay_factors_exponent: u8,
    /// An integer approximation of the sum of decay over epochs.
    pub(crate) decay_factor_epochs_sum: u32,
    /// The scaling of `decay_factor_epochs_sum` expressed as an exponent of 2.
    pub(crate) decay_factor_epochs_sum_exponent: u8,
    // Decay factor for 1 year.
    pub(crate) annual_decay_factor_percentage: u8,
}

impl ManaParameters {
    /// Returns the mana decay factors slice.
    pub fn decay_factors(&self) -> &[u32] {
        &self.decay_factors
    }

    /// Returns the mana decay factor for the given epoch index.
    pub fn decay_factor_at(&self, epoch_index: impl Into<EpochIndex>) -> Option<u32> {
        self.decay_factors.get(*epoch_index.into() as usize).copied()
    }

    /// Returns the max mana that can exist with the mana bits defined.
    pub fn max_mana(&self) -> u64 {
        (1 << self.bits_count) - 1
    }

    fn decay(&self, mut mana: u64, epoch_delta: u32) -> u64 {
        if mana == 0 || epoch_delta == 0 || self.decay_factors().is_empty() {
            return mana;
        }

        // we keep applying the lookup table factors as long as n epochs are left
        let mut remaining_epochs = epoch_delta;

        while remaining_epochs > 0 {
            let epochs_to_decay = remaining_epochs.min(self.decay_factors().len() as u32);
            remaining_epochs -= epochs_to_decay;

            // Unwrap: Safe because the index is at most the length
            let decay_factor = self.decay_factor_at(epochs_to_decay - 1).unwrap();

            // apply the decay using fixed-point arithmetics
            mana = fixed_point_multiply(mana, decay_factor, self.decay_factors_exponent());
        }

        mana
    }

    fn generate_mana(&self, amount: u64, slot_delta: u32) -> u64 {
        if self.generation_rate() == 0 || slot_delta == 0 {
            return 0;
        }

        fixed_point_multiply(
            amount,
            slot_delta * self.generation_rate() as u32,
            self.generation_rate_exponent(),
        )
    }
}

impl Default for ManaParameters {
    fn default() -> Self {
        // TODO: use actual values
        Self {
            bits_count: 10,
            generation_rate: Default::default(),
            generation_rate_exponent: Default::default(),
            decay_factors: Default::default(),
            decay_factors_exponent: Default::default(),
            decay_factor_epochs_sum: Default::default(),
            decay_factor_epochs_sum_exponent: Default::default(),
            annual_decay_factor_percentage: Default::default(),
        }
    }
}

impl ProtocolParameters {
    /// Applies mana decay to the given mana.
    pub fn mana_with_decay(
        &self,
        mana: u64,
        slot_index_created: impl Into<SlotIndex>,
        slot_index_target: impl Into<SlotIndex>,
    ) -> Result<u64, Error> {
        let (slot_index_created, slot_index_target) = (slot_index_created.into(), slot_index_target.into());
        let (epoch_index_created, epoch_index_target) = (
            self.epoch_index_of(slot_index_created),
            self.epoch_index_of(slot_index_target),
        );

        if epoch_index_created > epoch_index_target {
            return Err(Error::InvalidEpochDelta {
                created: epoch_index_created,
                target: epoch_index_target,
            });
        }

        Ok(self
            .mana_parameters()
            .decay(mana, epoch_index_target.0 - epoch_index_created.0))
    }

    /// Applies mana decay to the given stored mana.
    pub fn rewards_with_decay(
        &self,
        reward: u64,
        reward_epoch: impl Into<EpochIndex>,
        claimed_epoch: impl Into<EpochIndex>,
    ) -> Result<u64, Error> {
        let (reward_epoch, claimed_epoch) = (reward_epoch.into(), claimed_epoch.into());

        if reward_epoch > claimed_epoch {
            return Err(Error::InvalidEpochDelta {
                created: reward_epoch,
                target: claimed_epoch,
            });
        }

        Ok(self.mana_parameters().decay(reward, claimed_epoch.0 - reward_epoch.0))
    }

    /// Calculates the potential mana that is generated by holding `amount` tokens from `slot_index_created` to
    /// `slot_index_target` and applies the decay to the result
    pub fn potential_mana(
        &self,
        amount: u64,
        slot_index_created: impl Into<SlotIndex>,
        slot_index_target: impl Into<SlotIndex>,
    ) -> Result<u64, Error> {
        let (slot_index_created, slot_index_target) = (slot_index_created.into(), slot_index_target.into());
        let (epoch_index_created, epoch_index_target) = (
            self.epoch_index_of(slot_index_created),
            self.epoch_index_of(slot_index_target),
        );

        if epoch_index_created > epoch_index_target {
            return Err(Error::InvalidEpochDelta {
                created: epoch_index_created,
                target: epoch_index_target,
            });
        }
        if slot_index_created >= slot_index_target {
            return Ok(0);
        }

        let mana_parameters = self.mana_parameters();

        Ok(if epoch_index_created == epoch_index_target {
            mana_parameters.generate_mana(amount, slot_index_target.0 - slot_index_created.0)
        } else if epoch_index_target == epoch_index_created + 1 {
            let slots_before_next_epoch = self.first_slot_of(epoch_index_created + 1) - slot_index_created;
            let slots_since_epoch_start = slot_index_target - self.first_slot_of(epoch_index_target);
            let mana_decayed =
                mana_parameters.decay(mana_parameters.generate_mana(amount, slots_before_next_epoch.0), 1);
            let mana_generated = mana_parameters.generate_mana(amount, slots_since_epoch_start.0);
            mana_decayed + mana_generated
        } else {
            let c = fixed_point_multiply(
                amount,
                mana_parameters.decay_factor_epochs_sum() * mana_parameters.generation_rate() as u32,
                mana_parameters.decay_factor_epochs_sum_exponent() + mana_parameters.generation_rate_exponent()
                    - self.slots_per_epoch_exponent(),
            );
            let epoch_delta = epoch_index_target.0 - epoch_index_created.0;
            let slots_before_next_epoch = self.first_slot_of(epoch_index_created + 1) - slot_index_created;
            let slots_since_epoch_start = slot_index_target - self.first_slot_of(epoch_index_target);
            let potential_mana_n = mana_parameters.decay(
                mana_parameters.generate_mana(amount, slots_before_next_epoch.0),
                epoch_delta,
            );
            let potential_mana_n_1 = mana_parameters.decay(c, epoch_delta - 1);
            let potential_mana_0 = c + mana_parameters.generate_mana(amount, slots_since_epoch_start.0)
                - (c >> mana_parameters.decay_factors_exponent());
            potential_mana_0 - potential_mana_n_1 + potential_mana_n
        })
    }
}

/// Perform a multiplication and shift.
const fn fixed_point_multiply(value: u64, mult_factor: u32, shift_factor: u8) -> u64 {
    ((value as u128 * mult_factor as u128) >> shift_factor) as u64
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    // Tests from https://github.com/iotaledger/iota.go/blob/develop/mana_decay_provider_test.go

    const BETA_PER_YEAR: f64 = 1. / 3.;

    fn params() -> &'static ProtocolParameters {
        use once_cell::sync::Lazy;
        static PARAMS: Lazy<ProtocolParameters> = Lazy::new(|| {
            let mut params = ProtocolParameters {
                slots_per_epoch_exponent: 13,
                slot_duration_in_seconds: 10,
                mana_parameters: ManaParameters {
                    bits_count: 63,
                    generation_rate: 1,
                    generation_rate_exponent: 27,
                    decay_factors_exponent: 32,
                    decay_factor_epochs_sum_exponent: 20,
                    ..Default::default()
                },
                ..Default::default()
            };
            params.mana_parameters.decay_factors = {
                let epochs_per_year = ((365_u64 * 24 * 60 * 60) as f64 / params.slot_duration_in_seconds() as f64)
                    / params.slots_per_epoch() as f64;
                let beta_per_epoch_index = BETA_PER_YEAR / epochs_per_year;
                (1..=epochs_per_year.floor() as usize)
                    .map(|epoch| {
                        ((-beta_per_epoch_index * epoch as f64).exp()
                            * (params.mana_parameters().decay_factors_exponent() as f64).exp2())
                        .floor() as u32
                    })
                    .collect::<Box<[_]>>()
            }
            .try_into()
            .unwrap();
            params.mana_parameters.decay_factor_epochs_sum = {
                let delta = params.slots_per_epoch() as f64 * params.slot_duration_in_seconds() as f64
                    / (365_u64 * 24 * 60 * 60) as f64;
                (((-BETA_PER_YEAR * delta).exp() / (1. - (-BETA_PER_YEAR * delta).exp()))
                    * (params.mana_parameters().decay_factor_epochs_sum_exponent() as f64).exp2())
                .floor() as u32
            };
            params
        });
        &PARAMS
    }

    #[test]
    fn mana_decay_no_factors() {
        let mana_parameters = ManaParameters {
            decay_factors: Box::<[_]>::default().try_into().unwrap(),
            ..Default::default()
        };
        assert_eq!(mana_parameters.decay(100, 100), 100);
    }

    #[test]
    fn mana_decay_no_delta() {
        assert_eq!(
            params().mana_with_decay(100, params().first_slot_of(1), params().first_slot_of(1)),
            Ok(100)
        );
    }

    #[test]
    fn mana_decay_no_mana() {
        assert_eq!(
            params().mana_with_decay(0, params().first_slot_of(1), params().first_slot_of(400)),
            Ok(0)
        );
    }

    #[test]
    fn mana_decay_negative_delta() {
        assert_eq!(
            params().mana_with_decay(100, params().first_slot_of(2), params().first_slot_of(1)),
            Err(Error::InvalidEpochDelta {
                created: 2.into(),
                target: 1.into()
            })
        );
    }

    #[test]
    fn mana_decay_lookup_len_delta() {
        assert_eq!(
            params().mana_with_decay(
                u64::MAX,
                params().first_slot_of(1),
                params().first_slot_of(params().mana_parameters().decay_factors().len() as u32 + 1)
            ),
            Ok(13228672242897911807)
        );
    }

    #[test]
    fn mana_decay_lookup_len_delta_multiple() {
        assert_eq!(
            params().mana_with_decay(
                u64::MAX,
                params().first_slot_of(1),
                params().first_slot_of(3 * params().mana_parameters().decay_factors().len() as u32 + 1)
            ),
            Ok(6803138682699798504)
        );
    }

    #[test]
    fn mana_decay_max_mana() {
        assert_eq!(
            params().mana_with_decay(u64::MAX, params().first_slot_of(1), params().first_slot_of(401)),
            Ok(13046663022640287317)
        );
    }

    #[test]
    fn potential_mana_no_delta() {
        assert_eq!(
            params().potential_mana(100, params().first_slot_of(1), params().first_slot_of(1)),
            Ok(0)
        );
    }

    #[test]
    fn potential_mana_no_mana() {
        assert_eq!(
            params().potential_mana(0, params().first_slot_of(1), params().first_slot_of(400)),
            Ok(0)
        );
    }

    #[test]
    fn potential_mana_negative_delta() {
        assert_eq!(
            params().potential_mana(100, params().first_slot_of(2), params().first_slot_of(1)),
            Err(Error::InvalidEpochDelta {
                created: 2.into(),
                target: 1.into()
            })
        );
    }

    #[test]
    fn potential_mana_lookup_len_delta() {
        assert_eq!(
            params().potential_mana(
                i64::MAX as u64,
                params().first_slot_of(1),
                params().first_slot_of(params().mana_parameters().decay_factors().len() as u32 + 1)
            ),
            Ok(183827295065703076)
        );
    }

    #[test]
    fn potential_mana_lookup_len_delta_multiple() {
        assert_eq!(
            params().potential_mana(
                i64::MAX as u64,
                params().first_slot_of(1),
                params().first_slot_of(3 * params().mana_parameters().decay_factors().len() as u32 + 1)
            ),
            Ok(410192223115924783)
        );
    }

    #[test]
    fn potential_mana_same_epoch() {
        assert_eq!(
            params().potential_mana(i64::MAX as u64, params().first_slot_of(1), params().last_slot_of(1)),
            Ok(562881233944575)
        );
    }

    #[test]
    fn potential_mana_one_epoch() {
        assert_eq!(
            params().potential_mana(i64::MAX as u64, params().first_slot_of(1), params().last_slot_of(2)),
            Ok(1125343946211326)
        );
    }

    #[test]
    fn potential_mana_several_epochs() {
        assert_eq!(
            params().potential_mana(i64::MAX as u64, params().first_slot_of(1), params().last_slot_of(3)),
            Ok(1687319824887185)
        );
    }

    #[test]
    fn potential_mana_max_mana() {
        assert_eq!(
            params().potential_mana(i64::MAX as u64, params().first_slot_of(1), params().first_slot_of(401)),
            Ok(190239292388858706)
        );
    }
}
