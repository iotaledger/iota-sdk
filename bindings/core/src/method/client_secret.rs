// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::bip44::Bip44;
use derivative::Derivative;
use iota_sdk::types::block::{core::basic, payload::dto::PayloadDto, IssuerId};
use serde::{Deserialize, Serialize};

/// Each public client + secret manager method.
#[derive(Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
#[serde(tag = "name", content = "data", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ClientSecretMethod {
    /// Build a basic block containing the specified payload and post it to the network.
    PostBasicBlockPayload {
        /// The issuer's ID.
        issuer_id: IssuerId,
        strong_parents: Option<basic::StrongParents>,
        /// The payload to send.
        payload: PayloadDto,
        /// The Bip44 chain to use when signing the block.
        chain: Bip44,
    },
}
