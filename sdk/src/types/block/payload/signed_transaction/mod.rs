// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Module describing the transaction payload.

mod transaction;
mod transaction_id;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable, PackableExt};

pub(crate) use self::transaction::{ContextInputCount, InputCount, OutputCount};
pub use self::{
    transaction::{Transaction, TransactionBuilder, TransactionCapabilities, TransactionCapabilityFlag},
    transaction_id::TransactionId,
};
use crate::types::block::{protocol::ProtocolParameters, unlock::Unlocks, Error};

/// A signed transaction to move funds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedTransactionPayload {
    essence: Transaction,
    unlocks: Unlocks,
}

impl SignedTransactionPayload {
    /// The payload kind of a [`SignedTransactionPayload`].
    pub const KIND: u8 = 1;

    /// Creates a new [`SignedTransactionPayload`].
    pub fn new(essence: Transaction, unlocks: Unlocks) -> Result<Self, Error> {
        verify_essence_unlocks(&essence, &unlocks)?;

        Ok(Self { essence, unlocks })
    }

    /// Return the essence of a [`SignedTransactionPayload`].
    pub fn essence(&self) -> &Transaction {
        &self.essence
    }

    /// Return unlocks of a [`SignedTransactionPayload`].
    pub fn unlocks(&self) -> &Unlocks {
        &self.unlocks
    }

    /// Computes the identifier of a [`SignedTransactionPayload`].
    pub fn id(&self) -> TransactionId {
        let mut hasher = Blake2b256::new();

        hasher.update(Self::KIND.to_le_bytes());
        hasher.update(self.pack_to_vec());

        TransactionId::new(hasher.finalize().into())
    }
}

impl Packable for SignedTransactionPayload {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.essence.pack(packer)?;
        self.unlocks.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let essence = Transaction::unpack::<_, VERIFY>(unpacker, visitor)?;
        let unlocks = Unlocks::unpack::<_, VERIFY>(unpacker, &())?;

        if VERIFY {
            verify_essence_unlocks(&essence, &unlocks).map_err(UnpackError::Packable)?;
        }

        Ok(Self { essence, unlocks })
    }
}

fn verify_essence_unlocks(essence: &Transaction, unlocks: &Unlocks) -> Result<(), Error> {
    if essence.inputs().len() != unlocks.len() {
        return Err(Error::InputUnlockCountMismatch {
            input_count: essence.inputs().len(),
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
        pub essence: TransactionDto,
        pub unlocks: Vec<Unlock>,
    }

    impl From<&SignedTransactionPayload> for SignedTransactionPayloadDto {
        fn from(value: &SignedTransactionPayload) -> Self {
            Self {
                kind: SignedTransactionPayload::KIND,
                essence: value.essence().into(),
                unlocks: value.unlocks().to_vec(),
            }
        }
    }

    impl TryFromDto for SignedTransactionPayload {
        type Dto = SignedTransactionPayloadDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            let essence = Transaction::try_from_dto_with_params_inner(dto.essence, params)?;
            Self::new(essence, Unlocks::new(dto.unlocks)?)
        }
    }
}
