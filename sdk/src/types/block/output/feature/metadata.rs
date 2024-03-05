// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::ops::RangeInclusive;

use packable::{
    bounded::{BoundedU16, BoundedU8},
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    prefix::{BTreeMapPrefix, BoxedSlicePrefix},
    unpacker::{CounterUnpacker, Unpacker},
    Packable, PackableExt,
};

use crate::types::block::{
    output::{feature::FeatureError, StorageScore},
    protocol::WorkScore,
};

pub(crate) type MetadataFeatureEntryCount = BoundedU8<1, { u8::MAX }>;
pub(crate) type MetadataFeatureKeyLength = BoundedU8<1, { u8::MAX }>;
pub(crate) type MetadataFeatureValueLength = BoundedU16<0, { u16::MAX }>;

pub(crate) type MetadataBTreeMapPrefix = BTreeMapPrefix<
    BoxedSlicePrefix<u8, MetadataFeatureKeyLength>,
    BoxedSlicePrefix<u8, MetadataFeatureValueLength>,
    MetadataFeatureEntryCount,
>;
pub(crate) type MetadataBTreeMap =
    BTreeMap<BoxedSlicePrefix<u8, MetadataFeatureKeyLength>, BoxedSlicePrefix<u8, MetadataFeatureValueLength>>;

/// Defines metadata, arbitrary binary data, that will be stored in the output.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MetadataFeature(MetadataBTreeMapPrefix);

pub(crate) fn verify_keys(map: &MetadataBTreeMapPrefix) -> Result<(), FeatureError> {
    for key in map.keys() {
        if !key.iter().all(|c| c.is_ascii_graphic()) {
            return Err(FeatureError::NonGraphicAsciiMetadataKey(key.to_vec()));
        }
    }
    Ok(())
}

pub(crate) fn verify_packed_len(len: usize, bytes_length_range: RangeInclusive<u16>) -> Result<(), FeatureError> {
    if !bytes_length_range.contains(&u16::try_from(len).map_err(|e| FeatureError::MetadataFeature(e.to_string()))?) {
        return Err(FeatureError::MetadataFeature(format!(
            "Out of bounds byte length: {len}"
        )));
    }
    Ok(())
}

impl MetadataFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of [`MetadataFeature`].
    pub const KIND: u8 = 2;
    /// Valid byte lengths for a [`MetadataFeature`].
    pub const BYTE_LENGTH_RANGE: RangeInclusive<u16> = 1..=8192;

    /// Creates a new [`MetadataFeature`].
    #[inline(always)]
    pub fn new(data: impl IntoIterator<Item = (String, Vec<u8>)>) -> Result<Self, FeatureError> {
        let mut builder = Self::build();
        builder.extend(data);
        builder.finish()
    }

    /// Creates a new [`MetadataFeatureMap`].
    pub fn build() -> MetadataFeatureMap {
        Default::default()
    }

    /// Creates a [`MetadataFeatureMap`] with the data so it can be mutated.
    pub fn to_map(&self) -> MetadataFeatureMap {
        MetadataFeatureMap(
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

impl core::ops::Deref for MetadataFeature {
    type Target = MetadataBTreeMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A map of metadata feature keys to values. This type is not guaranteed to be valid.
#[derive(Clone, Debug, Default)]
pub struct MetadataFeatureMap(BTreeMap<String, Vec<u8>>);

impl MetadataFeatureMap {
    /// Creates a new [`MetadataFeatureMap`].
    #[inline(always)]
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn with_key_value(mut self, key: &str, value: impl Into<Vec<u8>>) -> Self {
        self.insert(key.to_owned(), value.into());
        self
    }

    pub fn finish(self) -> Result<MetadataFeature, FeatureError> {
        let res = MetadataFeature(
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
        verify_packed_len(res.packed_len(), MetadataFeature::BYTE_LENGTH_RANGE)?;

        Ok(res)
    }
}

impl core::ops::Deref for MetadataFeatureMap {
    type Target = BTreeMap<String, Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for MetadataFeatureMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl StorageScore for MetadataFeature {}

impl WorkScore for MetadataFeature {}

impl Packable for MetadataFeature {
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

impl TryFrom<Vec<(String, Vec<u8>)>> for MetadataFeature {
    type Error = FeatureError;

    fn try_from(data: Vec<(String, Vec<u8>)>) -> Result<Self, FeatureError> {
        Self::new(data)
    }
}

impl TryFrom<BTreeMap<String, Vec<u8>>> for MetadataFeature {
    type Error = FeatureError;

    fn try_from(data: BTreeMap<String, Vec<u8>>) -> Result<Self, FeatureError> {
        Self::new(data)
    }
}

impl core::fmt::Display for MetadataFeature {
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

impl core::fmt::Debug for MetadataFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "MetadataFeature({:?})",
            self.0
                .iter()
                .map(|(k, v)| (alloc::str::from_utf8(k).unwrap(), prefix_hex::encode(v.as_ref())))
                .collect::<BTreeMap<&str, String>>()
        )
    }
}

#[cfg(feature = "irc_27")]
pub(crate) mod irc_27 {
    use alloc::{
        borrow::ToOwned,
        collections::{BTreeMap, BTreeSet},
        string::String,
    };

    use getset::Getters;
    use serde::{Deserialize, Serialize};
    use url::Url;

    use super::*;
    use crate::types::block::address::Bech32Address;

    /// The IRC27 NFT standard schema.
    #[derive(Clone, Debug, Serialize, Deserialize, Getters, PartialEq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "standard", rename = "IRC27")]
    #[getset(get = "pub")]
    pub struct Irc27Metadata {
        version: String,
        /// The media type (MIME) of the asset.
        ///
        /// ## Examples
        /// - Image files: `image/jpeg`, `image/png`, `image/gif`, etc.
        /// - Video files: `video/x-msvideo` (avi), `video/mp4`, `video/mpeg`, etc.
        /// - Audio files: `audio/mpeg`, `audio/wav`, etc.
        /// - 3D Assets: `model/obj`, `model/u3d`, etc.
        /// - Documents: `application/pdf`, `text/plain`, etc.
        #[serde(rename = "type")]
        media_type: String,
        /// URL pointing to the NFT file location.
        uri: Url,
        /// The human-readable name of the native token.
        name: String,
        /// The human-readable collection name of the native token.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        collection_name: Option<String>,
        /// Royalty payment addresses mapped to the payout percentage.
        #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
        royalties: BTreeMap<Bech32Address, f64>,
        /// The human-readable name of the native token creator.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        issuer_name: Option<String>,
        /// The human-readable description of the token.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Additional attributes which follow [OpenSea Metadata standards](https://docs.opensea.io/docs/metadata-standards).
        #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
        attributes: BTreeSet<Attribute>,
    }

    impl Irc27Metadata {
        pub fn new(media_type: impl Into<String>, uri: Url, name: impl Into<String>) -> Self {
            Self {
                version: "v1.0".to_owned(),
                media_type: media_type.into(),
                uri,
                name: name.into(),
                collection_name: Default::default(),
                royalties: Default::default(),
                issuer_name: Default::default(),
                description: Default::default(),
                attributes: Default::default(),
            }
        }

        pub fn with_collection_name(mut self, collection_name: impl Into<String>) -> Self {
            self.collection_name.replace(collection_name.into());
            self
        }

        pub fn add_royalty(mut self, address: Bech32Address, percentage: f64) -> Self {
            self.royalties.insert(address, percentage);
            self
        }

        pub fn with_royalties(mut self, royalties: BTreeMap<Bech32Address, f64>) -> Self {
            self.royalties = royalties;
            self
        }

        pub fn with_issuer_name(mut self, issuer_name: impl Into<String>) -> Self {
            self.issuer_name.replace(issuer_name.into());
            self
        }

        pub fn with_description(mut self, description: impl Into<String>) -> Self {
            self.description.replace(description.into());
            self
        }

        pub fn add_attribute(mut self, attribute: Attribute) -> Self {
            self.attributes.insert(attribute);
            self
        }

        pub fn with_attributes(mut self, attributes: BTreeSet<Attribute>) -> Self {
            self.attributes = attributes;
            self
        }

        pub fn to_bytes(&self) -> Vec<u8> {
            // Unwrap: safe because this struct is known to be valid.
            serde_json::to_string(self).unwrap().into_bytes()
        }
    }

    impl TryFrom<Irc27Metadata> for MetadataFeature {
        type Error = FeatureError;

        fn try_from(value: Irc27Metadata) -> Result<Self, Self::Error> {
            // TODO: is this hardcoded key correct or should users provide it?
            Self::build().with_key_value("irc-27", value).finish()
        }
    }

    impl From<Irc27Metadata> for Vec<u8> {
        fn from(value: Irc27Metadata) -> Self {
            value.to_bytes()
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Getters, PartialEq, Eq)]
    #[getset(get = "pub")]
    pub struct Attribute {
        trait_type: String,
        value: serde_json::Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        display_type: Option<String>,
    }

    impl Attribute {
        pub fn new(trait_type: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
            Self {
                trait_type: trait_type.into(),
                display_type: None,
                value: value.into(),
            }
        }

        pub fn with_display_type(mut self, display_type: impl Into<String>) -> Self {
            self.display_type.replace(display_type.into());
            self
        }
    }

    impl Ord for Attribute {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            self.trait_type.cmp(&other.trait_type)
        }
    }
    impl PartialOrd for Attribute {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl core::hash::Hash for Attribute {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.trait_type.hash(state);
        }
    }

    #[cfg(test)]
    mod test {
        use pretty_assertions::assert_eq;

        use super::*;
        use crate::types::block::{address::ToBech32Ext, rand::address::rand_base_address};

        #[test]
        fn serialization() {
            let metadata = Irc27Metadata::new(
                "image/jpeg",
                "https://mywebsite.com/my-nft-files-1.jpeg".parse().unwrap(),
                "My NFT #0001",
            )
            .with_collection_name("My Collection of Art")
            .add_royalty(rand_base_address().to_bech32_unchecked("iota1"), 0.025)
            .add_royalty(rand_base_address().to_bech32_unchecked("iota1"), 0.025)
            .with_issuer_name("My Artist Name")
            .with_description("A little information about my NFT collection")
            .add_attribute(Attribute::new("Background", "Purple"))
            .add_attribute(Attribute::new("Element", "Water"))
            .add_attribute(Attribute::new("Attack", 150))
            .add_attribute(Attribute::new("Health", 500));
            let json = serde_json::json!(
                {
                    "standard": "IRC27",
                    "version": metadata.version(),
                    "type": metadata.media_type(),
                    "uri": metadata.uri(),
                    "name": metadata.name(),
                    "collectionName": metadata.collection_name(),
                    "royalties": metadata.royalties(),
                    "issuerName": metadata.issuer_name(),
                    "description": metadata.description(),
                    "attributes": metadata.attributes()
                  }
            );
            let metadata_deser = serde_json::from_value::<Irc27Metadata>(json.clone()).unwrap();

            assert_eq!(metadata, metadata_deser);
            assert_eq!(json, serde_json::to_value(metadata).unwrap())
        }
    }
}

#[cfg(feature = "irc_30")]
pub(crate) mod irc_30 {
    use alloc::string::String;

    use getset::Getters;
    use serde::{Deserialize, Serialize};
    use url::Url;

    use super::*;

    /// The IRC30 native token metadata standard schema.
    #[derive(Clone, Debug, Serialize, Deserialize, Getters, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "standard", rename = "IRC30")]
    #[getset(get = "pub")]
    pub struct Irc30Metadata {
        /// The human-readable name of the native token.
        name: String,
        /// The symbol/ticker of the token.
        symbol: String,
        /// Number of decimals the token uses (divide the token amount by `10^decimals` to get its user
        /// representation).
        decimals: u32,
        /// The human-readable description of the token.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// URL pointing to more resources about the token.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        url: Option<Url>,
        /// URL pointing to an image resource of the token logo.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logo_url: Option<Url>,
        /// The svg logo of the token encoded as a byte string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logo: Option<String>,
    }

    impl Irc30Metadata {
        pub fn new(name: impl Into<String>, symbol: impl Into<String>, decimals: u32) -> Self {
            Self {
                name: name.into(),
                symbol: symbol.into(),
                decimals,
                description: Default::default(),
                url: Default::default(),
                logo_url: Default::default(),
                logo: Default::default(),
            }
        }

        pub fn with_description(mut self, description: impl Into<String>) -> Self {
            self.description.replace(description.into());
            self
        }

        pub fn with_url(mut self, url: Url) -> Self {
            self.url.replace(url);
            self
        }

        pub fn with_logo_url(mut self, logo_url: Url) -> Self {
            self.logo_url.replace(logo_url);
            self
        }

        pub fn with_logo(mut self, logo: impl Into<String>) -> Self {
            self.logo.replace(logo.into());
            self
        }

        pub fn to_bytes(&self) -> Vec<u8> {
            // Unwrap: safe because this struct is known to be valid.
            serde_json::to_string(self).unwrap().into_bytes()
        }
    }

    impl TryFrom<Irc30Metadata> for MetadataFeature {
        type Error = FeatureError;

        fn try_from(value: Irc30Metadata) -> Result<Self, Self::Error> {
            // TODO: is this hardcoded key correct or should users provide it?
            Self::build().with_key_value("irc-30", value).finish()
        }
    }

    impl From<Irc30Metadata> for Vec<u8> {
        fn from(value: Irc30Metadata) -> Self {
            value.to_bytes()
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn serialization() {
            let description = "FooCoin is the utility and governance token of FooLand, \
                a revolutionary protocol in the play-to-earn crypto gaming field.";
            let metadata = Irc30Metadata::new("FooCoin", "FOO", 3)
                .with_description(description)
                .with_url("https://foocoin.io/".parse().unwrap())
                .with_logo_url(
                    "https://ipfs.io/ipfs/QmR36VFfo1hH2RAwVs4zVJ5btkopGip5cW7ydY4jUQBrkR"
                        .parse()
                        .unwrap(),
                );
            let json = serde_json::json!(
                {
                    "standard": "IRC30",
                    "name": metadata.name(),
                    "description": metadata.description(),
                    "decimals": metadata.decimals(),
                    "symbol": metadata.symbol(),
                    "url": metadata.url(),
                    "logoUrl": metadata.logo_url()
                }
            );
            let metadata_deser = serde_json::from_value::<Irc30Metadata>(json.clone()).unwrap();

            assert_eq!(metadata, metadata_deser);
            assert_eq!(json, serde_json::to_value(metadata).unwrap())
        }
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::{collections::BTreeMap, format};

    use serde::{de, Deserialize, Deserializer, Serialize};
    use serde_json::Value;

    use super::*;

    #[derive(Serialize)]
    struct MetadataFeatureDto {
        #[serde(rename = "type")]
        kind: u8,
        entries: BTreeMap<String, String>,
    }

    impl<'de> Deserialize<'de> for MetadataFeature {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match u8::try_from(
                    value
                        .get("type")
                        .and_then(Value::as_u64)
                        .ok_or_else(|| de::Error::custom("invalid metadata type"))?,
                )
                .map_err(|_| de::Error::custom("invalid metadata type: {e}"))?
                {
                    Self::KIND => {
                        let map: BTreeMap<String, String> = serde_json::from_value(
                            value
                                .get("entries")
                                .ok_or_else(|| de::Error::custom("missing metadata entries"))?
                                .clone(),
                        )
                        .map_err(|e| de::Error::custom(format!("cannot deserialize metadata feature: {e}")))?;

                        Self::try_from(
                            map.into_iter()
                                .map(|(key, value)| Ok((key, prefix_hex::decode::<Vec<u8>>(value)?)))
                                .collect::<Result<BTreeMap<String, Vec<u8>>, prefix_hex::Error>>()
                                .map_err(de::Error::custom)?,
                        )
                        .map_err(de::Error::custom)?
                    }
                    _ => return Err(de::Error::custom("invalid metadata feature")),
                },
            )
        }
    }

    impl From<&MetadataFeature> for MetadataFeatureDto {
        fn from(value: &MetadataFeature) -> Self {
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
                kind: MetadataFeature::KIND,
                entries,
            }
        }
    }
    impl Serialize for MetadataFeature {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            MetadataFeatureDto::from(self).serialize(s)
        }
    }
}
