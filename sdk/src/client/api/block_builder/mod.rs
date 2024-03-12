// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod options;
pub mod transaction;
pub mod transaction_builder;

use crate::{
    client::{constants::FIVE_MINUTES_IN_NANOSECONDS, Client, ClientError},
    types::block::{
        core::{BlockHeader, UnsignedBlock},
        output::AccountId,
        payload::Payload,
        BlockBody,
    },
};

impl Client {
    pub async fn build_basic_block(
        &self,
        issuer_id: AccountId,
        payload: impl Into<Option<Payload>> + Send,
    ) -> Result<UnsignedBlock, ClientError> {
        let issuance = self.get_issuance().await?;

        let issuing_time = {
            let issuing_time = instant::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos() as u64;

            // Check that the issuing_time is in the range of +-5 minutes of the node to prevent potential issues
            if !(issuance.latest_parent_block_issuing_time - FIVE_MINUTES_IN_NANOSECONDS
                ..issuance.latest_parent_block_issuing_time + FIVE_MINUTES_IN_NANOSECONDS)
                .contains(&issuing_time)
            {
                return Err(ClientError::TimeNotSynced {
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
            .with_payload(payload.into())
            .finish_block_body()?,
        ))
    }
}
