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
