// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{
    set::{OrderedSetError, SetError},
    Error, FromJson, JsonExt, ToJson, Value,
};

#[cfg(feature = "std")]
mod std_hash {
    use super::*;

    impl<K: ToString, V: ToJson> ToJson for std::collections::HashMap<K, V> {
        fn to_json(&self) -> Value {
            let mut obj = json::object::Object::new();
            for (k, v) in self {
                obj.insert(&k.to_string(), v.to_json());
            }
            Value::Object(obj)
        }
    }

    impl<K: core::str::FromStr + Eq + core::hash::Hash, V: FromJson> FromJson for std::collections::HashMap<K, V>
    where
        V::Error: core::fmt::Display,
    {
        type Error = Error;

        fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if let Value::Object(mut obj) = value {
                let mut map = Self::default();
                for (k, v) in obj.iter_mut() {
                    map.insert(
                        K::from_str(k).map_err(|_| Error::invalid_key::<K>(k))?,
                        v.take_value::<V>().map_err(|e| SetError::Item(e.to_string()))?,
                    );
                }
                Ok(map)
            } else {
                Err(Error::wrong_type::<Self>(value).into())
            }
        }
    }
}

impl<K: ToString, V: ToJson> ToJson for alloc::collections::BTreeMap<K, V> {
    fn to_json(&self) -> Value {
        let mut obj = json::object::Object::new();
        for (k, v) in self {
            obj.insert(&k.to_string(), v.to_json());
        }
        Value::Object(obj)
    }
}

impl<K: core::str::FromStr + Eq + Ord + core::hash::Hash, V: FromJson> FromJson for alloc::collections::BTreeMap<K, V>
where
    V::Error: core::fmt::Display,
{
    type Error = Error;

    fn from_non_null_json(value: Value) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if let Value::Object(mut obj) = value {
            let mut map = Self::default();
            for (k_value, v) in obj.iter_mut() {
                let k = K::from_str(k_value).map_err(|_| Error::invalid_key::<K>(k_value))?;
                let v = v.take_value::<V>().map_err(|e| SetError::Item(e.to_string()))?;
                if let Some((last, _)) = map.last_key_value() {
                    if &k < last {
                        return Err(Error::OrderedSet(OrderedSetError::Unordered(k_value.to_string())));
                    }
                }
                map.insert(k, v);
            }
            Ok(map)
        } else {
            Err(Error::wrong_type::<Self>(value).into())
        }
    }
}
