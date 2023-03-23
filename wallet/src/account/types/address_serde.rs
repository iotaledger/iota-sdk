// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{de::Visitor, Deserializer, Serializer};

use crate::account::types::address::AddressWrapper;

/// custom AddressWrapper serialization to use the bech32 representation
pub(crate) fn serialize<S: Serializer>(address: &AddressWrapper, s: S) -> std::result::Result<S::Ok, S::Error> {
    s.serialize_str(&address.to_bech32())
}

/// custom AddressWrapper deserialization to use the bech32 representation
pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<AddressWrapper, D::Error>
where
    D: Deserializer<'de>,
{
    struct AddressVisitor;
    impl<'de> Visitor<'de> for AddressVisitor {
        type Value = AddressWrapper;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a bech32 formatted string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            AddressWrapper::try_from_bech32(v).map_err(|e| serde::de::Error::custom(e.to_string()))
        }
    }

    deserializer.deserialize_str(AddressVisitor)
}
// }
