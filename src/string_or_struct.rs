//! Deserialize string or map to `T`
//!
//! This function enables parsing a string into any struct  
//! that implements FromStr.  The semantics of the parsing
//! from string to struct is managed by the struct itself.  
//!
//! # Examples
//!
//! ```rust
//! use serde::Deserialize;
//! use std::str::FromStr;
//! use void::Void;
//!
//! #[derive(Debug, Deserialize)]
//! pub struct Inner {
//!     pub item: String,
//! }
//!
//! impl FromStr for Inner {
//!     type Err = Void;
//!
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         Ok(Inner {
//!             item: s.to_string(),
//!         })
//!     }
//! }
//!
//! #[derive(Debug, Deserialize)]
//! pub struct Outer {
//!     #[serde(with = "serde_stuff::string_or_struct")]
//!     pub inner: Inner,
//! }
//! ```
//! The following will both deserialize to `Outer`
//! ```json
//! {
//!     "inner": "value"
//! }
//! ```
//! ```json
//! {
//!     "inner": { "item": "value"}
//! }
//! ```

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use void::Void;

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        // If the value is a string, use the objects FromStr impl
        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        // If the value is a map, pass it to Serde's Map deserializer
        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }

        // If the value is neither a string or a map, present an appropriate error
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json;
    use std::str::FromStr;
    use void::Void;

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Inner {
        pub item: String,
    }

    impl FromStr for Inner {
        type Err = Void;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Inner {
                item: s.to_string(),
            })
        }
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Outer {
        // depends on serde_with
        #[serde(with = "crate::string_or_struct")]
        pub inner: Inner,
    }

    #[test]
    fn string_test() {
        let test = r#"
        {
            "inner": "value"
        }"#;

        let inner = Inner {
            item: "value".to_string(),
        };
        let outer = Outer { inner };

        let result: Outer = serde_json::from_str(test).expect("Oops!");
        assert_eq!(&outer, &result);
    }

    #[test]
    fn map_test() {
        let test = r#"
        {
            "inner": { 
                "item": "value"
            }
        }"#;

        let inner = Inner {
            item: "value".to_string(),
        };
        let outer = Outer { inner };

        let result: Outer = serde_json::from_str(test).expect("Oops!");
        assert_eq!(&outer, &result);
    }
}
