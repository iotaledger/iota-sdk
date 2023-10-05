// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use json::JsonValue as Value;
use primitive_types::U256;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    WrongArraySize { expected: usize, found: usize },
    MissingValue,
    WrongType { expected: String, found: String },
}

impl Error {
    pub fn wrong_type<E>(found: impl alloc::string::ToString) -> Self {
        Self::WrongType {
            expected: core::any::type_name::<E>().to_owned(),
            found: found.to_string(),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WrongArraySize { expected, found } => {
                write!(f, "wrong array size: expected {expected}, found {found}")
            }
            Self::MissingValue => write!(f, "missing value"),
            Self::WrongType { expected, found } => {
                write!(f, "wrong type: expected {expected}, found {found}")
            }
        }
    }
}

pub trait ToJson {
    fn to_json(&self) -> Value;
}

impl<T: ToJson + ?Sized> ToJson for &T {
    fn to_json(&self) -> Value {
        ToJson::to_json(*self)
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
            return Err(Error::MissingValue.into());
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

pub trait JsonExt: Clone {
    def_ext_fns! {
        &str, to_str,
        u8,   to_u8,
        u16,  to_u16,
        u32,  to_u32,
        // u64, to_u64,
        i8,   to_i8,
        i16,  to_i16,
        i32,  to_i32,
        i64,  to_i64,
        bool, to_bool,
    }

    fn to_u64(&self) -> Result<u64, Error>;

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
    impl_ext_fns! {
        &str, to_str,  as_str,
        u8,   to_u8,   as_u8,
        u16,  to_u16,  as_u16,
        u32,  to_u32,  as_u32,
        // u64, to_u64, as_u64,
        i8,   to_i8,   as_i8,
        i16,  to_i16,  as_i16,
        i32,  to_i32,  as_i32,
        i64,  to_i64,  as_i64,
        bool, to_bool, as_bool,
    }

    fn to_u64(&self) -> Result<u64, Error> {
        self.to_str()?.parse().map_err(|_| Error::wrong_type::<u64>(self))
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
    u8,   to_u8, 
    u16,  to_u16, 
    u32,  to_u32, 
    // u64, to_u64,
    i8,   to_i8, 
    i16,  to_i16, 
    i32,  to_i32,
    i64,  to_i64, 
    bool, to_bool
);

// Special impls for u64 which cannot fit into json values

impl ToJson for u64 {
    fn to_json(&self) -> Value {
        self.to_string().into()
    }
}

impl FromJson for u64 {
    type Error = Error;

    fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        value.to_u64()
    }
}

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

macro_rules! impl_json_array {
    ($type:ty) => {
        impl<T: ToJson> ToJson for $type {
            fn to_json(&self) -> Value {
                Value::Array(self.iter().map(ToJson::to_json).collect())
            }
        }

        impl<T: FromJson> FromJson for $type
        where
            T::Error: From<Error>,
        {
            type Error = T::Error;

            fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
            where
                Self: Sized,
            {
                if let Value::Array(s) = value {
                    Ok(s.into_iter()
                        .map(FromJson::from_json)
                        .collect::<Result<_, T::Error>>()?)
                } else {
                    Err(Error::wrong_type::<T>(value).into())
                }
            }
        }
    };
}
impl_json_array!(alloc::vec::Vec<T>);
impl_json_array!(alloc::boxed::Box<[T]>);

impl<T: ToJson> ToJson for [T] {
    fn to_json(&self) -> Value {
        Value::Array(self.iter().map(ToJson::to_json).collect())
    }
}

impl<T: ToJson, const N: usize> ToJson for [T; N] {
    fn to_json(&self) -> Value {
        Value::Array(self.iter().map(ToJson::to_json).collect())
    }
}

impl<T: FromJson, const N: usize> FromJson for [T; N]
where
    T::Error: From<Error>,
{
    type Error = T::Error;

    fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if let Value::Array(s) = value {
            Ok(s.into_iter()
                .map(FromJson::from_json)
                .collect::<Result<Vec<T>, _>>()?
                .try_into()
                .map_err(|e: Vec<T>| Error::WrongArraySize {
                    expected: N,
                    found: e.len(),
                })?)
        } else {
            Err(Error::wrong_type::<T>(value).into())
        }
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
