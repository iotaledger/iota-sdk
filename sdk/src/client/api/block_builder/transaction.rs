// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Transaction preparation and signing

use packable::PackableExt;

use crate::{
    client::{secret::types::InputSigningData, Error, Result},
    types::block::{
        output::{Output, OutputId},
        payload::signed_transaction::{SignedTransactionPayload, Transaction},
        semantic::{semantic_validation, TransactionFailureReason, ValidationContext},
        signature::Ed25519Signature,
        BlockId, BlockWrapper,
    },
};

// TODO this is wrong because of https://github.com/iotaledger/iota-sdk/issues/1208
const MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS: usize =
    BlockWrapper::LENGTH_MAX - BlockWrapper::LENGTH_MIN - (7 * BlockId::LENGTH);
// Length for unlocks with a single signature unlock (unlocks length + unlock type + signature type + public key +
// signature)
const SINGLE_UNLOCK_LENGTH: usize = 1 + 1 + Ed25519Signature::PUBLIC_KEY_LENGTH + Ed25519Signature::SIGNATURE_LENGTH;
// Type + reference index
const REFERENCE_ACCOUNT_NFT_UNLOCK_LENGTH: usize = 1 + 2;

/// Verifies the semantic of a prepared transaction.
pub fn verify_semantic(
    input_signing_data: &[InputSigningData],
    transaction: &SignedTransactionPayload,
) -> crate::client::Result<Option<TransactionFailureReason>> {
    let transaction_id = transaction.id();
    let inputs = input_signing_data
        .iter()
        .map(|input| (input.output_id(), &input.output))
        .collect::<Vec<(&OutputId, &Output)>>();

    let context = ValidationContext::new(
        &transaction_id,
        transaction.transaction(),
        inputs.iter().map(|(id, input)| (*id, *input)),
        transaction.unlocks(),
    );

    Ok(semantic_validation(context, inputs.as_slice(), transaction.unlocks())?)
}

/// Verifies that the signed transaction payload doesn't exceed the block size limit with 8 parents.
pub fn validate_signed_transaction_payload_length(signed_transaction_payload: &SignedTransactionPayload) -> Result<()> {
    let signed_transaction_payload_bytes = signed_transaction_payload.pack_to_vec();
    if signed_transaction_payload_bytes.len() > MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS {
        return Err(Error::InvalidSignedTransactionPayloadLength {
            length: signed_transaction_payload_bytes.len(),
            max_length: MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS,
        });
    }
    Ok(())
}

/// Verifies that the transaction essence doesn't exceed the block size limit with 8 parents.
/// Assuming one signature unlock and otherwise reference/account/nft unlocks. `validate_transaction_payload_length()`
/// should later be used to check the length again with the correct unlocks.
pub fn validate_regular_transaction_essence_length(regular_transaction_essence: &Transaction) -> Result<()> {
    let regular_transaction_essence_bytes = regular_transaction_essence.pack_to_vec();

    // Assuming there is only 1 signature unlock and the rest is reference/account/nft unlocks
    let reference_account_nft_unlocks_amount = regular_transaction_essence.inputs().len() - 1;

    // Max tx payload length - length for one signature unlock (there might be more unlocks, we check with them
    // later again, when we built the transaction payload)
    let max_length = MAX_TX_LENGTH_FOR_BLOCK_WITH_8_PARENTS
        - SINGLE_UNLOCK_LENGTH
        - (reference_account_nft_unlocks_amount * REFERENCE_ACCOUNT_NFT_UNLOCK_LENGTH);

    if regular_transaction_essence_bytes.len() > max_length {
        return Err(Error::InvalidTransactionLength {
            length: regular_transaction_essence_bytes.len(),
            max_length,
        });
    }
    Ok(())
}
