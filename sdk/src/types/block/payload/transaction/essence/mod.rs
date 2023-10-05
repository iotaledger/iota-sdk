// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod regular;

use crypto::hashes::{blake2b::Blake2b256, Digest};
use derive_more::From;
use packable::PackableExt;

pub(crate) use self::regular::{ContextInputCount, InputCount, OutputCount};
pub use self::regular::{RegularTransactionEssence, RegularTransactionEssenceBuilder};
use crate::types::block::Error;

/// A generic essence that can represent different types defining transaction essences.
#[derive(Clone, Debug, Eq, PartialEq, From, packable::Packable)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidEssenceKind)]
pub enum TransactionEssence {
    /// A regular transaction essence.
    #[packable(tag = RegularTransactionEssence::KIND)]
    Regular(RegularTransactionEssence),
}

impl TransactionEssence {
    /// Returns the essence kind of an [`TransactionEssence`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Regular(_) => RegularTransactionEssence::KIND,
        }
    }

    /// Return the Blake2b hash of an [`TransactionEssence`].
    pub fn hash(&self) -> [u8; 32] {
        Blake2b256::digest(self.pack_to_vec()).into()
    }

    /// Checks whether the essence is a [`RegularTransactionEssence`].
    pub fn is_regular(&self) -> bool {
        matches!(self, Self::Regular(_))
    }

    /// Gets the essence as an actual [`RegularTransactionEssence`].
    /// PANIC: do not call on a non-regular essence.
    pub fn as_regular(&self) -> &RegularTransactionEssence {
        let Self::Regular(essence) = self;
        essence
    }
}

pub(crate) mod dto {
    pub use super::regular::dto::RegularTransactionEssenceDto;
    use super::*;
    use crate::types::{block::Error, TryFromDto, ValidationParams};

    /// Describes all the different essence types.
    #[derive(Clone, Debug, Eq, PartialEq, From)]
    #[cfg_attr(
        feature = "serde_types",
        derive(serde::Serialize, serde::Deserialize),
        serde(untagged)
    )]
    pub enum TransactionEssenceDto {
        Regular(RegularTransactionEssenceDto),
    }

    impl From<&TransactionEssence> for TransactionEssenceDto {
        fn from(value: &TransactionEssence) -> Self {
            match value {
                TransactionEssence::Regular(r) => Self::Regular(r.into()),
            }
        }
    }

    impl TryFromDto<TransactionEssenceDto> for TransactionEssence {
        type Error = Error;

        fn try_from_dto_with_params_inner(
            dto: TransactionEssenceDto,
            params: ValidationParams<'_>,
        ) -> Result<Self, Self::Error> {
            match dto {
                TransactionEssenceDto::Regular(r) => Ok(Self::Regular(
                    RegularTransactionEssence::try_from_dto_with_params_inner(r, params)?,
                )),
            }
        }
    }
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::utils::json::{FromJson, ToJson, Value};

    impl ToJson for TransactionEssence {
        fn to_json(&self) -> Value {
            match self {
                Self::Regular(e) => e.to_json(),
            }
        }
    }

    impl FromJson for dto::TransactionEssenceDto {
        type Error = Error;

        fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            Ok(match value["type"].as_u8() {
                Some(RegularTransactionEssence::KIND) => dto::RegularTransactionEssenceDto::from_json(value)?.into(),
                _ => {
                    return Err(Error::invalid_type::<Self>(
                        format!("one of {:?}", [RegularTransactionEssence::KIND]),
                        &value["type"],
                    ));
                }
            })
        }
    }
}
