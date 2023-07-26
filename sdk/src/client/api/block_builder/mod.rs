// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod input_selection;
pub mod transaction;

pub use self::transaction::verify_semantic;
use crate::{
    client::{ClientInner, Result},
    types::block::{parent::StrongParents, payload::Payload, Block},
};

impl ClientInner {
    pub async fn finish_basic_block_builder(
        &self,
        strong_parents: Option<StrongParents>,
        payload: Option<Payload>,
    ) -> Result<Block> {
        let strong_parents = match strong_parents {
            Some(strong_parents) => strong_parents,
            None => StrongParents::from_vec(self.get_tips().await?)?,
        };

        Ok(Block::build_basic(strong_parents).with_payload(payload).finish()?)
    }
}
