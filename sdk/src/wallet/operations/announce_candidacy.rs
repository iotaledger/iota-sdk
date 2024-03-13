// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{secret::SecretManage, ClientError},
    types::block::{
        output::AccountId,
        payload::{CandidacyAnnouncementPayload, Payload},
        BlockId,
    },
    wallet::{Wallet, WalletError},
};

impl<S: 'static + SecretManage> Wallet<S>
where
    WalletError: From<S::Error>,
    ClientError: From<S::Error>,
{
    /// Announce a staking account's candidacy for the staking period.
    pub async fn announce_candidacy(&self, account_id: AccountId) -> Result<BlockId, WalletError> {
        self.submit_basic_block(
            Payload::CandidacyAnnouncement(CandidacyAnnouncementPayload),
            account_id,
            false,
        )
        .await
    }
}
