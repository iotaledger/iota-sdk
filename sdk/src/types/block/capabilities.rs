// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use core::{convert::Infallible, marker::PhantomData};

use derive_more::Deref;
use packable::{
    error::{UnpackError, UnpackErrorExt},
    prefix::{BoxedSlicePrefix, UnpackPrefixError},
    Packable,
};

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
#[allow(missing_docs)]
pub enum CapabilityError {
    #[display(fmt = "invalid capabilities count: {_0}")]
    InvalidCount(<u8 as TryFrom<usize>>::Error),
    #[display(fmt = "invalid capability byte at index {index}: {byte:x}")]
    InvalidByte { index: usize, byte: u8 },
    #[display(fmt = "capability bytes have trailing zeroes")]
    TrailingBytes,
}

#[cfg(feature = "std")]
impl std::error::Error for CapabilityError {}

impl From<Infallible> for CapabilityError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}

/// A list of bitflags that represent capabilities.
#[derive(Deref)]
#[repr(transparent)]
pub struct Capabilities<Flag> {
    #[deref]
    bytes: BoxedSlicePrefix<u8, u8>,
    _flag: PhantomData<Flag>,
}

impl<Flag> Capabilities<Flag> {
    /// Returns a [`Capabilities`] with every possible flag disabled.
    pub fn none() -> Self {
        Self::default()
    }

    /// Returns whether every possible flag is disabled.
    pub fn is_none(&self) -> bool {
        self.iter().all(|b| 0.eq(b))
    }

    /// Disables every possible flag.
    pub fn set_none(&mut self) -> &mut Self {
        *self = Default::default();
        self
    }
}

impl<Flag: CapabilityFlag> core::fmt::Debug for Capabilities<Flag> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.capabilities_iter()).finish()
    }
}

impl<Flag: CapabilityFlag> Capabilities<Flag> {
    /// Try to create capabilities from serialized bytes. Bytes with trailing zeroes are invalid.
    pub fn from_bytes(bytes: impl Into<Box<[u8]>>) -> Result<Self, CapabilityError> {
        Self::from_prefix_box_slice(bytes.into().try_into().map_err(CapabilityError::InvalidCount)?)
    }

    /// Try to create capabilities from serialized bytes. Bytes with trailing zeroes are invalid.
    pub(crate) fn from_prefix_box_slice(bytes: BoxedSlicePrefix<u8, u8>) -> Result<Self, CapabilityError> {
        // Check if there is a trailing zero.
        if bytes.last().map(|b| *b == 0).unwrap_or_default() {
            return Err(CapabilityError::TrailingBytes);
        }
        // Check if the bytes are valid instances of the flag type.
        for (index, &byte) in bytes.iter().enumerate() {
            // Get the max value of the flags at this index
            let mut b = 0;
            for flag in Flag::all().filter(|f| f.index() == index) {
                b |= flag.as_byte();
            }
            // Check whether the byte contains erroneous bits by using the max value as a mask
            if b | byte != b {
                return Err(CapabilityError::InvalidByte { index, byte });
            }
        }
        Ok(Self {
            bytes,
            _flag: PhantomData,
        })
    }

    /// Returns a [`Capabilities`] with every possible flag enabled.
    pub fn all() -> Self {
        let mut res = Self::default();
        res.set_all();
        res
    }

    /// Returns whether every possible flag is enabled.
    pub fn is_all(&self) -> bool {
        Flag::all().all(|flag| self.has_capability(flag))
    }

    /// Enables every possible flag.
    pub fn set_all(&mut self) -> &mut Self {
        for flag in Flag::all() {
            self.add_capability(flag);
        }
        self
    }

    /// Enables a given flag.
    pub fn add_capability(&mut self, flag: Flag) -> &mut Self {
        if self.bytes.len() <= flag.index() {
            let mut v = Box::<[_]>::from(self.bytes.clone()).into_vec();
            v.resize(flag.index() + 1, 0);
            // Unwrap: safe because the indexes are within u8 bounds
            self.bytes = v.into_boxed_slice().try_into().unwrap();
        }
        self.bytes[flag.index()] |= flag.as_byte();
        self
    }

    /// Enables a given set of flags.
    pub fn add_capabilities(&mut self, flags: impl IntoIterator<Item = Flag>) -> &mut Self {
        for flag in flags {
            self.add_capability(flag);
        }
        self
    }

    /// Enables a given set of flags.
    pub fn with_capabilities(mut self, flags: impl IntoIterator<Item = Flag>) -> Self {
        self.add_capabilities(flags);
        self
    }

    /// Overwrites the flags with a given set of flags.
    pub fn set_capabilities(&mut self, flags: impl IntoIterator<Item = Flag>) -> &mut Self {
        *self = Self::default().with_capabilities(flags);
        self
    }

    /// Returns whether a given flag is enabled.
    pub fn has_capability(&self, flag: Flag) -> bool {
        self.get(flag.index())
            .map(|byte| byte & flag.as_byte() == flag.as_byte())
            .unwrap_or_default()
    }

    /// Returns whether a given set of flags are enabled.
    pub fn has_capabilities(&self, flags: impl IntoIterator<Item = Flag>) -> bool {
        flags.into_iter().all(|flag| self.has_capability(flag))
    }

    /// Returns an iterator over all enabled flags.
    pub fn capabilities_iter(&self) -> impl Iterator<Item = Flag> + '_ {
        self.iter().enumerate().flat_map(|(idx, byte)| {
            Flag::all().filter(move |f| (idx == f.index() && byte & f.as_byte() == f.as_byte()))
        })
    }
}

impl<Flag> Default for Capabilities<Flag> {
    fn default() -> Self {
        Self {
            bytes: Default::default(),
            _flag: PhantomData,
        }
    }
}

impl<Flag> Clone for Capabilities<Flag> {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
            _flag: PhantomData,
        }
    }
}

impl<Flag> PartialEq for Capabilities<Flag> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl<Flag> Eq for Capabilities<Flag> {}

impl<Flag> PartialOrd for Capabilities<Flag> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Flag> Ord for Capabilities<Flag> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.bytes.cmp(&other.bytes)
    }
}

impl<Flag> core::hash::Hash for Capabilities<Flag> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

impl<I: IntoIterator<Item = Flag>, Flag: CapabilityFlag> From<I> for Capabilities<Flag> {
    fn from(value: I) -> Self {
        Self::default().with_capabilities(value)
    }
}

impl<Flag: 'static + CapabilityFlag> Packable for Capabilities<Flag> {
    type UnpackError = CapabilityError;
    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.bytes.pack(packer)?;
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Self::from_prefix_box_slice(
            BoxedSlicePrefix::unpack(unpacker, visitor)
                .map_packable_err(|e| match e {
                    UnpackPrefixError::Item(i) | UnpackPrefixError::Prefix(i) => i,
                })
                .coerce()?,
        )
        .map_err(UnpackError::Packable)
    }
}

pub trait CapabilityFlag: core::fmt::Debug {
    type Iterator: Iterator<Item = Self>;

    /// Converts the flag into the byte representation.
    fn as_byte(&self) -> u8;

    /// Returns the index in [`Capabilities`] to which this flag is applied.
    fn index(&self) -> usize;

    /// Returns an iterator over all flags.
    fn all() -> Self::Iterator;
}

#[cfg(feature = "serde")]
mod serde {
    use ::serde::{Deserialize, Serialize};

    use super::*;

    impl<Flag> Serialize for Capabilities<Flag> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            crate::utils::serde::boxed_slice_prefix_hex_bytes::serialize(&self.bytes, serializer)
        }
    }

    impl<'de, Flag: CapabilityFlag> Deserialize<'de> for Capabilities<Flag> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::from_prefix_box_slice(crate::utils::serde::boxed_slice_prefix_hex_bytes::deserialize(
                deserializer,
            )?)
            .map_err(::serde::de::Error::custom)
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[derive(Debug)]
    #[allow(unused)]
    enum TestFlag {
        Val1,
        Val2,
        Val3,
        Val4,
        Val5,
        Val6,
        Val7,
        Val8,
        Val9,
    }

    impl TestFlag {
        const VAL_1: u8 = 0b00000001;
        const VAL_2: u8 = 0b00000010;
        const VAL_3: u8 = 0b00000100;
        const VAL_4: u8 = 0b00001000;
        const VAL_5: u8 = 0b00010000;
        const VAL_6: u8 = 0b00100000;
        const VAL_7: u8 = 0b01000000;
        const VAL_8: u8 = 0b10000000;
        const VAL_9: u8 = 0b00000001;
    }

    impl CapabilityFlag for TestFlag {
        type Iterator = core::array::IntoIter<Self, 9>;

        fn as_byte(&self) -> u8 {
            match self {
                Self::Val1 => Self::VAL_1,
                Self::Val2 => Self::VAL_2,
                Self::Val3 => Self::VAL_3,
                Self::Val4 => Self::VAL_4,
                Self::Val5 => Self::VAL_5,
                Self::Val6 => Self::VAL_6,
                Self::Val7 => Self::VAL_7,
                Self::Val8 => Self::VAL_8,
                Self::Val9 => Self::VAL_9,
            }
        }

        fn index(&self) -> usize {
            match self {
                Self::Val1
                | Self::Val2
                | Self::Val3
                | Self::Val4
                | Self::Val5
                | Self::Val6
                | Self::Val7
                | Self::Val8 => 0,
                Self::Val9 => 1,
            }
        }

        fn all() -> Self::Iterator {
            [
                Self::Val1,
                Self::Val2,
                Self::Val3,
                Self::Val4,
                Self::Val5,
                Self::Val6,
                Self::Val7,
                Self::Val8,
                Self::Val9,
            ]
            .into_iter()
        }
    }

    #[test]
    fn test_valid() {
        let capability_bytes = [TestFlag::VAL_1 | TestFlag::VAL_3 | TestFlag::VAL_4];
        let deser = Capabilities::<TestFlag>::from_bytes(capability_bytes).unwrap();
        let built = Capabilities::default().with_capabilities([TestFlag::Val1, TestFlag::Val3, TestFlag::Val4]);
        assert_eq!(deser, built);

        let capability_bytes = [0, TestFlag::VAL_9];
        let deser = Capabilities::<TestFlag>::from_bytes(capability_bytes).unwrap();
        let built = Capabilities::default().with_capabilities([TestFlag::Val9]);
        assert_eq!(deser, built);
    }

    #[test]
    fn test_out_of_range() {
        let capability_bytes = [TestFlag::VAL_1 | TestFlag::VAL_4, TestFlag::VAL_9, TestFlag::VAL_3];
        assert_eq!(
            Capabilities::<TestFlag>::from_bytes(capability_bytes),
            Err(CapabilityError::InvalidByte {
                index: 2,
                byte: TestFlag::VAL_3
            })
        );
    }

    #[test]
    fn test_trailing() {
        let capability_bytes = [0, 0];
        assert_eq!(
            Capabilities::<TestFlag>::from_bytes(capability_bytes),
            Err(CapabilityError::TrailingBytes)
        );

        let capability_bytes = [TestFlag::VAL_1 | TestFlag::VAL_4, 0];
        assert_eq!(
            Capabilities::<TestFlag>::from_bytes(capability_bytes),
            Err(CapabilityError::TrailingBytes)
        );
    }

    #[test]
    fn test_invalid_byte() {
        let capability_bytes = [TestFlag::VAL_1 | TestFlag::VAL_3, TestFlag::VAL_9 | TestFlag::VAL_2];
        assert_eq!(
            Capabilities::<TestFlag>::from_bytes(capability_bytes),
            Err(CapabilityError::InvalidByte {
                index: 1,
                byte: TestFlag::VAL_9 | TestFlag::VAL_2
            })
        );
    }
}
