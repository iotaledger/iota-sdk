// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use crate::{
    client::api::{PreparedTransactionData, SignedTransactionData},
    types::block::{
        output::{Output, OutputId},
        protocol::ProtocolParameters,
        semantic::{SemanticValidationContext, TransactionFailureReason},
        signature::Ed25519Signature,
        Block, BlockId,
    },
};

// TODO this is wrong because of https://github.com/iotaledger/iota-sdk/issues/1208
pub(crate) const MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS: usize =
    Block::LENGTH_MAX - Block::LENGTH_MIN - (7 * BlockId::LENGTH);
pub(crate) const MAX_TX_LENGTH_FOR_BLOCK_WITH_SINGLE_PARENT: usize =
    Block::LENGTH_MAX - Block::LENGTH_MIN - BlockId::LENGTH;
// Length for unlocks with a single signature unlock (unlocks length + unlock type + signature type + public key +
// signature)
pub(crate) const SINGLE_UNLOCK_LENGTH: usize =
    1 + 1 + Ed25519Signature::PUBLIC_KEY_LENGTH + Ed25519Signature::SIGNATURE_LENGTH;
// Type + reference index
pub(crate) const REFERENCE_ACCOUNT_NFT_UNLOCK_LENGTH: usize = 1 + 2;

impl PreparedTransactionData {
    /// Verifies the semantic of a prepared transaction.
    pub fn verify_semantic(&self, protocol_parameters: &ProtocolParameters) -> Result<(), TransactionFailureReason> {
        let inputs = self
            .inputs_data
            .iter()
            .map(|input| (input.output_id(), &input.output))
            .collect::<Vec<(&OutputId, &Output)>>();

        let context = SemanticValidationContext::new(
            &self.transaction,
            &inputs,
            None,
            Some(&self.mana_rewards),
            protocol_parameters,
        );

        context.validate()
    }
}

impl SignedTransactionData {
    /// Verifies the semantic of a prepared transaction.
    pub fn verify_semantic(&self, protocol_parameters: &ProtocolParameters) -> Result<(), TransactionFailureReason> {
        let inputs = self
            .inputs_data
            .iter()
            .map(|input| (input.output_id(), &input.output))
            .collect::<Vec<(&OutputId, &Output)>>();

        let context = SemanticValidationContext::new(
            self.payload.transaction(),
            &inputs,
            Some(self.payload.unlocks()),
            Some(&self.mana_rewards),
            protocol_parameters,
        );

        context.validate()
    }
}
