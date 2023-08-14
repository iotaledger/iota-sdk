// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod input_selection;
pub mod transaction;

pub use self::transaction::verify_semantic;
use crate::{
    client::{ClientInner, Result},
    types::block::{
        basic::BasicBlockData, core::Block, parent::StrongParents, payload::Payload, BlockBuilder, IssuerId,
    },
};

impl ClientInner {
    pub async fn unsigned_basic_block_builder(
        &self,
        issuer_id: IssuerId,
        issuing_time: Option<u64>,
        strong_parents: Option<StrongParents>,
        payload: Option<Payload>,
    ) -> Result<BlockBuilder<BasicBlockData>> {
        // Use tips as strong parents if none are provided.
        let strong_parents = match strong_parents {
            Some(strong_parents) => strong_parents,
            None => StrongParents::from_vec(self.get_tips().await?)?,
        };

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

        let node_info = self.get_info().await?.node_info;
        let latest_finalized_slot = node_info.status.latest_finalized_slot;
        let slot_commitment_id = self.get_slot_commitment_by_index(latest_finalized_slot).await?.id();

        let builder = Block::build_basic(
            self.get_network_id().await?,
            issuing_time,
            slot_commitment_id,
            latest_finalized_slot,
            issuer_id,
            strong_parents,
        )
        .with_payload(payload);

        Ok(builder)
    }
}
