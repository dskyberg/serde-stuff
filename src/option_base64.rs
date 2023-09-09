//! Serialize and Deserialize a `Option<Vec<u8>>` to a [base64] string.
//!
//! ## USE DEFAULT!!
//! **Note:** The attribute must be decorated with `default`, or it will not
//! be properly serialized.  You will get a missing attribute error from Serde.
//!
//! #Examples
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serde_json;
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! pub struct Outer {
//!     #[serde(default, with = "serde_stuff::option_base64")]
//!     pub item: Option<Vec<u8>>,
//! }
//! ```

use base64::{engine::general_purpose, Engine as _};

use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};

pub fn serialize<S: Serializer>(v: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error> {
    let base64 = v
        .as_ref()
        //        .map(|v| base64::encode_config(v, base64::URL_SAFE));
        .map(|v| general_purpose::URL_SAFE.encode(v));
    /*
       let base64 = match v {
            Some(v) => Some(base64::encode_config(v, base64::URL_SAFE)),
            None => None,
        };
    */
    <Option<String>>::serialize(&base64, s)
}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<u8>>, D::Error> {
    let base64 = <Option<String>>::deserialize(d)?;
    match base64 {
        //Some(v) => base64::decode_config(v.as_bytes(), base64::URL_SAFE)
        Some(v) => general_purpose::URL_SAFE
            .decode(v.as_bytes())
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Outer {
        #[serde(
            default,
            with = "crate::option_base64",
            skip_serializing_if = "Option::is_none"
        )]
        pub item: Option<Vec<u8>>,
        pub other: String,
    }

    const TEST_B64: &str = "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=";
    const TEST_VEC: [u8; 32] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    #[test]
    fn serialize_some() {
        let model = format!(r#"{{"item":"{}","other":"value"}}"#, TEST_B64);
        let outer = Outer {
            item: Some(TEST_VEC.to_vec()),
            other: "value".to_string(),
        };
        let result = serde_json::to_string(&outer).expect("Oops!");

        assert_eq!(&result, &model);
    }

    #[test]
    fn serialize_none() {
        let model = r#"{"other":"value"}"#;
        let outer = Outer {
            item: None,
            other: "value".to_string(),
        };
        let result = serde_json::to_string(&outer).expect("Oops!");

        assert_eq!(&result, &model);
    }

    #[test]
    fn deserialize_some() {
        let model = format!(
            r#"{{
            "item": "{}",
            "other": "value"
        }}"#,
            TEST_B64
        );

        let outer = Outer {
            item: Some(TEST_VEC.to_vec()),
            other: "value".to_string(),
        };

        let result: Outer = serde_json::from_str(&model).expect("Oops!");
        assert_eq!(&outer, &result);
    }

    #[test]
    fn deserialize_none() {
        let model = r#"{
                "other": "value"
            }"#;

        let outer = Outer {
            item: None,
            other: "value".to_string(),
        };

        let result: Outer = serde_json::from_str(model).expect("Oops!");
        assert_eq!(&outer, &result);
    }
}
