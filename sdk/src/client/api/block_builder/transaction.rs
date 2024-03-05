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
        payload::signed_transaction::{SignedTransactionPayload, Transaction},
        protocol::ProtocolParameters,
        semantic::{SemanticValidationContext, TransactionFailureReason},
        signature::Ed25519Signature,
        Block, BlockId,
    },
};

// TODO this is wrong because of https://github.com/iotaledger/iota-sdk/issues/1208
const MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS: usize = Block::LENGTH_MAX - Block::LENGTH_MIN - (7 * BlockId::LENGTH);
// Length for unlocks with a single signature unlock (unlocks length + unlock type + signature type + public key +
// signature)
const SINGLE_UNLOCK_LENGTH: usize = 1 + 1 + Ed25519Signature::PUBLIC_KEY_LENGTH + Ed25519Signature::SIGNATURE_LENGTH;
// Type + reference index
const REFERENCE_ACCOUNT_NFT_UNLOCK_LENGTH: usize = 1 + 2;

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
            &self.payload.transaction(),
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

impl Transaction {
    /// Verifies that the transaction doesn't exceed the block size limit with 8 parents.
    /// Assuming one signature unlock and otherwise reference/account/nft unlocks.
    /// `validate_transaction_payload_length()` should later be used to check the length again with the correct
    /// unlocks.
    pub fn validate_length(&self) -> Result<(), ClientError> {
        let transaction_bytes = self.pack_to_vec();

        // Assuming there is only 1 signature unlock and the rest is reference/account/nft unlocks
        let reference_account_nft_unlocks_amount = self.inputs().len() - 1;

        // Max tx payload length - length for one signature unlock (there might be more unlocks, we check with them
        // later again, when we built the transaction payload)
        let max_length = MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS
            - SINGLE_UNLOCK_LENGTH
            - (reference_account_nft_unlocks_amount * REFERENCE_ACCOUNT_NFT_UNLOCK_LENGTH);

        if transaction_bytes.len() > max_length {
            return Err(ClientError::InvalidTransactionLength {
                length: transaction_bytes.len(),
                max_length,
            });
        }
        Ok(())
    }
}
