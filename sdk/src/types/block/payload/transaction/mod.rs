// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Module describing the transaction payload.

mod essence;
mod transaction_id;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use packable::{error::UnpackError, packer::Packer, unpacker::Unpacker, Packable, PackableExt};

pub(crate) use self::essence::{ContextInputCount, InputCount, OutputCount};
pub use self::{
    essence::{RegularTransactionEssence, RegularTransactionEssenceBuilder, TransactionEssence},
    transaction_id::TransactionId,
};
use crate::types::block::{
    protocol::{ProtocolParameters, WorkScoreStructure},
    unlock::{Unlock, Unlocks},
    Error,
};

/// A transaction to move funds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionPayload {
    essence: RegularTransactionEssence,
    unlocks: Unlocks,
}

impl TransactionPayload {
    /// The payload kind of a [`TransactionPayload`].
    pub const KIND: u32 = 6;

    /// Creates a new [`TransactionPayload`].
    pub fn new(essence: RegularTransactionEssence, unlocks: Unlocks) -> Result<Self, Error> {
        verify_essence_unlocks(&essence, &unlocks)?;

        Ok(Self { essence, unlocks })
    }

    /// Return the essence of a [`TransactionPayload`].
    pub fn essence(&self) -> &RegularTransactionEssence {
        &self.essence
    }

    /// Return unlocks of a [`TransactionPayload`].
    pub fn unlocks(&self) -> &Unlocks {
        &self.unlocks
    }

    /// Computes the identifier of a [`TransactionPayload`].
    pub fn id(&self) -> TransactionId {
        let mut hasher = Blake2b256::new();

        hasher.update(Self::KIND.to_le_bytes());
        hasher.update(self.pack_to_vec());

        TransactionId::new(hasher.finalize().into())
    }

    /// Returns the work score of a `TransactionPayload`.
    pub fn workscore(&self, workscore_structure: WorkScoreStructure) -> u32 {
        let mut score = self.essence().workscore(workscore_structure);
        for unlock in self.unlocks.iter() {
            if matches!(unlock, Unlock::Signature(_)) {
                score += workscore_structure.signature_ed25519;
            }
        }
        score
    }
}

impl Packable for TransactionPayload {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        RegularTransactionEssence::KIND.pack(packer)?;
        self.essence.pack(packer)?;
        self.unlocks.pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let TransactionEssence::Regular(essence) = TransactionEssence::unpack::<_, VERIFY>(unpacker, visitor)?;
        let unlocks = Unlocks::unpack::<_, VERIFY>(unpacker, &())?;

        if VERIFY {
            verify_essence_unlocks(&essence, &unlocks).map_err(UnpackError::Packable)?;
        }

        Ok(Self { essence, unlocks })
    }
}

fn verify_essence_unlocks(essence: &RegularTransactionEssence, unlocks: &Unlocks) -> Result<(), Error> {
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

    pub use super::essence::dto::{RegularTransactionEssenceDto, TransactionEssenceDto};
    use super::*;
    use crate::types::{
        block::{unlock::Unlock, Error},
        TryFromDto, ValidationParams,
    };

    /// The payload type to define a value transaction.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct TransactionPayloadDto {
        #[serde(rename = "type")]
        pub kind: u32,
        pub essence: TransactionEssenceDto,
        pub unlocks: Vec<Unlock>,
    }

    impl From<&TransactionPayload> for TransactionPayloadDto {
        fn from(value: &TransactionPayload) -> Self {
            Self {
                kind: TransactionPayload::KIND,
                essence: TransactionEssenceDto::Regular(value.essence().into()),
                unlocks: value.unlocks().to_vec(),
            }
        }
    }

    impl TryFromDto for TransactionPayload {
        type Dto = TransactionPayloadDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            let TransactionEssence::Regular(essence) =
                TransactionEssence::try_from_dto_with_params_inner(dto.essence, params)?;
            Self::new(essence, Unlocks::new(dto.unlocks)?)
        }
    }
}
