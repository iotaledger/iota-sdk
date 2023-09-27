// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod input_selection;
pub mod transaction;

pub use self::transaction::verify_semantic;
use crate::{
    client::{ClientInner, Result},
    types::{
        api::core::response::IssuanceBlockHeaderResponse,
        block::{
            core::{BasicBlockBuilder, BlockBuilder, BlockWrapper},
            parent::StrongParents,
            payload::Payload,
            IssuerId,
        },
    },
};

impl ClientInner {
    pub async fn unsigned_basic_block_builder(
        &self,
        issuer_id: IssuerId,
        issuing_time: Option<u64>,
        strong_parents: Option<StrongParents>,
        payload: Option<Payload>,
    ) -> Result<BlockBuilder<BasicBlockBuilder>> {
        let IssuanceBlockHeaderResponse {
            strong_parents: default_strong_parents,
            weak_parents,
            shallow_like_parents,
            latest_finalized_slot,
            commitment,
        } = self.get_issuance().await?;
        let strong_parents = strong_parents.unwrap_or(default_strong_parents);

        let issuing_time = issuing_time.unwrap_or_else(|| {
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
        });

        let protocol_params = self.get_protocol_parameters().await?;

        Ok(BlockWrapper::build_basic(
            protocol_params.version(),
            protocol_params.network_id(),
            commitment.id(),
            latest_finalized_slot,
            issuer_id,
            strong_parents,
            0, // TODO: burned mana calculation
        )
        .with_issuing_time(issuing_time)
        .with_weak_parents(weak_parents)
        .with_shallow_like_parents(shallow_like_parents)
        .with_payload(payload))
    }
}
