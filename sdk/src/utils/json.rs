// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use json::JsonValue as Value;

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
    ($type:ty, $fn:ident) => {
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
                Ok(value.$fn().ok_or_else(|| Error::wrong_type::<$type>(value))?)
            }
        }
    };
}
impl_json_via!(u8, as_u8);
impl_json_via!(u16, as_u16);
impl_json_via!(u32, as_u32);
impl_json_via!(u64, as_u64);
impl_json_via!(bool, as_bool);

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
