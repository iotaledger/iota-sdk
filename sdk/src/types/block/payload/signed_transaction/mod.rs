// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Module describing the signed transaction payload.

mod transaction;
mod transaction_id;

use packable::{Packable, PackableExt};

use self::transaction::MAX_TX_LENGTH_FOR_BLOCK_WITH_SINGLE_PARENT;
pub(crate) use self::transaction::{InputCount, OutputCount};
pub use self::{
    transaction::{Transaction, TransactionBuilder, TransactionCapabilities, TransactionCapabilityFlag},
    transaction_id::{TransactionHash, TransactionId, TransactionSigningHash},
};
use crate::types::block::{
    payload::PayloadError,
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
    unlock::Unlocks,
};

/// A signed transaction to move funds.
#[derive(Clone, Debug, Eq, PartialEq, Packable)]
#[packable(unpack_error = PayloadError)]
#[packable(verify_with = verify_signed_transaction_payload)]
pub struct SignedTransactionPayload {
    transaction: Transaction,
    unlocks: Unlocks,
}

impl SignedTransactionPayload {
    /// The [`Payload`](crate::types::block::payload::Payload) kind of a [`SignedTransactionPayload`].
    pub const KIND: u8 = 1;

    /// Creates a new [`SignedTransactionPayload`].
    pub fn new(transaction: Transaction, unlocks: Unlocks) -> Result<Self, PayloadError> {
        let payload = Self { transaction, unlocks };

        verify_signed_transaction_payload(&payload)?;

        Ok(payload)
    }

    /// Returns the transaction of a [`SignedTransactionPayload`].
    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    /// Returns unlocks of a [`SignedTransactionPayload`].
    pub fn unlocks(&self) -> &Unlocks {
        &self.unlocks
    }
}

impl WorkScore for SignedTransactionPayload {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        // 1 byte for the payload kind
        (1 + self.packed_len() as u32) * params.data_byte()
            + self.transaction().work_score(params)
            + self.unlocks().work_score(params)
    }
}

fn verify_signed_transaction_payload(payload: &SignedTransactionPayload) -> Result<(), PayloadError> {
    if payload.transaction.inputs().len() != payload.unlocks.len() {
        return Err(PayloadError::InputUnlockCountMismatch {
            input_count: payload.transaction.inputs().len(),
            unlock_count: payload.unlocks.len(),
        });
    }

    verify_signed_transaction_payload_length(payload)?;

    Ok(())
}

fn verify_signed_transaction_payload_length(payload: &SignedTransactionPayload) -> Result<(), PayloadError> {
    let signed_transaction_payload_bytes = payload.pack_to_vec();
    if signed_transaction_payload_bytes.len() > MAX_TX_LENGTH_FOR_BLOCK_WITH_SINGLE_PARENT {
        return Err(PayloadError::SignedTransactionPayloadLength {
            length: signed_transaction_payload_bytes.len(),
            max_length: MAX_TX_LENGTH_FOR_BLOCK_WITH_SINGLE_PARENT,
        });
    }

    Ok(())
}

#[cfg(feature = "serde")]
pub mod dto {
    use alloc::vec::Vec;

    use serde::{Deserialize, Serialize};

    pub use super::transaction::dto::TransactionDto;
    use super::*;
    use crate::types::{block::unlock::Unlock, TryFromDto};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct SignedTransactionPayloadDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub transaction: TransactionDto,
        pub unlocks: Vec<Unlock>,
    }

    impl From<&SignedTransactionPayload> for SignedTransactionPayloadDto {
        fn from(value: &SignedTransactionPayload) -> Self {
            Self {
                kind: SignedTransactionPayload::KIND,
                transaction: value.transaction().into(),
                unlocks: value.unlocks().to_vec(),
            }
        }
    }

    impl TryFromDto<SignedTransactionPayloadDto> for SignedTransactionPayload {
        type Error = PayloadError;

        fn try_from_dto_with_params_inner(
            dto: SignedTransactionPayloadDto,
            params: Option<&ProtocolParameters>,
        ) -> Result<Self, Self::Error> {
            let transaction = Transaction::try_from_dto_with_params_inner(dto.transaction, params)?;
            Self::new(transaction, Unlocks::new(dto.unlocks)?)
        }
    }
}
