// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Module describing the signed transaction payload.

mod transaction;
mod transaction_id;

use packable::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable};

pub(crate) use self::transaction::{ContextInputCount, InputCount, OutputCount};
pub use self::{
    transaction::{Transaction, TransactionBuilder, TransactionCapabilities, TransactionCapabilityFlag},
    transaction_id::{TransactionHash, TransactionId},
};
use crate::types::block::{protocol::ProtocolParameters, unlock::Unlocks, Error};

/// A signed transaction to move funds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedTransactionPayload {
    transaction: Transaction,
    unlocks: Unlocks,
}

impl SignedTransactionPayload {
    /// The payload kind of a [`SignedTransactionPayload`].
    pub const KIND: u8 = 1;

    /// Creates a new [`SignedTransactionPayload`].
    pub fn new(transaction: Transaction, unlocks: Unlocks) -> Result<Self, Error> {
        verify_transaction_unlocks(&transaction, &unlocks)?;

        Ok(Self { transaction, unlocks })
    }

    /// Returns the transaction of a [`SignedTransactionPayload`].
    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    /// Returns unlocks of a [`SignedTransactionPayload`].
    pub fn unlocks(&self) -> &Unlocks {
        &self.unlocks
    }

    /// Computes the identifier of a [`SignedTransactionPayload`].
    pub fn id(&self) -> TransactionId {
        self.transaction()
            .hash()
            .into_transaction_id(self.transaction.creation_slot())
    }
}

impl Packable for SignedTransactionPayload {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.transaction.pack(packer)?;
        self.unlocks.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let transaction = Transaction::unpack::<_, VERIFY>(unpacker, visitor)?;
        let unlocks = Unlocks::unpack::<_, VERIFY>(unpacker, &())?;

        if VERIFY {
            verify_transaction_unlocks(&transaction, &unlocks).map_err(UnpackError::Packable)?;
        }

        Ok(Self { transaction, unlocks })
    }
}

fn verify_transaction_unlocks(transaction: &Transaction, unlocks: &Unlocks) -> Result<(), Error> {
    if transaction.inputs().len() != unlocks.len() {
        return Err(Error::InputUnlockCountMismatch {
            input_count: transaction.inputs().len(),
            unlock_count: unlocks.len(),
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
    use crate::types::{
        block::{unlock::Unlock, Error},
        TryFromDto, ValidationParams,
    };

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

    impl TryFromDto for SignedTransactionPayload {
        type Dto = SignedTransactionPayloadDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            let transaction = Transaction::try_from_dto_with_params_inner(dto.transaction, params)?;
            Self::new(transaction, Unlocks::new(dto.unlocks)?)
        }
    }
}
