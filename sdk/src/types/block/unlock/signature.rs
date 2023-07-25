// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Deref, From};

use crate::types::block::signature::Signature;

/// An [`Unlock`](crate::types::block::unlock::Unlock) which is used to unlock a signature locked
/// [`Input`](crate::types::block::input::Input).
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, Deref, packable::Packable)]
pub struct SignatureUnlock(pub(crate) Signature);

impl SignatureUnlock {
    /// The [`Unlock`](crate::types::block::unlock::Unlock) kind of a [`SignatureUnlock`].
    pub const KIND: u8 = 0;

    /// Creates a new [`SignatureUnlock`].
    #[inline(always)]
    pub fn new(signature: Signature) -> Self {
        Self(signature)
    }

    /// Returns the actual [`Signature`] of the [`SignatureUnlock`].
    #[inline(always)]
    pub fn signature(&self) -> &Signature {
        &self.0
    }
}

pub(crate) mod dto {
    use alloc::format;

    use serde::{Deserialize, Serialize};

    use super::*;

    /// Defines an unlock containing signature(s) unlocking input(s).
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct SignatureUnlockDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub signature: Signature,
    }

    impl From<&SignatureUnlock> for SignatureUnlockDto {
        fn from(value: &SignatureUnlock) -> Self {
            Self {
                kind: SignatureUnlock::KIND,
                signature: value.0,
            }
        }
    }

    impl From<SignatureUnlockDto> for SignatureUnlock {
        fn from(value: SignatureUnlockDto) -> Self {
            Self::new(value.signature)
        }
    }

    impl<'de> Deserialize<'de> for SignatureUnlock {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = SignatureUnlockDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid signature unlock type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            dto.try_into().map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for SignatureUnlock {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            SignatureUnlockDto::from(self).serialize(s)
        }
    }
}
