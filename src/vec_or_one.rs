//! Deserialize `T` or `[T]` to `T`
//!
//! Many specs will allow either a single instance of an element, or a
//! collection of elements.
//!
//! # Examples
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! #[derive(Debug, Serialize, Deserialize)]
//! pub struct Inner {
//!     pub item: String
//! }
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! pub struct Outer {
//!     #[serde(with = "crate::option_string_or_struct")]
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

use serde::{self, de, Deserialize};

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

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json;

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Inner {
        pub item: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Outer {
        #[serde(with = "crate::vec_or_one")]
        pub inners: Vec<Inner>,
    }

    #[test]
    fn single_test() {
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
    fn multple_test() {
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
}
