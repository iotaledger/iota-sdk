// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod input_selection;
pub mod transaction;

pub use self::transaction::verify_semantic;
use crate::{
    client::{ClientInner, Result},
    types::block::{
        core::{BasicBlockBody, BlockHeader, UnsignedBlock},
        output::AccountId,
        payload::Payload,
        protocol::{WorkScore, WorkScoreParameters},
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
            issuing_time
        };

        let protocol_params = self.get_protocol_parameters().await?;

        Ok(UnsignedBlock::new(
            BlockHeader::new(
                protocol_params.version(),
                protocol_params.network_id(),
                issuing_time,
                issuance.commitment.id(),
                issuance.latest_finalized_slot,
                issuer_id,
            ),
            BlockBody::build_basic(issuance.strong_parents()?)
                .with_weak_parents(issuance.weak_parents()?)
                .with_shallow_like_parents(issuance.shallow_like_parents()?)
                .with_payload(payload)
                .finish_block_body_with_minimum_mana_amount(
                    protocol_params.work_score_parameters,
                    issuance.commitment.reference_mana_cost(),
                )?,
        ))
    }
}
