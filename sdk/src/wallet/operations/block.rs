// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "events")]
use crate::wallet::events::types::{TransactionProgressEvent, WalletEvent};
use crate::{
    client::{
        secret::{SecretManage, SignBlock},
        ClientError,
    },
    types::block::{output::AccountId, payload::Payload, BlockId},
    wallet::{Wallet, WalletError},
};

const MAX_POST_BLOCK_ATTEMPTS: u64 = 3;

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    pub(crate) async fn submit_basic_block(
        &self,
        payload: impl Into<Option<Payload>> + Send,
        issuer_id: impl Into<Option<AccountId>> + Send,
        allow_negative_bic: bool,
    ) -> Result<BlockId, WalletError> {
        log::debug!("submit_basic_block");
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        // If an issuer ID is provided, use it; otherwise, use the first available account or implicit account.
        let issuer_id = match issuer_id.into() {
            Some(id) => id,
            None => {
                let current_slot = self.client().get_slot_index().await?;

                self.ledger()
                    .await
                    .first_block_issuer_account_id(current_slot, protocol_parameters.network_id())
                    .ok_or(WalletError::AccountNotFound)?
            }
        };

        let unsigned_block = self.client().build_basic_block(issuer_id, payload).await?;

        if !allow_negative_bic {
            let work_score = protocol_parameters.work_score(unsigned_block.body.as_basic());
            let congestion = self.client().get_account_congestion(&issuer_id, work_score).await?;
            if (congestion.reference_mana_cost * work_score as u64) as i128 > congestion.block_issuance_credits {
                return Err(WalletError::InsufficientBic {
                    available: congestion.block_issuance_credits,
                    required: work_score as u64 * congestion.reference_mana_cost,
                });
            }
        }

        #[cfg(feature = "events")]
        self.emit(WalletEvent::TransactionProgress(
            TransactionProgressEvent::PreparedBlockSigningInput(prefix_hex::encode(unsigned_block.signing_input())),
        ))
        .await;

        let block = unsigned_block
            .sign_ed25519(
                &*self.secret_manager().read().await,
                self.bip_path().await.ok_or(WalletError::MissingBipPath)?,
            )
            .await?;

        #[cfg(feature = "events")]
        self.emit(WalletEvent::TransactionProgress(TransactionProgressEvent::Broadcasting))
            .await;

        log::debug!("submitting block {}", block.id(&protocol_parameters));
        log::debug!("submitting block {block:?}");

        let mut attempt = 1;
        loop {
            match self.client().post_block(&block).await {
                Ok(block_id) => break Ok(block_id),
                Err(err) => {
                    if attempt >= MAX_POST_BLOCK_ATTEMPTS {
                        return Err(err.into());
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(attempt)).await;
            attempt += 1;
        }
    }
}
