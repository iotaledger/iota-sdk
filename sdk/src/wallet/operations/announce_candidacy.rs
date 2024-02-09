// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::secret::SecretManage,
    types::block::{
        output::AccountId,
        payload::{CandidacyAnnouncementPayload, Payload},
        BlockId,
    },
    wallet::{core::SecretData, Wallet},
};

impl<S: SecretManage> Wallet<SecretData<S>> {
    /// Announce a staking account's candidacy for the staking period.
    pub async fn announce_candidacy(&self, account_id: AccountId) -> crate::wallet::Result<BlockId> {
        self.submit_basic_block(
            Payload::CandidacyAnnouncement(CandidacyAnnouncementPayload),
            account_id,
            false,
        )
        .await
    }
}
