//! Deserialize `T` or `[T]` to `T`
//!
//! Many specs will allow either a single instance of an element, or a
//! collection of elements.
//!
//! # Examples
//!
//! ```rust
//! use serde::{Deserialize};
//! #[derive(Debug, Deserialize)]
//! pub struct Inner {
//!     pub item: String
//! }
//!
//! #[derive(Debug, Deserialize)]
//! pub struct Outer {
//!     #[serde(with = "serde_stuff::vec_or_one")]
//!     pub inners: Vec<Inner>
//! }
//! ```
//! The above will accept either of the following formats:
//! ```json
//! {
//!     "inners": { "item": "value"}
//! }
//! ```
//! ```json
//! {
//!     "inners": [{"item": "value"}, {"Item": "value"}]
//! }
//! ```

use serde::{self, de, Deserialize, Serialize, Serializer};

#[derive(Deserialize, Debug)]
#[serde(untagged)] // This is the magic. see https://serde.rs/enum-representations.html
pub enum VecOrOne<T> {
    Vec(Vec<T>),
    One(T),
}

pub fn deserialize<'de, D: de::Deserializer<'de>, T: Deserialize<'de>>(
    de: D,
) -> Result<Vec<T>, D::Error> {
    use de::Deserialize as _;
    match VecOrOne::deserialize(de)? {
        VecOrOne::Vec(v) => Ok(v),
        VecOrOne::One(i) => Ok(vec![i]),
    }
}

pub fn serialize<S: Serializer, T: Serialize>(v: &Vec<T>, s: S) -> Result<S::Ok, S::Error> {
    match v.len() {
        1 => T::serialize(v.first().unwrap(), s),
        _ => Vec::<T>::serialize(v, s),
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Inner {
        pub item: String,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct Outer {
        #[serde(with = "crate::vec_or_one")]
        pub inners: Vec<Inner>,
    }

    #[test]
    fn deserialize_single_test() {
        let test1 = r#"
        {
            "inners": {
                "item": "value"
            }
        }"#;

        let inners = vec![Inner {
            item: "value".to_string(),
        }];
        let outer = Outer { inners };

        let result: Outer = serde_json::from_str(test1).expect("Oops!");
        assert_eq!(&outer, &result);
    }

    #[test]
    fn deserialize_multple_test() {
        let test1 = r#"
        {
            "inners": [
                {
                "item": "value"
                },
                {
                    "item": "value"
                }
            ]
        }"#;

        let inners = vec![
            Inner {
                item: "value".to_string(),
            },
            Inner {
                item: "value".to_string(),
            },
        ];
        let outer = Outer { inners };

        let result: Outer = serde_json::from_str(test1).expect("Oops!");
        assert_eq!(&outer, &result);
    }

    #[test]
    fn serialize_one_test() {
        let json = r##"{"inners":{"item":"value 1"}}"##;
        let outer = Outer {
            inners: vec![Inner {
                item: "value 1".to_string(),
            }],
        };
        let result = serde_json::from_str::<Outer>(json);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), outer);
    }

    #[test]
    fn serialize_multiple_test() {
        let json = r##"{"inners":[{"item":"value 1"},{"item":"value 2"},{"item":"value 3"}]}"##;
        let outer = Outer {
            inners: vec![
                Inner {
                    item: "value 1".to_string(),
                },
                Inner {
                    item: "value 2".to_string(),
                },
                Inner {
                    item: "value 3".to_string(),
                },
            ],
        };
        let result = serde_json::to_string(&outer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json);
    }
}
