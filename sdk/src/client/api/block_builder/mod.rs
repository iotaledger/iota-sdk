// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod input_selection;
pub mod transaction;

pub use self::transaction::verify_semantic;
use crate::{
    client::{constants::FIVE_MINUTES_IN_NANOSECONDS, ClientInner, Error, Result},
    types::block::{
        core::{BlockHeader, UnsignedBlock},
        output::AccountId,
        payload::Payload,
        BlockBody,
    },
};

impl ClientInner {
    pub async fn build_basic_block(&self, issuer_id: AccountId, payload: Option<Payload>) -> Result<UnsignedBlock> {
        let issuance = self.get_issuance().await?;

        let issuing_time = {
            #[cfg(feature = "std")]
            let issuing_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos() as u64;
            // TODO no_std way to have a nanosecond timestamp
            // https://github.com/iotaledger/iota-sdk/issues/647
            #[cfg(not(feature = "std"))]
            let issuing_time = 0;

            // Check that the issuing_time is in the range of +-5 minutes of the node to prevent potential issues
            if !(issuance.latest_parent_block_issuing_time - FIVE_MINUTES_IN_NANOSECONDS
                ..issuance.latest_parent_block_issuing_time + FIVE_MINUTES_IN_NANOSECONDS)
                .contains(&issuing_time)
            {
                return Err(Error::TimeNotSynced {
                    current_time: issuing_time,
                    tangle_time: issuance.latest_parent_block_issuing_time,
                });
            }
            // If timestamp is below latest_parent_block_issuing_time, just increase it to that +1 so the block doesn't
            // get rejected
            issuing_time.max(issuance.latest_parent_block_issuing_time + 1)
        };

        let protocol_params = self.get_protocol_parameters().await?;

        Ok(UnsignedBlock::new(
            BlockHeader::new(
                protocol_params.version(),
                protocol_params.network_id(),
                issuing_time,
                issuance.latest_commitment.id(),
                issuance.latest_finalized_slot,
                issuer_id,
            ),
            BlockBody::build_basic(
                issuance.strong_parents()?,
                (
                    protocol_params.work_score_parameters,
                    issuance.latest_commitment.reference_mana_cost(),
                ),
            )
            .with_weak_parents(issuance.weak_parents()?)
            .with_shallow_like_parents(issuance.shallow_like_parents()?)
            .with_payload(payload)
            .finish_block_body()?,
        ))
    }
}
