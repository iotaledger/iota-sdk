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

#[macro_export]
macro_rules! def_is_as_opt {
    ($type:ty => $($name:ident),+$(,)?) => {
        paste::paste! {
            $(
                #[doc = "Checks whether the " [<$type:snake>] " is a(n) [`" [<$name $type>] "`]."]
                pub fn [<is_ $name:snake>](&self) -> bool {
                    matches!(self, Self::$name(_))
                }

                #[doc = "Gets the " [<$type:snake>] " as an actual [`" [<$name $type>] "`]."]
                #[doc = "PANIC: do not call on a non-" [<$name>] " " [<$type:snake>] "."]
                pub fn [<as_ $name:snake>](&self) -> &[<$name $type>] {
                    if let Self::$name(address) = self {
                        address
                    } else {
                        panic!("{} called on a non-{} {}", stringify!([<as_ $name>]), stringify!([<$name>]), stringify!($type:snake));
                    }
                }

                #[doc = "Gets the " [<$type:snake>] " as an actual [`" [<$name $type>] "`], if it is one."]
                pub fn [<as_ $name:snake _opt>](&self) -> Option<&[<$name $type>]> {
                    if let Self::$name(address) = self {
                        Some(address)
                    } else {
                        None
                    }
                }
            )+
        }
    };
}
