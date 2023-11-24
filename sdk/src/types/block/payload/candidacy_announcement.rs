// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::{Packable, PackableExt};

use crate::types::block::protocol::WorkScore;

/// A payload which is used to indicate candidacy for committee selection for the next epoch.
#[derive(Clone, Debug, Eq, PartialEq, Packable)]
pub struct CandidacyAnnouncementPayload;

impl CandidacyAnnouncementPayload {
    /// The [`Payload`](crate::types::block::payload::Payload) kind of a [`CandidacyAnnouncementPayload`].
    pub const KIND: u8 = 2;
}

// # TODO: check with TIP
impl WorkScore for CandidacyAnnouncementPayload {
    fn work_score(&self, params: crate::types::block::protocol::WorkScoreParameters) -> u32 {
        // 1 byte for the payload kind
        (1 + self.packed_len()) as u32 * params.data_byte()
    }

    fn mana_cost(&self, params: crate::types::block::protocol::WorkScoreParameters, reference_mana_cost: u64) -> u64 {
        reference_mana_cost * self.work_score(params) as u64
    }
}
