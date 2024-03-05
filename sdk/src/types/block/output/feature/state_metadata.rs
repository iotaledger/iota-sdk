// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use core::ops::RangeInclusive;

use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    prefix::BoxedSlicePrefix,
    unpacker::{CounterUnpacker, Unpacker},
    Packable, PackableExt,
};

use super::{
    metadata::{verify_keys, verify_packed_len, MetadataBTreeMap, MetadataBTreeMapPrefix},
    MetadataFeatureKeyLength, MetadataFeatureValueLength,
};
use crate::types::block::{
    output::{feature::FeatureError, StorageScore},
    protocol::WorkScore,
};

/// A Metadata Feature that can only be changed by the State Controller.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StateMetadataFeature(pub(crate) MetadataBTreeMapPrefix);

impl StateMetadataFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of [`StateMetadataFeature`].
    pub const KIND: u8 = 3;
    /// Valid byte lengths for a [`StateMetadataFeature`].
    pub const BYTE_LENGTH_RANGE: RangeInclusive<u16> = 1..=8192;

    /// Creates a new [`StateMetadataFeature`].
    #[inline(always)]
    pub fn new(data: impl IntoIterator<Item = (String, Vec<u8>)>) -> Result<Self, FeatureError> {
        let mut builder = Self::build();
        builder.extend(data);
        builder.finish()
    }

    /// Creates a new [`StateMetadataFeature`].
    pub fn build() -> StateMetadataFeatureMap {
        Default::default()
    }

    /// Creates a [`StateMetadataFeatureMap`] with the data so it can be mutated.
    pub fn to_map(&self) -> StateMetadataFeatureMap {
        StateMetadataFeatureMap(
            self.0
                .iter()
                .map(|(k, v)| {
                    (
                        String::from_utf8(k.as_ref().to_owned()).expect("key must be ASCII"),
                        v.as_ref().to_owned(),
                    )
                })
                .collect(),
        )
    }

    /// Returns the data for a given key.
    #[inline(always)]
    pub fn get(&self, key: &str) -> Option<&[u8]> {
        BoxedSlicePrefix::<u8, MetadataFeatureKeyLength>::try_from(key.as_bytes().to_vec().into_boxed_slice())
            .ok()
            .and_then(|key| self.0.get(&key))
            .map(|v| v.as_ref())
    }
}

impl core::ops::Deref for StateMetadataFeature {
    type Target = MetadataBTreeMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A map of state metadata feature keys to values. This type is not guaranteed to be valid.
#[derive(Clone, Debug, Default)]
pub struct StateMetadataFeatureMap(pub(crate) BTreeMap<String, Vec<u8>>);

impl StateMetadataFeatureMap {
    /// Creates a new [`StateMetadataFeatureMap`].
    #[inline(always)]
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn with_key_value(mut self, key: &str, value: impl Into<Vec<u8>>) -> Self {
        self.insert(key.to_owned(), value.into());
        self
    }

    pub fn finish(self) -> Result<StateMetadataFeature, FeatureError> {
        let res = StateMetadataFeature(
            MetadataBTreeMapPrefix::try_from(
                self.0
                    .iter()
                    .map(|(k, v)| {
                        Ok((
                            BoxedSlicePrefix::<u8, MetadataFeatureKeyLength>::try_from(
                                k.as_bytes().to_vec().into_boxed_slice(),
                            )
                            .map_err(|e| FeatureError::MetadataFeature(e.to_string()))?,
                            BoxedSlicePrefix::<u8, MetadataFeatureValueLength>::try_from(v.clone().into_boxed_slice())
                                .map_err(|e| FeatureError::MetadataFeature(e.to_string()))?,
                        ))
                    })
                    .collect::<Result<MetadataBTreeMap, FeatureError>>()?,
            )
            .map_err(FeatureError::MetadataFeatureEntryCount)?,
        );

        verify_keys(&res.0)?;
        verify_packed_len(res.packed_len(), StateMetadataFeature::BYTE_LENGTH_RANGE)?;

        Ok(res)
    }
}

impl core::ops::Deref for StateMetadataFeatureMap {
    type Target = BTreeMap<String, Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for StateMetadataFeatureMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl StorageScore for StateMetadataFeature {}

impl WorkScore for StateMetadataFeature {}

impl Packable for StateMetadataFeature {
    type UnpackError = FeatureError;
    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.0.pack(packer)
    }

    fn unpack<U: Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let mut unpacker = CounterUnpacker::new(unpacker);
        let res = Self(
            MetadataBTreeMapPrefix::unpack(&mut unpacker, visitor)
                .map_packable_err(|e| FeatureError::MetadataFeature(e.to_string()))?,
        );

        if visitor.is_some() {
            verify_keys(&res.0).map_err(UnpackError::Packable)?;
            verify_packed_len(unpacker.counter(), Self::BYTE_LENGTH_RANGE).map_err(UnpackError::Packable)?;
        }

        Ok(res)
    }
}

impl TryFrom<Vec<(String, Vec<u8>)>> for StateMetadataFeature {
    type Error = FeatureError;

    fn try_from(data: Vec<(String, Vec<u8>)>) -> Result<Self, Self::Error> {
        Self::new(data)
    }
}

impl TryFrom<BTreeMap<String, Vec<u8>>> for StateMetadataFeature {
    type Error = FeatureError;

    fn try_from(data: BTreeMap<String, Vec<u8>>) -> Result<Self, Self::Error> {
        Self::new(data)
    }
}

impl core::fmt::Display for StateMetadataFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:?}",
            self.0
                .keys()
                // Safe to unwrap, keys must be ascii
                .map(|k| alloc::str::from_utf8(k).unwrap())
                .collect::<Vec<&str>>()
        )
    }
}

impl core::fmt::Debug for StateMetadataFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "StateMetadataFeature({:?})",
            self.0
                .iter()
                .map(|(k, v)| (alloc::str::from_utf8(k).unwrap(), prefix_hex::encode(v.as_ref())))
                .collect::<BTreeMap<&str, String>>()
        )
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::{collections::BTreeMap, format};

    use serde::{de, Deserialize, Deserializer, Serialize};
    use serde_json::Value;

    use super::*;

    #[derive(Serialize)]
    struct StateMetadataFeatureDto {
        #[serde(rename = "type")]
        kind: u8,
        entries: BTreeMap<String, String>,
    }

    impl<'de> Deserialize<'de> for StateMetadataFeature {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match u8::try_from(
                    value
                        .get("type")
                        .and_then(Value::as_u64)
                        .ok_or_else(|| de::Error::custom("invalid state metadata type"))?,
                )
                .map_err(|_| de::Error::custom("invalid state metadata type: {e}"))?
                {
                    Self::KIND => {
                        let map: BTreeMap<String, String> = serde_json::from_value(
                            value
                                .get("entries")
                                .ok_or_else(|| de::Error::custom("missing state metadata entries"))?
                                .clone(),
                        )
                        .map_err(|e| de::Error::custom(format!("cannot deserialize state metadata feature: {e}")))?;

                        Self::try_from(
                            map.into_iter()
                                .map(|(key, value)| Ok((key, prefix_hex::decode::<Vec<u8>>(value)?)))
                                .collect::<Result<BTreeMap<String, Vec<u8>>, prefix_hex::Error>>()
                                .map_err(de::Error::custom)?,
                        )
                        .map_err(de::Error::custom)?
                    }
                    _ => return Err(de::Error::custom("invalid state metadata feature")),
                },
            )
        }
    }

    impl From<&StateMetadataFeature> for StateMetadataFeatureDto {
        fn from(value: &StateMetadataFeature) -> Self {
            let entries = value
                .0
                .iter()
                .map(|(k, v)| {
                    (
                        // Safe to unwrap, keys must be ascii
                        alloc::str::from_utf8(k.as_ref()).unwrap().to_string(),
                        prefix_hex::encode(v.as_ref()),
                    )
                })
                .collect::<BTreeMap<_, _>>();

            Self {
                kind: StateMetadataFeature::KIND,
                entries,
            }
        }
    }

    impl Serialize for StateMetadataFeature {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            StateMetadataFeatureDto::from(self).serialize(s)
        }
    }
}
