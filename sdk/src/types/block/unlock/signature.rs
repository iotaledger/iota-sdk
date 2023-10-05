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

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

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

    impl_serde_typed_dto!(SignatureUnlock, SignatureUnlockDto, "signature unlock");
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::{
        types::block::Error,
        utils::json::{FromJson, JsonExt, ToJson, Value},
    };

    impl ToJson for SignatureUnlock {
        fn to_json(&self) -> Value {
            crate::json! ({
                "type": Self::KIND,
                "signature": self.0
            })
        }
    }

    impl FromJson for SignatureUnlock {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if value["type"] != Self::KIND {
                return Err(Error::invalid_type::<Self>(Self::KIND, &value["type"]));
            }
            Ok(Self::new(value["signature"].take_value()?))
        }
    }
}
