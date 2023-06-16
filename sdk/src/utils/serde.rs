// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod string {
    use alloc::string::String;
    use core::{fmt::Display, str::FromStr};

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
    }
}

pub mod option_string {
    use alloc::string::String;
    use core::{fmt::Display, str::FromStr};

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        match value {
            Some(value) => serializer.collect_str(value),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        Option::<String>::deserialize(deserializer)?
            .map(|string| string.parse().map_err(de::Error::custom))
            .transpose()
    }
}

pub mod prefix_hex_box {
    use alloc::{string::String, vec::Vec};

    use packable::{bounded::Bounded, prefix::BoxedSlicePrefix};
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S, L: Bounded>(value: &BoxedSlicePrefix<u8, L>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&prefix_hex::encode(value.as_ref()))
    }

    pub fn deserialize<'de, D, L: Bounded>(deserializer: D) -> Result<BoxedSlicePrefix<u8, L>, D::Error>
    where
        D: Deserializer<'de>,
        <L as TryFrom<usize>>::Error: core::fmt::Display,
    {
        prefix_hex::decode::<Vec<_>>(String::deserialize(deserializer)?)
            .map_err(de::Error::custom)?
            .into_boxed_slice()
            .try_into()
            .map_err(de::Error::custom)
    }
}

pub mod boxed_slice_prefix {
    use packable::{bounded::Bounded, prefix::BoxedSlicePrefix};
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, V: Serialize, L: Bounded>(
        value: &BoxedSlicePrefix<V, L>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(value.as_ref())
    }

    pub fn deserialize<'de, D, V: Deserialize<'de>, L: Bounded>(
        deserializer: D,
    ) -> Result<BoxedSlicePrefix<V, L>, D::Error>
    where
        D: Deserializer<'de>,
        <L as TryFrom<usize>>::Error: core::fmt::Display,
    {
        Box::<[V]>::deserialize(deserializer)?
            .try_into()
            .map_err(de::Error::custom)
    }
}

pub mod option_prefix_hex_vec {
    use alloc::{string::String, vec::Vec};

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(bytes) => serializer.serialize_str(&prefix_hex::encode(bytes.as_slice())),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<String>::deserialize(deserializer)?
            .map(|string| prefix_hex::decode(string).map_err(de::Error::custom))
            .transpose()
    }
}
