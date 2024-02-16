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

pub mod prefix_hex_bytes {
    use alloc::string::String;

    use prefix_hex::{FromHexPrefixed, ToHexPrefixed};
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        for<'a> &'a T: ToHexPrefixed,
    {
        serializer.serialize_str(&prefix_hex::encode(value))
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromHexPrefixed,
    {
        prefix_hex::decode(String::deserialize(deserializer)?)
            .map_err(crate::types::block::Error::Hex)
            .map_err(de::Error::custom)
    }
}

pub mod option_prefix_hex_bytes {
    use alloc::string::String;

    use prefix_hex::{FromHexPrefixed, ToHexPrefixed};
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        for<'a> &'a T: ToHexPrefixed,
    {
        match value {
            Some(bytes) => super::prefix_hex_bytes::serialize(bytes, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromHexPrefixed,
    {
        Option::<String>::deserialize(deserializer)?
            .map(|string| prefix_hex::decode(string).map_err(de::Error::custom))
            .transpose()
    }
}

pub mod string_prefix {
    use alloc::string::String;

    use packable::{bounded::Bounded, prefix::StringPrefix};
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T: Bounded, S>(value: &StringPrefix<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&**value)
    }

    pub fn deserialize<'de, T: Bounded, D>(deserializer: D) -> Result<StringPrefix<T>, D::Error>
    where
        D: Deserializer<'de>,
        <T as TryFrom<usize>>::Error: core::fmt::Display,
    {
        String::deserialize(deserializer)
            .map_err(de::Error::custom)
            .and_then(|s| s.try_into().map_err(de::Error::custom))
    }
}

pub mod boxed_slice_prefix_hex_bytes {
    use alloc::boxed::Box;

    use packable::{bounded::Bounded, prefix::BoxedSlicePrefix};
    use prefix_hex::{FromHexPrefixed, ToHexPrefixed};
    use serde::{de, Deserializer, Serializer};

    pub fn serialize<S, T, B: Bounded>(value: &BoxedSlicePrefix<T, B>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        for<'a> &'a Box<[T]>: ToHexPrefixed,
    {
        super::prefix_hex_bytes::serialize(&**value, serializer)
    }

    pub fn deserialize<'de, D, T, B: Bounded>(deserializer: D) -> Result<BoxedSlicePrefix<T, B>, D::Error>
    where
        D: Deserializer<'de>,
        Box<[T]>: FromHexPrefixed,
        <B as TryFrom<usize>>::Error: core::fmt::Display,
    {
        super::prefix_hex_bytes::deserialize::<_, Box<[T]>>(deserializer)?
            .try_into()
            .map_err(de::Error::custom)
    }
}

pub mod cow_boxed_slice_prefix_hex_bytes {
    use alloc::{borrow::Cow, boxed::Box};

    use packable::{bounded::Bounded, prefix::BoxedSlicePrefix};
    use prefix_hex::FromHexPrefixed;
    use serde::Deserializer;

    pub use super::boxed_slice_prefix_hex_bytes::serialize;

    pub fn deserialize<'de, 'a, D, B>(deserializer: D) -> Result<Cow<'a, BoxedSlicePrefix<u8, B>>, D::Error>
    where
        D: Deserializer<'de>,
        B: Bounded + Clone,
        Box<[u8]>: FromHexPrefixed,
        <B as TryFrom<usize>>::Error: core::fmt::Display,
    {
        Ok(Cow::Owned(super::boxed_slice_prefix_hex_bytes::deserialize(
            deserializer,
        )?))
    }
}

pub mod boxed_slice_prefix {
    use alloc::vec::Vec;

    use packable::{bounded::Bounded, prefix::BoxedSlicePrefix};
    use serde::{de, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T, B: Bounded>(value: &BoxedSlicePrefix<T, B>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for e in value.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D, T, B: Bounded>(deserializer: D) -> Result<BoxedSlicePrefix<T, B>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
        <B as TryFrom<usize>>::Error: core::fmt::Display,
    {
        BoxedSlicePrefix::try_from(Vec::<T>::deserialize(deserializer)?.into_boxed_slice()).map_err(de::Error::custom)
    }
}

#[cfg(feature = "client")]
pub mod bip44 {
    use crypto::keys::bip44::Bip44;
    use serde::{Deserialize, Serialize};

    #[derive(Default, Serialize, Deserialize)]
    #[serde(default = "default_bip44", rename_all = "camelCase", remote = "Bip44")]
    pub struct Bip44Def {
        coin_type: u32,
        account: u32,
        change: u32,
        address_index: u32,
    }

    fn default_bip44() -> Bip44 {
        Bip44::new(crate::client::constants::IOTA_COIN_TYPE)
    }

    pub mod option_bip44 {
        use serde::{Deserializer, Serializer};

        use super::{Bip44Def, *};

        pub fn serialize<S>(value: &Option<Bip44>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            #[derive(Serialize)]
            struct Helper<'a>(#[serde(with = "Bip44Def")] &'a Bip44);

            value.as_ref().map(Helper).serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Bip44>, D::Error>
        where
            D: Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct Helper(#[serde(with = "Bip44Def")] Bip44);

            let helper = Option::deserialize(deserializer)?;
            Ok(helper.map(|Helper(external)| external))
        }
    }
}

#[cfg(feature = "client")]
pub mod mana_rewards {
    use alloc::collections::BTreeMap;

    use serde::{Deserialize, Deserializer, Serialize};

    use crate::types::block::output::OutputId;

    pub fn serialize<S: serde::Serializer>(mana_rewards: &BTreeMap<OutputId, u64>, s: S) -> Result<S::Ok, S::Error> {
        let map = mana_rewards
            .iter()
            .map(|(k, v)| (k, v.to_string()))
            .collect::<BTreeMap<_, _>>();
        map.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<BTreeMap<OutputId, u64>, D::Error> {
        BTreeMap::<OutputId, String>::deserialize(d)?
            .into_iter()
            .map(|(k, v)| Ok((k, v.parse().map_err(serde::de::Error::custom)?)))
            .collect::<Result<BTreeMap<_, u64>, _>>()
    }
}
