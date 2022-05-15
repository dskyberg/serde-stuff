//! Deserialize string or map to `<Option<T>>` if present
//!
//!Support optional 'short' and 'long' versions of objects
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
//!     #[serde(with = "serde_stuff::option_string_or_struct")]
//!     pub inner: Option<Inner>,
//!     pub other: String,
//! }
//! ```
//! The following will deserialize `Some(Inner)` from a string
//! ```json
//! {
//!     "inner": "value",
//!     "other": "other_value"
//! }
//! ```
//! The following will deserialize `Some(Inner)` from a map
//! ```json
//! {
//!     "inner": { "item": "value"}
//!     "other": "other_value"
//! }
//! ```
//! The following will deserializes to None
//! ```json
//! {
//!     "other": "other_value"
//! }
//! ```

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use void::Void;

use super::string_or_struct;

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct OptStringOrStruct<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for OptStringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nul, a string or map")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            string_or_struct::deserialize(deserializer).map(Some)
        }
    }

    deserializer.deserialize_option(OptStringOrStruct(PhantomData))
}
