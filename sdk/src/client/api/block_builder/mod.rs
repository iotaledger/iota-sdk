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
            core::{basic, Block, BlockWrapper},
            payload::Payload,
            signature::Ed25519Signature,
            IssuerId,
        },
    },
};

impl ClientInner {
    pub async fn finish_basic_block_builder(
        &self,
        issuer_id: IssuerId,
        signature: Ed25519Signature,
        issuing_time: Option<u64>,
        strong_parents: Option<basic::StrongParents>,
        payload: Option<Payload>,
    ) -> Result<BlockWrapper> {
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

        let protocol_parameters = self.get_protocol_parameters().await?;

        Ok(BlockWrapper::new(
            protocol_parameters.version(),
            protocol_parameters.network_id(),
            issuing_time,
            commitment.id(),
            latest_finalized_slot,
            issuer_id,
            // TODO correct value for burned_mana
            Block::build_basic(strong_parents, 0)
                .with_weak_parents(weak_parents)
                .with_shallow_like_parents(shallow_like_parents)
                .with_payload(payload)
                .finish_block()?,
            signature,
        ))
    }
}
