// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::Packable;

/// A payload which is used to indicate candidacy for committee selection for the next epoch.
#[derive(Clone, Debug, Eq, PartialEq, Packable)]
pub struct CandidacyAnnouncementPayload;

impl CandidacyAnnouncementPayload {
    /// The [`Payload`](crate::types::block::payload::Payload) kind of a [`CandidacyAnnouncementPayload`].
    pub const KIND: u8 = 2;
}
