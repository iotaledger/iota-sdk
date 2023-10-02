// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::Client,
    types::block::{
        protocol::{lower_bits, multiplication_and_shift, upper_bits},
        slot::SlotIndex,
    },
};

impl Client {
    /// Calculates the potential mana that is generated by holding `amount` tokens from `slot_index_created` to
    /// `slot_index_target` and applies the decay to the result
    pub async fn potential_mana(
        &self,
        amount: u64,
        slot_index_created: SlotIndex,
        slot_index_target: SlotIndex,
    ) -> crate::client::Result<u64> {
        if slot_index_created >= slot_index_target {
            return Ok(0);
        }
        let protocol_parameters_map = self.get_info().await?.node_info.protocol_parameters;
        let params: &crate::types::block::protocol::ProtocolParameters = &protocol_parameters_map.latest().parameters;
        let mana_structure = params.mana_structure();
        let slots_per_epoch_exponent = protocol_parameters_map
            .iter()
            .map(|r| (r.start_epoch, r.parameters.slots_per_epoch_exponent()))
            .collect::<Vec<_>>();
        let (epoch_index_created, epoch_index_target) = (
            slot_index_created.to_epoch_index(slots_per_epoch_exponent.iter().copied())?,
            slot_index_target.to_epoch_index(slots_per_epoch_exponent.iter().copied())?,
        );
        Ok(if epoch_index_created == epoch_index_target {
            mana_structure.generate_mana(amount, (*slot_index_target - *slot_index_created) as u32)
        } else if epoch_index_created == epoch_index_target - 1 {
            let slots_before_next_epoch = *slot_index_created
                - **(epoch_index_created + 1)
                    .slot_index_range(slots_per_epoch_exponent.iter().copied())?
                    .start();
            let slots_since_epoch_start = *slot_index_target
                - **(epoch_index_target - 1)
                    .slot_index_range(slots_per_epoch_exponent.iter().copied())?
                    .end();
            let mana_decayed =
                mana_structure.decay(mana_structure.generate_mana(amount, slots_before_next_epoch as u32), 1);
            let mana_generated = mana_structure.generate_mana(amount, slots_since_epoch_start as u32);
            mana_decayed + mana_generated
        } else {
            let c = {
                let amount_hi = upper_bits(amount);
                let amount_lo = lower_bits(amount);
                let (amount_hi, amount_lo) = multiplication_and_shift(
                    amount_hi,
                    amount_lo,
                    mana_structure.decay_factor_epochs_sum() * mana_structure.generation_rate() as u32,
                    mana_structure.decay_factor_epochs_sum_exponent() + mana_structure.generation_rate_exponent()
                        - params.slots_per_epoch_exponent(),
                );
                amount_hi << 32 | amount_lo
            };
            let slots_before_next_epoch = *slot_index_created
                - **(epoch_index_created + 1)
                    .slot_index_range(slots_per_epoch_exponent.iter().copied())?
                    .start();
            let slots_since_epoch_start = *slot_index_target
                - **(epoch_index_target - 1)
                    .slot_index_range(slots_per_epoch_exponent.iter().copied())?
                    .end();
            let potential_mana_n = mana_structure.decay(
                mana_structure.generate_mana(amount, slots_before_next_epoch as u32),
                *epoch_index_target - *epoch_index_created,
            );
            let potential_mana_n_1 = mana_structure.decay(c, *epoch_index_target - *epoch_index_created);
            let potential_mana_0 = c + mana_structure.generate_mana(amount, slots_since_epoch_start as u32)
                - (c >> mana_structure.generation_rate_exponent());
            potential_mana_0 - potential_mana_n_1 + potential_mana_n
        })
    }
}
