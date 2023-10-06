// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod array;
mod error;
mod map;
mod set;

use crypto::keys::bip44::Bip44;
pub use json::JsonValue as Value;
use primitive_types::U256;

pub use self::error::Error;

pub trait ToJson {
    fn to_json(&self) -> Value;
}

impl<T: ToJson + ?Sized> ToJson for &T {
    fn to_json(&self) -> Value {
        ToJson::to_json(*self)
    }
}

impl ToJson for str {
    fn to_json(&self) -> Value {
        self.to_owned().to_json()
    }
}

impl ToJson for Value {
    fn to_json(&self) -> Value {
        self.clone()
    }
}

pub trait FromJson
where
    Self::Error: From<Error>,
{
    type Error;

    fn from_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if value.is_null() {
            return Err(Error::missing_value::<Self>().into());
        }
        Self::from_non_null_json(value)
    }

    fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

macro_rules! def_ext_fns {
    ($($type:ty, $to_fn:ident),+$(,)?) => {
        $(
            fn $to_fn(&self) -> Result<$type, Error>;
        )+
    };
}

macro_rules! impl_ext_fns {
    ($($type:ty, $to_fn:ident, $as_fn:ident),+$(,)?) => {
        $(
            fn $to_fn(&self) -> Result<$type, Error> {
                self.$as_fn().ok_or_else(|| Error::wrong_type::<$type>(self))
            }
        )+
    };
}

macro_rules! impl_ext_maybe_str_fns {
    ($($type:ty, $to_fn:ident, $as_fn:ident),+$(,)?) => {
        $(
            fn $to_fn(&self) -> Result<$type, Error> {
                match self.to_str() {
                    Ok(s) => {
                        s.parse().map_err(|_| Error::wrong_type::<$type>(self))
                    }
                    Err(_) => {
                        self.$as_fn().ok_or_else(|| Error::wrong_type::<$type>(self))
                    }
                }
            }
        )+
    };
}

macro_rules! impl_ext_str_fns {
    ($($type:ty, $to_fn:ident),+$(,)?) => {
        $(
            fn $to_fn(&self) -> Result<$type, Error> {
                self.to_str()?.parse().map_err(|_| Error::wrong_type::<$type>(self))
            }
        )+
    };
}

pub trait JsonExt: Clone {
    def_ext_fns! {
        &str,  to_str,
        u8,    to_u8,
        u16,   to_u16,
        u32,   to_u32,
        u64,   to_u64,
        u128,  to_u128,
        usize, to_usize,
        i8,    to_i8,
        i16,   to_i16,
        i32,   to_i32,
        i64,   to_i64,
        i128,  to_i128,
        isize, to_isize,
        bool,  to_bool,
    }

    fn to_array<T: FromJson, const N: usize>(&self) -> Result<[T; N], T::Error>
    where
        T::Error: From<Error>,
    {
        self.clone().take_array()
    }

    fn take_array<T: FromJson, const N: usize>(&mut self) -> Result<[T; N], T::Error>
    where
        T::Error: From<Error>;

    fn to_vec<T: FromJson>(&self) -> Result<Vec<T>, T::Error>
    where
        T::Error: From<Error>,
    {
        self.clone().take_vec()
    }

    fn take_vec<T: FromJson>(&mut self) -> Result<Vec<T>, T::Error>
    where
        T::Error: From<Error>;

    fn to_value<T: FromJson>(&self) -> Result<T, T::Error> {
        self.clone().take_value()
    }

    fn take_value<T: FromJson>(&mut self) -> Result<T, T::Error>;

    fn to_opt<T: FromJson>(&self) -> Result<Option<T>, T::Error> {
        self.clone().take_opt()
    }

    fn to_opt_or_default<T: FromJson + Default>(&self) -> Result<T, T::Error> {
        Ok(self.to_opt::<T>()?.unwrap_or_default())
    }

    fn take_opt<T: FromJson>(&mut self) -> Result<Option<T>, T::Error>;

    fn take_opt_or_default<T: FromJson + Default>(&mut self) -> Result<T, T::Error> {
        Ok(self.take_opt::<T>()?.unwrap_or_default())
    }
}

impl JsonExt for Value {
    // These types are expected to be numbers
    impl_ext_fns! {
        &str,  to_str,   as_str,
        u8,    to_u8,    as_u8,
        u16,   to_u16,   as_u16,
        u32,   to_u32,   as_u32,
        i8,    to_i8,    as_i8,
        i16,   to_i16,   as_i16,
        i32,   to_i32,   as_i32,
        i64,   to_i64,   as_i64,
        bool,  to_bool,  as_bool,
    }

    // These types can be created from strings OR numbers
    impl_ext_maybe_str_fns! {
        u64,  to_u64, as_u64,
        usize, to_usize, as_usize,
        isize, to_isize, as_isize,
    }

    // These types can only be created from strings
    impl_ext_str_fns! {
        u128, to_u128,
        i128, to_i128,
    }

    fn take_array<T: FromJson, const N: usize>(&mut self) -> Result<[T; N], T::Error>
    where
        T::Error: From<Error>,
    {
        if self.is_array() && self.len() == N {
            let Ok(r) = self.take_vec::<T>()?.try_into() else {
                unreachable!()
            };
            Ok(r)
        } else {
            Err(Error::wrong_type::<[T; N]>(self).into())
        }
    }

    fn take_vec<T: FromJson>(&mut self) -> Result<Vec<T>, T::Error>
    where
        T::Error: From<Error>,
    {
        match self.take() {
            Value::Array(a) => Ok(a
                .into_iter()
                .map(FromJson::from_json)
                .collect::<Result<Vec<_>, T::Error>>())?,
            v => Err(Error::wrong_type::<Vec<T>>(v).into()),
        }
    }

    fn take_value<T: FromJson>(&mut self) -> Result<T, T::Error> {
        T::from_json(self.take())
    }

    fn take_opt<T: FromJson>(&mut self) -> Result<Option<T>, T::Error> {
        Option::from_json(self.take())
    }
}

impl<T: ToJson> ToJson for Option<T> {
    fn to_json(&self) -> Value {
        match self {
            Some(t) => t.to_json(),
            None => Value::Null,
        }
    }
}

impl<T: FromJson> FromJson for Option<T> {
    type Error = T::Error;

    fn from_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(if value.is_null() {
            None
        } else {
            Self::from_non_null_json(value)?
        })
    }

    fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Some(T::from_non_null_json(value)?))
    }
}

impl ToJson for String {
    fn to_json(&self) -> Value {
        self.clone().into()
    }
}

impl FromJson for String {
    type Error = Error;

    fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if let Value::String(s) = value {
            Ok(s)
        } else {
            Err(Error::wrong_type::<String>(value))
        }
    }
}

macro_rules! impl_json_via {
    ($($type:ty, $fn:ident),+$(,)?) => {
        $(
            impl ToJson for $type {
                fn to_json(&self) -> Value {
                    (*self).into()
                }
            }

            impl FromJson for $type {
                type Error = Error;

                fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
                where
                    Self: Sized,
                {
                    value.$fn()
                }
            }
        )+
    };
}
#[rustfmt::skip]
impl_json_via!(
    u8,    to_u8, 
    u16,   to_u16, 
    u32,   to_u32, 
    usize, to_usize,
    i8,    to_i8, 
    i16,   to_i16, 
    i32,   to_i32,
    i64,   to_i64,
    isize, to_isize,
    bool,  to_bool
);

// Special impls for u64 which cannot fit into json values

macro_rules! impl_json_via_str {
    ($($type:ty, $fn:ident),+$(,)?) => {
        $(
            impl ToJson for $type {
                fn to_json(&self) -> Value {
                    self.to_string().into()
                }
            }

            impl FromJson for $type {
                type Error = Error;

                fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
                where
                    Self: Sized,
                {
                    value.$fn()
                }
            }
        )+
    };
}
#[rustfmt::skip]
impl_json_via_str!(
    u64,  to_u64,
    u128, to_u128,
    i128, to_i128,
);

impl ToJson for U256 {
    fn to_json(&self) -> Value {
        <[u8; 32]>::from(*self).to_json()
    }
}

impl FromJson for U256 {
    type Error = Error;

    fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let bytes = value.take_vec::<u8>()?;
        match <[u8; 32]>::try_from(bytes) {
            Ok(r) => Ok(r.into()),
            Err(bytes) => Err(Error::wrong_type::<U256>(format!("{:?}", bytes))),
        }
    }
}

impl ToJson for Bip44 {
    fn to_json(&self) -> Value {
        crate::json!({
            "coin_type": self.coin_type,
            "account": self.account,
            "change": self.change,
            "address_index": self.address_index,
        })
    }
}

impl FromJson for Bip44 {
    type Error = Error;

    fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            coin_type: value["coin_type"].to_u32()?,
            account: value["account"].to_u32()?,
            change: value["change"].to_u32()?,
            address_index: value["address_index"].to_u32()?,
        })
    }
}

#[macro_export]
macro_rules! json {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: json!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        $crate::json_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        $crate::json_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        $crate::json!(@array [$($elems,)* $crate::json!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        $crate::json!(@array [$($elems,)* $crate::json!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        $crate::json!(@array [$($elems,)* $crate::json!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        $crate::json!(@array [$($elems,)* $crate::json!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        $crate::json!(@array [$($elems,)* $crate::json!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        $crate::json!(@array [$($elems,)* $crate::json!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        $crate::json!(@array [$($elems,)* $crate::json!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        $crate::json!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        $crate::json_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: json!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        $crate::json!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        $crate::json_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        $crate::json!(@object $object [$($key)+] ($crate::json!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        $crate::json!(@object $object [$($key)+] ($crate::json!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        $crate::json!(@object $object [$($key)+] ($crate::json!(false)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        $crate::json!(@object $object [$($key)+] ($crate::json!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        $crate::json!(@object $object [$($key)+] ($crate::json!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        $crate::json!(@object $object [$($key)+] ($crate::json!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        $crate::json!(@object $object [$($key)+] ($crate::json!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        $crate::json!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        $crate::json!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        $crate::json_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        $crate::json_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        $crate::json!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Refuse to absorb colon token into key expression.
    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        $crate::json_expect_expr_comma!($($unexpected)+);
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        $crate::json!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: json!($($json)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        ::json::JsonValue::Null
    };

    (true) => {
        ::json::JsonValue::Bool(true)
    };

    (false) => {
        ::json::JsonValue::Bool(false)
    };

    ([]) => {
        ::json::JsonValue::Array(json_internal_vec![])
    };

    ([ $($tt:tt)+ ]) => {
        ::json::JsonValue::Array(json!(@array [] $($tt)+))
    };

    ({}) => {
        ::json::JsonValue::Object(::json::object::Object::new())
    };

    ({ $($tt:tt)+ }) => {
        ::json::JsonValue::Object({
            let mut object = ::json::object::Object::new();
            $crate::json!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        $crate::utils::json::ToJson::to_json(&$other)
    };
}

// The json_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! json_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}

#[cfg(test)]
mod test {
    use alloc::collections::{BTreeMap, BTreeSet};
    use std::collections::{HashMap, HashSet};

    use super::{
        array::ArrayError,
        set::{OrderedSetError, SetError},
        *,
    };

    #[derive(Debug, Eq, PartialEq)]
    struct JsonUnsigned {
        u8: u8,
        u16: u16,
        u32: u32,
        u64: u64,
        u128: u128,
        usize: usize,
    }

    impl ToJson for JsonUnsigned {
        fn to_json(&self) -> Value {
            json!({
                "u8": self.u8,
                "u16": self.u16,
                "u32": self.u32,
                "u64": self.u64,
                "u128": self.u128,
                "usize": self.usize,
            })
        }
    }

    impl FromJson for JsonUnsigned {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            Ok(Self {
                u8: value["u8"].take_value()?,
                u16: value["u16"].take_value()?,
                u32: value["u32"].take_value()?,
                u64: value["u64"].take_value()?,
                u128: value["u128"].take_value()?,
                usize: value["usize"].take_value()?,
            })
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    struct JsonSigned {
        i8: i8,
        i16: i16,
        i32: i32,
        i64: i64,
        i128: i128,
        isize: isize,
    }

    impl ToJson for JsonSigned {
        fn to_json(&self) -> Value {
            json!({
                "i8": self.i8,
                "i16": self.i16,
                "i32": self.i32,
                "i64": self.i64,
                "i128": self.i128,
                "isize": self.isize
            })
        }
    }

    impl FromJson for JsonSigned {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            Ok(Self {
                i8: value["i8"].take_value()?,
                i16: value["i16"].take_value()?,
                i32: value["i32"].take_value()?,
                i64: value["i64"].take_value()?,
                i128: value["i128"].take_value()?,
                isize: value["isize"].take_value()?,
            })
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    struct JsonArrays {
        array: [usize; 9],
        vec: Vec<usize>,
        boxed: Box<[usize]>,
        set: HashSet<usize>,
        ordered_set: BTreeSet<usize>,
    }

    impl ToJson for JsonArrays {
        fn to_json(&self) -> Value {
            json!({
                "array": self.array,
                "vec": self.vec,
                "boxed": self.boxed,
                "set": self.set,
                "ordered_set": self.ordered_set,
            })
        }
    }

    impl FromJson for JsonArrays {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            Ok(Self {
                array: value["array"].take_value()?,
                vec: value["vec"].take_value()?,
                boxed: value["boxed"].take_value()?,
                set: value["set"].take_value()?,
                ordered_set: value["ordered_set"].take_value()?,
            })
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    struct JsonTest {
        signed: JsonSigned,
        unsigned: JsonUnsigned,
        optional: Option<String>,
        arrays: JsonArrays,
        map: HashMap<usize, String>,
        ordered_map: BTreeMap<usize, String>,
    }

    impl ToJson for JsonTest {
        fn to_json(&self) -> Value {
            json!({
                "signed": self.signed,
                "unsigned": self.unsigned,
                "optional": self.optional,
                "arrays": self.arrays,
                "map": self.map,
                "ordered_map": self.ordered_map,
            })
        }
    }

    impl FromJson for JsonTest {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            Ok(Self {
                signed: value["signed"].take_value()?,
                unsigned: value["unsigned"].take_value()?,
                optional: value["optional"].take_value()?,
                arrays: value["arrays"].take_value()?,
                map: value["map"].take_value()?,
                ordered_map: value["ordered_map"].take_value()?,
            })
        }
    }

    #[test]
    fn test_json_valid() {
        let data = json!({
            "signed": {
                "i8": 5,
                "i16": -6,
                "i32": 7,
                "i64": -8,
                "i128": "9",
                "isize": "-10",
            },
            "unsigned": {
                "u8": 5,
                "u16": 6,
                "u32": 7,
                "u64": 8,
                "u128": "9",
                "usize": 10,
            },
            "optional": null,
            "arrays": {
                "array": [2, 1, 2, 1, 2, 1, 2, 1, 2],
                "vec": [1, 2, 3, 4, 5, 4, 3, 2, 1],
                "boxed": [5, 4, 3, 2, 1, 2, 3, 4, 5],
                "set": [5, 4, 3, 2, 1],
                "ordered_set": [1, 2, 3, 4, 5],
            },
            "map": {
                "3": "three",
                "1": "one",
                "2": "two",
                "5": "five",
                "4": "four",
            },
            "ordered_map": {
                "1": "one",
                "2": "two",
                "3": "three",
                "4": "four",
                "5": "five",
            }
        });

        let test_val = data.to_value::<JsonTest>().unwrap();
        let test_val_2 = test_val.to_json().to_value::<JsonTest>().unwrap();
        assert_eq!(test_val, test_val_2);
    }

    #[test]
    fn test_out_of_range() {
        // The macro will use ToJson which converts 128 to a string
        let data = json!(u128::MAX);
        data.to_value::<u128>().unwrap();

        // The parse fn will not
        let data = json::parse(&u128::MAX.to_string()).unwrap();
        assert!(
            matches!(data.to_value::<u128>(), Err(Error::WrongType { expected, found }) if expected == "&str" && found == "3.402823669209384634e38")
        );
    }

    #[test]
    fn test_missing() {
        let data = json!(null);
        data.to_value::<Option<String>>().unwrap();
        assert!(matches!(data.to_value::<String>(), Err(Error::MissingValue(v)) if v == "alloc::string::String"));
    }

    #[test]
    fn test_arrays() {
        let data = json!([2, 1, 3, 5, 4]);
        data.to_value::<[usize; 5]>().unwrap();
        data.to_value::<Vec<usize>>().unwrap();
        data.to_value::<Box<[usize]>>().unwrap();
        data.to_value::<HashSet<usize>>().unwrap();
        assert!(
            matches!(data.to_value::<BTreeSet<usize>>(), Err(Error::OrderedSet(OrderedSetError::Unordered(v))) if v == "1")
        );
        assert!(
            matches!(data.to_value::<[usize; 6]>(), Err(Error::Array(ArrayError::WrongSize { expected, found })) if expected == 6 && found == 5)
        );
        let data = json!([2, 2, 3, 5, 4]);
        data.to_value::<[usize; 5]>().unwrap();
        data.to_value::<Vec<usize>>().unwrap();
        data.to_value::<Box<[usize]>>().unwrap();
        assert!(matches!(data.to_value::<HashSet<usize>>(), Err(Error::Set(SetError::Duplicate(v))) if v == "2"));
        assert!(matches!(data.to_value::<BTreeSet<usize>>(), Err(Error::Set(SetError::Duplicate(v))) if v == "2"));
    }

    #[test]
    fn test_objects() {
        let data = json!({
            "3": "three",
            "1": "one",
            "2": "two",
            "5": "five",
            "4": "four",
        });
        data.to_value::<HashMap<String, String>>().unwrap();
        data.to_value::<HashMap<usize, String>>().unwrap();
        assert!(
            matches!(data.to_value::<BTreeMap<String, String>>(), Err(Error::OrderedSet(OrderedSetError::Unordered(v))) if v == "1")
        );
        assert!(
            matches!(data.to_value::<BTreeMap<usize, String>>(), Err(Error::OrderedSet(OrderedSetError::Unordered(v))) if v == "1")
        );

        let data = json!({
            "1": "one",
            "2": "two",
            "3": "three",
            "4": "four",
            "5": "five",
        });
        data.to_value::<HashMap<String, String>>().unwrap();
        data.to_value::<HashMap<usize, String>>().unwrap();
        data.to_value::<BTreeMap<String, String>>().unwrap();
        data.to_value::<BTreeMap<usize, String>>().unwrap();
    }
}
