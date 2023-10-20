// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use core::marker::PhantomData;

use derive_more::Deref;
use packable::{
    error::UnpackErrorExt,
    prefix::{BoxedSlicePrefix, UnpackPrefixError},
    Packable,
};

/// A list of bitflags that represent capabilities.
#[derive(Debug, Deref)]
#[repr(transparent)]
pub struct Capabilities<Flag> {
    #[deref]
    bytes: BoxedSlicePrefix<u8, u8>,
    _flag: PhantomData<Flag>,
}

impl<Flag> Capabilities<Flag> {
    pub(crate) fn from_bytes(bytes: BoxedSlicePrefix<u8, u8>) -> Self {
        Self {
            bytes,
            _flag: PhantomData,
        }
    }

    /// Returns a [`Capabilities`] with every possible flag disabled.
    pub fn none() -> Self {
        Self::default()
    }

    /// Returns whether every possible flag is disabled.
    pub fn is_none(&self) -> bool {
        self.iter().all(|b| 0.eq(b))
    }
}

impl<Flag: CapabilityFlag> Capabilities<Flag> {
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

    /// Disables every possible flag.
    pub fn set_none(&mut self) -> &mut Self {
        *self = Default::default();
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
        Some(self.cmp(&other))
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

impl<Flag: 'static> Packable for Capabilities<Flag> {
    type UnpackError = crate::types::block::Error;
    type UnpackVisitor = ();

    fn pack<P: packable::packer::Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        if !self.is_none() {
            self.bytes.pack(packer)?;
        } else {
            0_u8.pack(packer)?;
        }
        Ok(())
    }

    fn unpack<U: packable::unpacker::Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, packable::error::UnpackError<Self::UnpackError, U::Error>> {
        Ok(Self::from_bytes(
            BoxedSlicePrefix::unpack::<_, VERIFY>(unpacker, visitor)
                .map_packable_err(|e| match e {
                    UnpackPrefixError::Item(i) | UnpackPrefixError::Prefix(i) => i,
                })
                .coerce()?,
        ))
    }
}

pub trait CapabilityFlag {
    type Iterator: Iterator<Item = Self>;

    /// Converts the flag into the byte representation.
    fn as_byte(&self) -> u8;

    /// Returns the index in [`Capabilities`] to which this flag is applied.
    fn index(&self) -> usize;

    /// Returns an iterator over all flags.
    fn all() -> Self::Iterator;
}
