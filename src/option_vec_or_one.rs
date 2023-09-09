//! Serialize `Option<T>` or `Option<Vec<T>>`
//!
//! Many specs will allow either a single instance of an element, or a
//! collection of elements.
//!
//! ## USE DEFAULT!!
//! Note: The attribute must be decorated with default, or it will not be properly serialized. You will get a missing attribute error from Serde.
//!
//! # Examples
//!
//! ```rust, ignore
//! use serde::{Deserialize, Serialize};
//! #[derive(Debug, Deserialize, Serialize)]
//! pub struct Inner {
//!     pub item: String
//! }
//!
//! #[derive(Debug, Deserialize, Serialize)]
//! pub struct Outer {
//!     #[serde(default, with = "serde_stuff::option_vec_or_one")]
//!     pub items: Option<Vec<Inner>>
//! }
//! ```
//! The above will accept either of the following formats:
//! ```json
//! {
//!     "items": { "item": "value"}
//! }
//! ```
//!
//! ```json
//! {
//!     "items": [{"item": "value"}, {"Item": "value"}]
//! }
//! ```
//!
//! ```json
//! {
//! }
//! ```
use serde::{self, de, Deserialize, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;

pub fn deserialize<'de, D: de::Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<Vec<T>>, D::Error>
where
    T: de::Deserialize<'de>,
    D: de::Deserializer<'de>,
{
    struct OptionVec<T>(PhantomData<Option<T>>);

    impl<'de, T> de::Visitor<'de> for OptionVec<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Option<Vec<T>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a null, an array or map")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        /// If the value is present,
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            super::vec_or_one::deserialize(deserializer).map(Some)
        }
    }

    deserializer.deserialize_option(OptionVec(PhantomData))
}

/// Serializes either T or Vec<T> if Some<Vec<T>>.  Else serializes nothing.
/// This works!
pub fn serialize<S: Serializer, T: Serialize>(
    ov: &Option<Vec<T>>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match ov {
        Some(v) => match v.len() {
            1 => T::serialize(v.first().unwrap(), s),
            _ => Vec::<T>::serialize(v, s),
        },
        None => s.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    pub struct Inner {
        pub item: String,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    pub struct Outer {
        #[serde(
            default,
            with = "crate::option_vec_or_one",
            skip_serializing_if = "Option::is_none"
        )]
        pub items: Option<Vec<Inner>>,
    }

    #[test]
    fn deserialize_one() {
        let test1 = r#"
        {
            "items": {
                "item": "value"
            }
        }"#;

        let items = vec![Inner {
            item: "value".to_string(),
        }];
        let outer = Outer { items: Some(items) };

        let result: Outer = serde_json::from_str(test1).expect("Oops!");
        assert_eq!(&outer, &result);
    }

    #[test]
    fn deseerialize_multple() {
        let json = r#"
        {
            "items": [
                {
                "item": "value"
                },
                {
                    "item": "value"
                }
            ]
        }"#;

        let items = vec![
            Inner {
                item: "value".to_string(),
            },
            Inner {
                item: "value".to_string(),
            },
        ];
        let outer = Outer { items: Some(items) };

        let result: Outer = serde_json::from_str(json).expect("Oops!");
        assert_eq!(&outer, &result);
    }

    #[test]
    fn deserialize_none() {
        let json = r#"
        {
        }"#;

        let outer = Outer { items: None };

        let result: Outer = serde_json::from_str(json).expect("Oops!");
        assert_eq!(&outer, &result);
    }

    #[test]
    fn serialize_none() {
        let outer = Outer { items: None };
        let json = r#"{}"#;
        let result = serde_json::to_string(&outer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json);
    }

    #[test]
    fn serialize_one() {
        let outer = Outer {
            items: Some(vec![Inner {
                item: "value 1".to_string(),
            }]),
        };
        let json = r#"{"items":{"item":"value 1"}}"#;
        let result = serde_json::to_string(&outer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json);
    }

    #[test]
    fn serialize_some() {
        let outer = Outer {
            items: Some(vec![
                Inner {
                    item: "value 1".to_string(),
                },
                Inner {
                    item: "value 2".to_string(),
                },
                Inner {
                    item: "value 3".to_string(),
                },
            ]),
        };
        let json = r#"{"items":[{"item":"value 1"},{"item":"value 2"},{"item":"value 3"}]}"#;
        let result = serde_json::to_string(&outer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json);
    }
}
