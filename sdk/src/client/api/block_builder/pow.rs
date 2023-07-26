// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! PoW functions.

#[cfg(not(target_family = "wasm"))]
use crate::pow::miner::{Miner, MinerBuilder, MinerCancel};
#[cfg(target_family = "wasm")]
use crate::pow::wasm_miner::{SingleThreadedMiner, SingleThreadedMinerBuilder};
use crate::{
    client::{ClientInner, Error, Result},
    types::block::{
        basic::BasicBlock, parent::StrongParents, payload::Payload, Block, BlockBuilder, Error as BlockError,
    },
};

impl ClientInner {
    /// Finishes the block with local PoW if needed.
    /// Without local PoW, it will finish the block with a 0 nonce.
    pub async fn finish_basic_block_builder(
        &self,
        strong_parents: Option<StrongParents>,
        payload: Option<Payload>,
    ) -> Result<Block> {
        if self.get_local_pow().await {
            self.finish_pow(strong_parents, payload).await
        } else {
            // Finish block without doing PoW.
            let strong_parents = match strong_parents {
                Some(strong_parents) => strong_parents,
                None => StrongParents::from_vec(self.get_tips().await?)?,
            };

            #[cfg(feature = "std")]
            let issuing_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos() as u64;
            // TODO no_std way to have a nanosecond timestamp
            // https://github.com/iotaledger/iota-sdk/issues/647
            #[cfg(not(feature = "std"))]
            let issuing_time = 0;

            let node_info = self.get_info().await?.node_info;
            let latest_finalized_slot = node_info.status.latest_finalized_slot;
            let slot_commitment_id = self.get_slot_commitment_by_index(latest_finalized_slot).await?.id();

            let signature = todo!();

            Ok(Block::build_basic(
                self.get_network_id().await?,
                issuing_time,
                slot_commitment_id,
                latest_finalized_slot,
                node_info.issuer_id,
                strong_parents,
                signature,
            )
            .with_payload(payload)
            .finish()?)
        }
    }

    /// Calls the appropriate PoW function depending whether the compilation is for wasm or not.
    pub async fn finish_pow(&self, strong_parents: Option<StrongParents>, payload: Option<Payload>) -> Result<Block> {
        #[cfg(not(target_family = "wasm"))]
        let block = self.finish_multi_threaded_pow(strong_parents, payload).await?;
        #[cfg(target_family = "wasm")]
        let block = self.finish_single_threaded_pow(strong_parents, payload).await?;

        Ok(block)
    }

    /// Performs multi-threaded proof-of-work.
    ///
    /// Always fetches new tips after each tips interval elapses if no strong parents are provided.
    #[cfg(not(target_family = "wasm"))]
    async fn finish_multi_threaded_pow(
        &self,
        strong_parents: Option<StrongParents>,
        payload: Option<Payload>,
    ) -> Result<Block> {
        let pow_worker_count = *self.pow_worker_count.read().await;
        let min_pow_score = self.get_min_pow_score().await?;
        let tips_interval = self.get_tips_interval().await;

        loop {
            let cancel = MinerCancel::new();
            let cancel_2 = cancel.clone();
            let payload_ = payload.clone();
            let strong_parents = match &strong_parents {
                Some(strong_parents) => strong_parents.clone(),
                None => StrongParents::from_vec(self.get_tips().await?)?,
            };
            let time_thread = std::thread::spawn(move || Ok(pow_timeout(tips_interval, cancel)));
            let pow_thread = std::thread::spawn(move || {
                let mut client_miner = MinerBuilder::new().with_cancel(cancel_2);
                if let Some(worker_count) = pow_worker_count {
                    client_miner = client_miner.with_num_workers(worker_count);
                }
                do_pow(client_miner.finish(), min_pow_score, payload_, strong_parents).map(Some)
            });

            for t in [pow_thread, time_thread] {
                match t.join().expect("failed to join threads.") {
                    Ok(block) => {
                        if let Some(block) = block {
                            return Ok(block);
                        }
                    }
                    Err(Error::Block(BlockError::NonceNotFound)) => {}
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }
    }

    /// Single threaded proof-of-work for Wasm, which cannot generally spawn the native threads used
    /// by the `ClientMiner`.
    ///
    /// Fetches new tips after each tips interval elapses if no strong parents are provided.
    #[cfg(target_family = "wasm")]
    async fn finish_single_threaded_pow(
        &self,
        strong_parents: Option<StrongParents>,
        payload: Option<Payload>,
    ) -> Result<Block> {
        let min_pow_score: u32 = self.get_min_pow_score().await?;
        let tips_interval: u64 = self.get_tips_interval().await;

        loop {
            let strong_parents = match &strong_parents {
                Some(strong_parents) => strong_parents.clone(),
                None => StrongParents::from_vec(self.get_tips().await?)?,
            };

            let single_threaded_miner = SingleThreadedMinerBuilder::new()
                .with_timeout_in_seconds(tips_interval)
                .finish();

            match do_pow(single_threaded_miner, min_pow_score, payload.clone(), strong_parents) {
                Ok(block) => {
                    return Ok(block);
                }
                Err(Error::Block(BlockError::NonceNotFound)) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}

/// Performs proof-of-work to construct a [`Block`].
fn do_pow(
    #[cfg(not(target_family = "wasm"))] miner: Miner,
    #[cfg(target_family = "wasm")] miner: SingleThreadedMiner,
    min_pow_score: u32,
    payload: Option<Payload>,
    strong_parents: StrongParents,
) -> Result<Block> {
    Ok(BlockBuilder::<BasicBlock>::new(strong_parents)
        .with_payload(payload)
        .finish_nonce(|bytes| miner.nonce(bytes, min_pow_score))?)
}

// PoW timeout, if we reach this we will restart the PoW with new tips, so the final block will never be lazy.
#[cfg(not(target_family = "wasm"))]
fn pow_timeout(after_seconds: u64, cancel: MinerCancel) -> Option<Block> {
    std::thread::sleep(std::time::Duration::from_secs(after_seconds));

    cancel.trigger();

    None
}
