// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Convenience macro to implement and derive base features of identifiers.
#[macro_export(local_inner_macros)]
macro_rules! impl_id {
    (
        $(#[$hash_meta:meta])*
        $hash_vis:vis $hash_name:ident {
            $len_vis:vis const LENGTH: usize = $length:literal;
        }
        $(
            $(#[$id_meta:meta])*
            $id_vis:vis $id_name:ident;
        )?
    ) => {
        $(#[$hash_meta])*
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
        $hash_vis struct $hash_name([u8; Self::LENGTH]);

        impl $hash_name {
            #[doc = core::concat!("The length of a [`", core::stringify!($hash_name), "`].")]
            $len_vis const LENGTH: usize = $length;

            #[doc = core::concat!("Creates a new [`", core::stringify!($hash_name), "`].")]
            $hash_vis const fn new(bytes: [u8; Self::LENGTH]) -> Self {
                Self(bytes)
            }

            #[doc = core::concat!("Creates a null [`", core::stringify!($hash_name), "`].")]
            $hash_vis const fn null() -> Self {
                Self([0u8; Self::LENGTH])
            }

            #[doc = core::concat!("Checks if the [`", core::stringify!($hash_name), "`] is null.")]
            $hash_vis fn is_null(&self) -> bool {
                self.0.iter().all(|&b| b == 0)
            }
        }

        impl core::str::FromStr for $hash_name {
            type Err = $crate::types::block::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self::new(prefix_hex::decode(s).map_err($crate::types::block::Error::Hex)?))
            }
        }

        impl TryFrom<&alloc::string::String> for $hash_name {
            type Error = $crate::types::block::Error;

            fn try_from(s: &alloc::string::String) -> Result<Self, Self::Error> {
                core::str::FromStr::from_str(s.as_str())
            }
        }

        impl TryFrom<&str> for $hash_name {
            type Error = $crate::types::block::Error;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                core::str::FromStr::from_str(s)
            }
        }

        impl $crate::utils::ConvertTo<$hash_name> for &alloc::string::String {
            fn convert(self) -> Result<$hash_name, $crate::types::block::Error> {
                self.try_into()
            }
        }

        impl $crate::utils::ConvertTo<$hash_name> for &str {
            fn convert(self) -> Result<$hash_name, $crate::types::block::Error> {
                self.try_into()
            }
        }

        impl core::fmt::Display for $hash_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::write!(f, "{}", prefix_hex::encode(self.0))
            }
        }

        impl core::fmt::Debug for $hash_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::write!(f, "{}({})", core::stringify!($hash_name), self)
            }
        }

        impl core::ops::Deref for $hash_name {
            type Target = [u8; Self::LENGTH];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[cfg(feature = "serde")]
        string_serde_impl!($hash_name);

        $(
            paste::paste!{
                impl $hash_name {
                    pub fn [<into_ $id_name:snake>](self, slot_index: impl Into<$crate::types::block::slot::SlotIndex>) -> $id_name {
                        $id_name {
                            hash: self,
                            slot_index: slot_index.into().to_le_bytes(),
                        }
                    }

                    pub const fn [<const_into_ $id_name:snake>](self, slot_index: $crate::types::block::slot::SlotIndex) -> $id_name {
                        $id_name {
                            hash: self,
                            slot_index: slot_index.0.to_le_bytes(),
                        }
                    }
                }
            }

            $(#[$id_meta])*
            #[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, packable::Packable)]
            #[packable(unpack_error = $crate::types::block::Error)]
            #[repr(C)]
            $id_vis struct $id_name {
                pub(crate) hash: $hash_name,
                slot_index: [u8; core::mem::size_of::<$crate::types::block::slot::SlotIndex>()],
            }

            impl $id_name {
                #[doc = core::concat!("The length of a [`", core::stringify!($id_name), "`].")]
                pub const LENGTH: usize = $hash_name::LENGTH + core::mem::size_of::<$crate::types::block::slot::SlotIndex>();

                pub fn new(bytes: [u8; Self::LENGTH]) -> Self {
                    unsafe { core::mem::transmute(bytes) }
                }

                #[doc = core::concat!("Returns the [`", core::stringify!($id_name), "`]'s hash part.")]
                pub fn hash(&self) -> &$hash_name {
                    &self.hash
                }

                #[doc = core::concat!("Returns the [`", core::stringify!($id_name), "`]'s slot index part.")]
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
                        .field("id", &alloc::string::ToString::to_string(self))
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

            impl $crate::utils::ConvertTo<$id_name> for &alloc::string::String {
                fn convert(self) -> Result<$id_name, $crate::types::block::Error> {
                    self.try_into()
                }
            }

            impl $crate::utils::ConvertTo<$id_name> for &str {
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
        )?
    };
}

/// Convenience macro to serialize types to string via serde.
#[cfg(feature = "serde")]
#[macro_export]
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
    ($type:ty: $($name:ident),+$(,)?) => {
        paste::paste! {
            $(
                #[doc = "Checks whether the " [<$type:snake>] " is a(n) [`" [<$name $type>] "`]."]
                pub fn [<is_ $name:snake>](&self) -> bool {
                    matches!(self, Self::$name(_))
                }

                #[doc = "Gets the " [<$type:snake>] " as an actual [`" [<$name $type>] "`]."]
                #[doc = "PANIC: do not call on a non-" [<$name>] " " [<$type:snake>] "."]
                pub fn [<as_ $name:snake>](&self) -> &[<$name $type>] {
                    #[allow(irrefutable_let_patterns)]
                    if let Self::$name(v) = self {
                        v
                    } else {
                        panic!("{} called on a non-{} {}", stringify!([<as_ $name:snake>]), stringify!([<$name>]), stringify!([<$type:snake>]));
                    }
                }

                #[doc = "Gets the " [<$type:snake>] " as an actual [`" [<$name $type>] "`], if it is one."]
                pub fn [<as_ $name:snake _opt>](&self) -> Option<&[<$name $type>]> {
                    #[allow(irrefutable_let_patterns)]
                    if let Self::$name(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! impl_deserialize_untagged {
    ($type:ty: $($name:ident),+$(,)?) => {
        paste::paste!{
            impl<'de> serde::Deserialize<'de> for $type {
                fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                    let value = serde_json::Value::deserialize(d)?;
                    Ok(
                        match value
                            .get("type")
                            .and_then(serde_json::Value::as_u64)
                            .ok_or_else(|| serde::de::Error::custom(core::concat!("invalid ", core::stringify!($type), " type")))? as u8
                        {
                            $(
                            [<$name $type>]::KIND => {
                                Self::from([<$name $type>]::deserialize(value).map_err(|e| {
                                    serde::de::Error::custom(alloc::format!(core::concat!("cannot deserialize ", core::stringify!($name), core::stringify!($type), ": {}"), e))
                                })?)
                            }
                            )+
                            _ => return Err(serde::de::Error::custom(core::concat!("invalid ", core::stringify!($type), " type"))),
                        },
                    )
                }
            }
        }
    }
}
