// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use packable::PackableExt;

use crate::{
    client::{
        api::{PreparedTransactionData, SignedTransactionData},
        ClientError,
    },
    types::block::{
        output::{Output, OutputId},
        payload::signed_transaction::SignedTransactionPayload,
        protocol::ProtocolParameters,
        semantic::{SemanticValidationContext, TransactionFailureReason},
        signature::Ed25519Signature,
        Block, BlockId,
    },
};

// TODO this is wrong because of https://github.com/iotaledger/iota-sdk/issues/1208
pub(crate) const MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS: usize =
    Block::LENGTH_MAX - Block::LENGTH_MIN - (7 * BlockId::LENGTH);
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

impl SignedTransactionPayload {
    /// Verifies that the signed transaction payload doesn't exceed the block size limit with 8 parents.
    pub fn validate_length(&self) -> Result<(), ClientError> {
        let signed_transaction_payload_bytes = self.pack_to_vec();
        if signed_transaction_payload_bytes.len() > MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS {
            return Err(ClientError::InvalidSignedTransactionPayloadLength {
                length: signed_transaction_payload_bytes.len(),
                max_length: MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS,
            });
        }
        Ok(())
    }
}
