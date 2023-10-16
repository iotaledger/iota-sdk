// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Convenience macro to implement and derive base features of identifiers.
#[macro_export]
macro_rules! impl_id {
    ($vis:vis $name:ident, $length:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(
            Clone,
            Copy,
            Eq,
            Hash,
            PartialEq,
            Ord,
            PartialOrd,
            derive_more::From,
            derive_more::AsRef,
            packable::Packable,
        )]
        #[as_ref(forward)]
        #[repr(transparent)]
        $vis struct $name([u8; $name::LENGTH]);

        impl $name {
            #[doc = concat!("The length of a [`", stringify!($ty),"`].")]
            $vis const LENGTH: usize = $length;

            #[doc = concat!("Creates a new [`", stringify!($ty),"`].")]
            $vis fn new(bytes: [u8; $name::LENGTH]) -> Self {
                Self::from(bytes)
            }

            #[doc = concat!("Creates a null [`", stringify!($ty),"`].")]
            pub fn null() -> Self {
                Self::from([0u8; $name::LENGTH])
            }

            #[doc = concat!("Checks if the [`", stringify!($ty),"`] is null.")]
            pub fn is_null(&self) -> bool {
                self.0.iter().all(|&b| b == 0)
            }
        }

        impl core::str::FromStr for $name {
            type Err = $crate::types::block::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($name::new(prefix_hex::decode(s).map_err($crate::types::block::Error::Hex)?))
            }
        }

        impl TryFrom<&alloc::string::String> for $name {
            type Error = $crate::types::block::Error;

            fn try_from(s: &alloc::string::String) -> Result<Self, Self::Error> {
                core::str::FromStr::from_str(s.as_str())
            }
        }

        impl TryFrom<&str> for $name {
            type Error = $crate::types::block::Error;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                core::str::FromStr::from_str(s)
            }
        }

        impl $crate::types::block::ConvertTo<$name> for &alloc::string::String {
            fn convert(self) -> Result<$name, $crate::types::block::Error> {
                self.try_into()
            }
        }

        impl $crate::types::block::ConvertTo<$name> for &str {
            fn convert(self) -> Result<$name, $crate::types::block::Error> {
                self.try_into()
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", prefix_hex::encode(self.0))
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({})", stringify!($name), self)
            }
        }

        impl core::ops::Deref for $name {
            type Target = [u8; $name::LENGTH];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
#[cfg(feature = "serde")]
pub(crate) use impl_id;

#[macro_export(local_inner_macros)]
macro_rules! impl_id_with_slot {
    ($hash_vis:vis $hash_name:ident, $hash_length:literal, $hash_doc:literal, $id_vis:vis $id_name:ident, $id_doc:literal) => {
        impl_id!($hash_vis $hash_name, $hash_length, $hash_doc);

        impl $hash_name {
            pub fn with_slot_index(self, slot_index: impl Into<$crate::types::block::slot::SlotIndex>) -> $id_name {
                $id_name {
                    hash: self,
                    slot_index: slot_index.into().to_le_bytes(),
                }
            }
        }

        #[doc = $id_doc]
        #[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, packable::Packable)]
        #[packable(unpack_error = $crate::types::block::Error)]
        #[repr(C)]
        $id_vis struct $id_name {
            pub(crate) hash: $hash_name,
            slot_index: [u8; core::mem::size_of::<$crate::types::block::slot::SlotIndex>()],
        }

        impl $id_name {
            #[doc = core::concat!("The length of a [`", core::stringify!($id_name),"`].")]
            pub const LENGTH: usize = $hash_name::LENGTH + core::mem::size_of::<$crate::types::block::slot::SlotIndex>();

            pub fn new(bytes: [u8; Self::LENGTH]) -> Self {
                unsafe { core::mem::transmute(bytes) }
            }

            #[doc = core::concat!("Returns the [`", core::stringify!($id_name),"`]'s hash part.")]
            pub fn hash(&self) -> &$hash_name {
                &self.hash
            }

            #[doc = core::concat!("Returns the [`", core::stringify!($id_name),"`]'s slot index part.")]
            pub fn slot_index(&self) -> $crate::types::block::slot::SlotIndex {
                unsafe {
                    #[cfg(target_endian = "little")]
                    {
                        core::mem::transmute(self.slot_index)
                    }

                    #[cfg(target_endian = "big")]
                    {
                        core::mem::transmute(self.slot_index.to_le())
                    }
                }
            }
        }

        impl AsRef<[u8]> for $id_name {
            fn as_ref(&self) -> &[u8] {
                unsafe { core::mem::transmute::<_, &[u8; Self::LENGTH]>(self) }
            }
        }

        impl core::str::FromStr for $id_name {
            type Err = $crate::types::block::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self::new(prefix_hex::decode(s).map_err($crate::types::block::Error::Hex)?))
            }
        }

        impl core::fmt::Debug for $id_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(core::stringify!($id_name))
                    .field("id", &self.to_string())
                    .field("slot_index", &self.slot_index())
                    .finish()
            }
        }

        impl core::fmt::Display for $id_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                prefix_hex::encode(self.as_ref()).fmt(f)
            }
        }

        impl TryFrom<&alloc::string::String> for $id_name {
            type Error = $crate::types::block::Error;

            fn try_from(s: &alloc::string::String) -> Result<Self, Self::Error> {
                core::str::FromStr::from_str(s.as_str())
            }
        }

        impl TryFrom<&str> for $id_name {
            type Error = $crate::types::block::Error;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                core::str::FromStr::from_str(s)
            }
        }

        impl $crate::types::block::ConvertTo<$id_name> for &alloc::string::String {
            fn convert(self) -> Result<$id_name, $crate::types::block::Error> {
                self.try_into()
            }
        }

        impl $crate::types::block::ConvertTo<$id_name> for &str {
            fn convert(self) -> Result<$id_name, $crate::types::block::Error> {
                self.try_into()
            }
        }

        impl core::ops::Deref for $id_name {
            type Target = [u8; Self::LENGTH];

            fn deref(&self) -> &Self::Target {
                unsafe { core::mem::transmute::<_, &[u8; Self::LENGTH]>(self) }
            }
        }
        #[cfg(feature = "serde")]
        string_serde_impl!($id_name);
    }
}

/// Convenience macro to serialize types to string via serde.
#[macro_export]
#[cfg(feature = "serde")]
macro_rules! string_serde_impl {
    ($type:ty) => {
        impl serde::Serialize for $type {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                use alloc::string::ToString;

                s.serialize_str(&self.to_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<$type, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct StringVisitor;

                impl<'de> serde::de::Visitor<'de> for StringVisitor {
                    type Value = $type;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        formatter.write_str("a string representing the value")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        let value = core::str::FromStr::from_str(v).map_err(serde::de::Error::custom)?;
                        Ok(value)
                    }
                }

                deserializer.deserialize_str(StringVisitor)
            }
        }
    };
}
#[cfg(feature = "serde")]
pub(crate) use string_serde_impl;

/// Convenience macro to work around the fact the `[bitflags]` crate does not yet support iterating over the
/// individual flags. This macro essentially creates the `[bitflags]` and puts the individual flags into an associated
/// constant `pub const ALL_FLAGS: &'static []`.
#[macro_export]
macro_rules! create_bitflags {
    ($(#[$meta:meta])* $vis:vis $Name:ident, $TagType:ty, [$(($FlagName:ident, $TypeName:ident),)+]) => {
        bitflags! {
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $vis struct $Name: $TagType {
                $(
                    #[doc = concat!("Signals the presence of a [`", stringify!($TypeName), "`].")]
                    const $FlagName = 1 << $TypeName::KIND;
                )*
            }
        }

        impl $Name {
            #[allow(dead_code)]
            /// Returns a slice of all possible base flags.
            $vis const ALL_FLAGS: &'static [$Name] = &[$($Name::$FlagName),*];
        }
    };
}
pub(crate) use create_bitflags;

#[macro_export]
macro_rules! impl_serde_typed_dto {
    ($base:ty, $dto:ty, $type_str:literal) => {
        impl<'de> Deserialize<'de> for $base {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let dto = <$dto>::deserialize(d)?;
                if dto.kind != Self::KIND {
                    return Err(serde::de::Error::custom(alloc::format!(
                        "invalid {} type: expected {}, found {}",
                        $type_str,
                        Self::KIND,
                        dto.kind
                    )));
                }
                dto.try_into().map_err(serde::de::Error::custom)
            }
        }

        impl Serialize for $base {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                <$dto>::from(self).serialize(s)
            }
        }
    };
}
