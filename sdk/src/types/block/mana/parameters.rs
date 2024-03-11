// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::CopyGetters;
use packable::{prefix::BoxedSlicePrefix, Packable};

use crate::types::block::{
    mana::ManaError,
    protocol::{ProtocolParameters, ProtocolParametersError},
    slot::{EpochIndex, SlotIndex},
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Packable, CopyGetters)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[packable(unpack_error = ProtocolParametersError)]
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
    #[packable(unpack_error_with = |_| ProtocolParametersError::ManaDecayFactors)]
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

    /// Returns the annual decay factor.
    pub fn annual_decay_factor(&self) -> f64 {
        self.annual_decay_factor_percentage() as f64 / 100.0
    }

    /// Returns the max mana that can exist with the mana bits defined.
    pub fn max_mana(&self) -> u64 {
        (1 << self.bits_count) - 1
    }

    fn decay(&self, mut mana: u64, epoch_diff: u32) -> u64 {
        if mana == 0 || epoch_diff == 0 || self.decay_factors().is_empty() {
            return mana;
        }

        // we keep applying the lookup table factors as long as n epochs are left
        let mut remaining_epochs = epoch_diff;

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

    fn generate_mana(&self, amount: u64, slot_diff: u32) -> u64 {
        if self.generation_rate() == 0 || slot_diff == 0 {
            return 0;
        }

        fixed_point_multiply(
            amount,
            slot_diff * self.generation_rate() as u32,
            self.generation_rate_exponent(),
        )
    }
}

impl ProtocolParameters {
    /// Applies mana decay to the given mana.
    pub fn mana_with_decay(
        &self,
        mana: u64,
        slot_index_created: impl Into<SlotIndex>,
        slot_index_target: impl Into<SlotIndex>,
    ) -> Result<u64, ManaError> {
        let (epoch_index_created, epoch_index_target) = (
            self.epoch_index_of(slot_index_created),
            self.epoch_index_of(slot_index_target),
        );

        if epoch_index_created > epoch_index_target {
            return Err(ManaError::EpochDiff {
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
    ) -> Result<u64, ManaError> {
        let (reward_epoch, claimed_epoch) = (reward_epoch.into(), claimed_epoch.into());

        if reward_epoch > claimed_epoch {
            return Err(ManaError::EpochDiff {
                created: reward_epoch,
                target: claimed_epoch,
            });
        }

        Ok(self.mana_parameters().decay(reward, claimed_epoch.0 - reward_epoch.0))
    }

    /// Calculates the potential mana that is generated by holding `amount` tokens from `slot_index_created` to
    /// `slot_index_target` and applies the decay to the result
    pub fn generate_mana_with_decay(
        &self,
        amount: u64,
        slot_index_created: impl Into<SlotIndex>,
        slot_index_target: impl Into<SlotIndex>,
    ) -> Result<u64, ManaError> {
        let (slot_index_created, slot_index_target) = (slot_index_created.into(), slot_index_target.into());
        let (epoch_index_created, epoch_index_target) = (
            self.epoch_index_of(slot_index_created),
            self.epoch_index_of(slot_index_target),
        );

        if epoch_index_created > epoch_index_target {
            return Err(ManaError::EpochDiff {
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
            let mana_generated_first_epoch =
                mana_parameters.generate_mana(amount, self.slots_before_next_epoch(slot_index_created));
            let mana_decayed_first_epoch = mana_parameters.decay(mana_generated_first_epoch, 1);
            let mana_generated_second_epoch =
                mana_parameters.generate_mana(amount, self.slots_since_epoch_start(slot_index_target));
            mana_decayed_first_epoch + mana_generated_second_epoch
        } else {
            let mana_generated_first_epoch =
                mana_parameters.generate_mana(amount, self.slots_before_next_epoch(slot_index_created));
            let mana_decayed_first_epoch =
                mana_parameters.decay(mana_generated_first_epoch, epoch_index_target.0 - epoch_index_created.0);
            let c = fixed_point_multiply(
                amount,
                mana_parameters.decay_factor_epochs_sum() * mana_parameters.generation_rate() as u32,
                mana_parameters.decay_factor_epochs_sum_exponent() + mana_parameters.generation_rate_exponent()
                    - self.slots_per_epoch_exponent(),
            );
            let mana_decayed_intermediate_epochs =
                c - mana_parameters.decay(c, epoch_index_target.0 - epoch_index_created.0 - 1);
            let mana_generated_last_epoch =
                mana_parameters.generate_mana(amount, self.slots_since_epoch_start(slot_index_target));
            mana_decayed_intermediate_epochs + mana_generated_last_epoch + mana_decayed_first_epoch
                - (c >> mana_parameters.decay_factors_exponent())
        })
    }

    pub fn slots_until_generated(
        &self,
        generation_amount: u64,
        stored_mana: u64,
        required_mana: u64,
    ) -> Result<u32, ManaError> {
        if required_mana == 0 {
            return Ok(0);
        }
        let mut num_slots = 0;
        let mana_generated_per_epoch = self
            .mana_parameters()
            .generate_mana(generation_amount, self.slots_per_epoch());
        if mana_generated_per_epoch == 0 {
            return Err(ManaError::InsufficientGenerationAmount);
        }
        let mut required_mana_remaining = required_mana;
        loop {
            // Get the minimum number of slots required to achieve the needed mana (i.e. not including decay)
            num_slots +=
                u32::try_from(1 + (required_mana_remaining * self.slots_per_epoch() as u64) / mana_generated_per_epoch)
                    .map_err(|_| ManaError::InsufficientGenerationAmount)?;
            // Get the actual values after than many slots
            let decayed_mana = stored_mana - self.mana_with_decay(stored_mana, 0, num_slots)?;
            let generated_mana = self.generate_mana_with_decay(generation_amount, 0, num_slots)?;
            // If we generated less than how much we lost, this is not going to work out
            if generated_mana <= decayed_mana {
                return Err(ManaError::InsufficientGenerationAmount);
            }
            if generated_mana - decayed_mana >= required_mana {
                return Ok(num_slots);
            } else {
                required_mana_remaining = required_mana + decayed_mana - generated_mana;
            }
        }
    }
}

/// Perform a multiplication and shift.
const fn fixed_point_multiply(value: u64, mult_factor: u32, shift_factor: u8) -> u64 {
    ((value as u128 * mult_factor as u128) >> shift_factor) as u64
}

#[cfg(all(test, feature = "protocol_parameters_samples"))]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::types::block::protocol::iota_mainnet_protocol_parameters;

    // Tests from https://github.com/iotaledger/iota.go/blob/develop/mana_decay_provider_test.go

    fn params() -> &'static ProtocolParameters {
        iota_mainnet_protocol_parameters()
    }

    #[test]
    fn mana_decay_no_factors() {
        let mut mana_parameters = params().mana_parameters().clone();
        mana_parameters.decay_factors = Box::<[_]>::default().try_into().unwrap();
        assert_eq!(mana_parameters.decay(100, 100), 100);
    }

    struct ManaDecayTest {
        name: &'static str,
        stored_mana: u64,
        created_slot: SlotIndex,
        target_slot: SlotIndex,
        err: Option<ManaError>,
    }

    #[test]
    fn mana_decay() {
        let tests = [
            ManaDecayTest {
                name: "check if mana decay works for 0 mana values",
                stored_mana: 0,
                created_slot: params().first_slot_of(1),
                target_slot: params().first_slot_of(400),
                err: None,
            },
            ManaDecayTest {
                name: "check if mana decay works for 0 slot index diffs",
                stored_mana: u64::MAX,
                created_slot: params().first_slot_of(1),
                target_slot: params().first_slot_of(1),
                err: None,
            },
            ManaDecayTest {
                name: "check for error if target index is lower than created index",
                stored_mana: 0,
                created_slot: params().first_slot_of(2),
                target_slot: params().first_slot_of(1),
                err: Some(ManaError::EpochDiff {
                    created: 2.into(),
                    target: 1.into(),
                }),
            },
            ManaDecayTest {
                name: "check if mana decay works for exactly the amount of epochs in the lookup table",
                stored_mana: u64::MAX,
                created_slot: params().first_slot_of(1),
                target_slot: params().first_slot_of(params().mana_parameters().decay_factors().len() as u32 + 1),
                err: None,
            },
            ManaDecayTest {
                name: "check if mana decay works for multiples of the available epochs in the lookup table",
                stored_mana: u64::MAX,
                created_slot: params().first_slot_of(1),
                target_slot: params().first_slot_of(3 * params().mana_parameters().decay_factors().len() as u32 + 1),
                err: None,
            },
            ManaDecayTest {
                name: "even with the highest possible uint64 number, the calculation should not overflow",
                stored_mana: u64::MAX,
                created_slot: params().first_slot_of(1),
                target_slot: params().first_slot_of(401),
                err: None,
            },
        ];

        for ManaDecayTest {
            name,
            stored_mana,
            created_slot,
            target_slot,
            err,
        } in tests
        {
            let result = params().mana_with_decay(stored_mana, created_slot, target_slot);
            if let Some(err) = err {
                assert_eq!(result, Err(err), "{name}");
            } else {
                let result = result.map_err(|e| format!("{name}: {e}")).unwrap();
                let upper_bound = upper_bound_mana_decay(stored_mana, created_slot, target_slot);
                let lower_bound = lower_bound_mana_decay(stored_mana, created_slot, target_slot);

                assert!(
                    result as f64 <= upper_bound,
                    "{name}: result {result} above upper bound {upper_bound}",
                );
                assert!(
                    result as f64 >= lower_bound,
                    "{name}: result {result} below lower bound {upper_bound}",
                );
            }
        }
    }

    struct ManaGenerationTest {
        name: &'static str,
        amount: u64,
        created_slot: SlotIndex,
        target_slot: SlotIndex,
        potential_mana: Option<u64>,
        err: Option<ManaError>,
    }

    #[test]
    fn mana_generation() {
        let tests = [
            ManaGenerationTest {
                name: "check if mana generation works for 0 mana values",
                amount: 0,
                created_slot: params().first_slot_of(1),
                target_slot: params().first_slot_of(400),
                potential_mana: None,
                err: None,
            },
            ManaGenerationTest {
                name: "check if mana generation works for 0 slot index diffs",
                amount: i64::MAX as _,
                created_slot: params().first_slot_of(1),
                target_slot: params().first_slot_of(1),
                potential_mana: None,
                err: None,
            },
            ManaGenerationTest {
                name: "check for error if target index is lower than created index",
                amount: 0,
                created_slot: params().first_slot_of(2),
                target_slot: params().first_slot_of(1),
                potential_mana: None,
                err: Some(ManaError::EpochDiff {
                    created: 2.into(),
                    target: 1.into(),
                }),
            },
            ManaGenerationTest {
                name: "check if mana generation works for exactly the amount of epochs in the lookup table",
                amount: params().token_supply(),
                created_slot: params().first_slot_of(1),
                target_slot: params().first_slot_of(params().mana_parameters().decay_factors().len() as u32 + 1),
                potential_mana: None,
                err: None,
            },
            ManaGenerationTest {
                name: "check if mana generation works for multiples of the available epochs in the lookup table",
                amount: params().token_supply(),
                created_slot: params().first_slot_of(1),
                target_slot: params().first_slot_of(3 * params().mana_parameters().decay_factors().len() as u32 + 1),
                potential_mana: None,
                err: None,
            },
            ManaGenerationTest {
                name: "check if mana generation works for 0 epoch diffs",
                amount: params().token_supply(),
                created_slot: params().first_slot_of(1),
                target_slot: params().last_slot_of(1),
                potential_mana: None,
                err: None,
            },
            ManaGenerationTest {
                name: "check if mana generation works for 1 epoch diffs",
                amount: params().token_supply(),
                created_slot: params().first_slot_of(1),
                target_slot: params().last_slot_of(2),
                potential_mana: None,
                err: None,
            },
            ManaGenerationTest {
                name: "check if mana generation works for >=2 epoch diffs",
                amount: params().token_supply(),
                created_slot: params().first_slot_of(1),
                target_slot: params().last_slot_of(3),
                potential_mana: None,
                err: None,
            },
            ManaGenerationTest {
                name: "check exact value mana generation 1000000000 mana slot 1 to 10000",
                amount: 1000000000,
                created_slot: SlotIndex(1),
                target_slot: SlotIndex(10000),
                potential_mana: Some(76228441),
                err: None,
            },
            ManaGenerationTest {
                name: "check exact value mana generation 1000000000 mana slot 9000 to 10000",
                amount: 1000000000,
                created_slot: SlotIndex(9000),
                target_slot: SlotIndex(10000),
                potential_mana: Some(7629394),
                err: None,
            },
            ManaGenerationTest {
                name: "check exact value mana generation 800000000000000000 mana slot 1 to 10000",
                amount: 800000000000000000,
                created_slot: SlotIndex(1),
                target_slot: SlotIndex(10000),
                potential_mana: Some(60982753715241244),
                err: None,
            },
            ManaGenerationTest {
                name: "check exact value mana generation 800000000000000000 mana slot 9000 to 10000",
                amount: 800000000000000000,
                created_slot: SlotIndex(9000),
                target_slot: SlotIndex(10000),
                potential_mana: Some(6103515625000000),
                err: None,
            },
        ];

        for ManaGenerationTest {
            name,
            amount,
            created_slot,
            target_slot,
            potential_mana,
            err,
        } in tests
        {
            let result = params().generate_mana_with_decay(amount, created_slot, target_slot);
            if let Some(err) = err {
                assert_eq!(result, Err(err), "{name}");
            } else {
                let result = result.map_err(|e| format!("{name}: {e}")).unwrap();
                if let Some(potential_mana) = potential_mana {
                    assert_eq!(result, potential_mana);
                } else {
                    let upper_bound = upper_bound_mana_generation(amount, created_slot, target_slot);
                    let lower_bound = lower_bound_mana_generation(amount, created_slot, target_slot);

                    assert!(
                        result as f64 <= upper_bound,
                        "{name}: result {result} above upper bound {upper_bound}",
                    );
                    assert!(
                        result as f64 >= lower_bound,
                        "{name}: result {result} below lower bound {upper_bound}",
                    );

                    if result != 0 {
                        let float_res = mana_generation_with_decay_float(amount as _, created_slot, target_slot);
                        let epsilon = 0.001;
                        let allowed_delta = float_res.abs().min(result as f64) * epsilon;
                        let dt = float_res - result as f64;
                        assert!(
                            dt >= -allowed_delta && dt <= allowed_delta,
                            "{name}: fixed point result varies too greatly from float result"
                        );
                    }
                }
            }
        }
    }

    fn mana_decay_float(mana: f64, creation_slot: SlotIndex, target_slot: SlotIndex) -> f64 {
        let (creation_epoch, target_epoch) = (
            params().epoch_index_of(creation_slot),
            params().epoch_index_of(target_slot),
        );
        mana * params()
            .decay_per_epoch()
            .powi((target_epoch.0 - creation_epoch.0) as _)
    }

    fn mana_generation_with_decay_float(amount: f64, creation_slot: SlotIndex, target_slot: SlotIndex) -> f64 {
        let (creation_epoch, target_epoch) = (
            params().epoch_index_of(creation_slot),
            params().epoch_index_of(target_slot),
        );
        let decay_per_epoch = params().decay_per_epoch();
        let generation_rate = params().mana_parameters().generation_rate() as f64
            * 2f64.powi(-(params().mana_parameters().generation_rate_exponent() as i32));

        if creation_epoch == target_epoch {
            (target_slot.0 - creation_slot.0) as f64 * amount * generation_rate
        } else if target_epoch == creation_epoch + 1 {
            let slots_before_next_epoch = params().slots_before_next_epoch(creation_slot);
            let slots_since_epoch_start = params().slots_since_epoch_start(target_slot);
            let mana_decayed = slots_before_next_epoch as f64 * amount * generation_rate * decay_per_epoch;
            let mana_generated = slots_since_epoch_start as f64 * amount * generation_rate;
            mana_decayed + mana_generated
        } else {
            let slots_before_next_epoch = params().slots_before_next_epoch(creation_slot);
            let slots_since_epoch_start = params().slots_since_epoch_start(target_slot);
            let c = decay_per_epoch * (1.0 - decay_per_epoch.powi((target_epoch.0 - creation_epoch.0) as i32 - 1))
                / (1.0 - decay_per_epoch);
            let potential_mana_n = slots_before_next_epoch as f64
                * amount
                * generation_rate
                * decay_per_epoch.powi((target_epoch.0 - creation_epoch.0) as i32);
            let potential_mana_n_1 = c * amount * generation_rate * params().slots_per_epoch() as f64;
            let potential_mana_0 = slots_since_epoch_start as f64 * amount * generation_rate;
            potential_mana_n + potential_mana_n_1 + potential_mana_0
        }
    }

    fn upper_bound_mana_decay(mana: u64, creation_slot: SlotIndex, target_slot: SlotIndex) -> f64 {
        mana_decay_float(mana as _, creation_slot, target_slot)
    }

    fn lower_bound_mana_decay(mana: u64, creation_slot: SlotIndex, target_slot: SlotIndex) -> f64 {
        mana_decay_float(mana as _, creation_slot, target_slot)
            - (mana as f64).mul_add(
                2f64.powi(-(params().mana_parameters().decay_factors_exponent() as i32)),
                1.0,
            )
    }

    fn upper_bound_mana_generation(amount: u64, creation_slot: SlotIndex, target_slot: SlotIndex) -> f64 {
        mana_generation_with_decay_float(amount as _, creation_slot, target_slot) + 2.0
            - 2f64.powi(-(params().mana_parameters().decay_factors_exponent() as i32) - 1)
    }

    fn lower_bound_mana_generation(amount: u64, creation_slot: SlotIndex, target_slot: SlotIndex) -> f64 {
        let decay_per_epoch = params().decay_per_epoch();
        let c = decay_per_epoch / (1.0 - decay_per_epoch);

        mana_generation_with_decay_float(amount as _, creation_slot, target_slot)
            - (amount as f64
                * params().mana_parameters().generation_rate() as f64
                * 2f64.powi(
                    params().slots_per_epoch_exponent() as i32
                        - params().mana_parameters().generation_rate_exponent() as i32,
                ))
            .mul_add(
                c.mul_add(
                    2f64.powi(-(params().mana_parameters().decay_factors_exponent() as i32)),
                    1.0,
                ),
                4.0,
            )
    }

    #[test]
    fn slots_until_generated() {
        let generation_amount = 100000;
        let stored_mana = 1000000;
        let required_mana = 50000;

        let slots_left = params()
            .slots_until_generated(generation_amount, stored_mana, required_mana)
            .unwrap();
        assert_eq!(
            params()
                .generate_mana_with_decay(generation_amount, 0, slots_left)
                .unwrap()
                + params().mana_with_decay(stored_mana, 0, slots_left).unwrap(),
            stored_mana + required_mana
        );

        let generation_amount = 500000;
        let stored_mana = 12345;
        let required_mana = 999999;

        let slots_left = params()
            .slots_until_generated(generation_amount, stored_mana, required_mana)
            .unwrap();
        assert_eq!(
            params()
                .generate_mana_with_decay(generation_amount, 0, slots_left)
                .unwrap()
                + params().mana_with_decay(stored_mana, 0, slots_left).unwrap(),
            stored_mana + required_mana
        );
    }

    #[test]
    fn slots_until_generated_insufficient_amount() {
        let generation_amount = 1000;
        let stored_mana = 1000000;
        let required_mana = 50000;

        let slots_left = params()
            .slots_until_generated(generation_amount, stored_mana, required_mana)
            .unwrap_err();
        assert_eq!(slots_left, ManaError::InsufficientGenerationAmount);
    }

    #[test]
    fn slots_until_generated_absurd_requirement() {
        let generation_amount = 100000;
        let stored_mana = 1000000;
        let required_mana = 500000000000;

        let slots_left = params()
            .slots_until_generated(generation_amount, stored_mana, required_mana)
            .unwrap_err();
        assert_eq!(slots_left, ManaError::InsufficientGenerationAmount);
    }
}
