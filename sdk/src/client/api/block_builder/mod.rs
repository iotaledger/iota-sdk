// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod input_selection;
pub mod transaction;

use crypto::keys::bip44::Bip44;

pub use self::transaction::verify_semantic;
use crate::{
    client::{
        secret::{SecretManage, SignBlock},
        ClientInner, Result,
    },
    types::block::{
        core::{basic, BlockHeader, BlockWrapper},
        payload::Payload,
        Block, IssuerId,
    },
};

impl ClientInner {
    pub async fn build_basic_block<S: SecretManage>(
        &self,
        issuer_id: IssuerId,
        issuing_time: Option<u64>,
        strong_parents: Option<basic::StrongParents>,
        payload: Option<Payload>,
        secret_manager: &S,
        chain: Bip44,
    ) -> Result<BlockWrapper>
    where
        crate::client::Error: From<S::Error>,
    {
        let issuance = self.get_issuance().await?;
        let strong_parents = strong_parents.unwrap_or(issuance.strong_parents()?);

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

        BlockWrapper::build(
            BlockHeader::new(
                protocol_params.version(),
                protocol_params.network_id(),
                issuing_time,
                issuance.commitment.id(),
                issuance.latest_finalized_slot,
                issuer_id,
            ),
            Block::build_basic(strong_parents, 0) // TODO: burned mana calculation
                .with_weak_parents(issuance.weak_parents()?)
                .with_shallow_like_parents(issuance.shallow_like_parents()?)
                .with_payload(payload)
                .finish_block()?,
        )
        .sign_ed25519(secret_manager, chain)
        .await
    }
}
