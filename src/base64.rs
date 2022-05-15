//! Serialize and Deserialize a `Vec<u8>` to a [base64] string.
//!
//! #Examples
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serde_json;
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! pub struct Outer {
//!     #[serde(with = "serde_stuff::base64")]
//!     pub item: Vec<u8>,
//! }
//! ```

use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};

pub fn serialize<S: Serializer>(v: &[u8], s: S) -> Result<S::Ok, S::Error> {
    let base64 = base64::encode_config(v, base64::URL_SAFE);
    String::serialize(&base64, s)
}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
    let base64 = String::deserialize(d)?;
    base64::decode_config(base64.as_bytes(), base64::URL_SAFE).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Outer {
        #[serde(with = "crate::base64")]
        pub item: Vec<u8>,
    }
    const TEST_B64: &str = "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=";
    const TEST_VEC: [u8; 32] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    #[test]
    fn serialize() {
        let model = format!(r#"{{"item":"{}"}}"#, TEST_B64);
        let outer = Outer {
            item: TEST_VEC.to_vec(),
        };
        let result = serde_json::to_string(&outer).expect("Oops!");

        assert_eq!(&result, &model);
    }

    #[test]
    fn deserialize() {
        let model = format!(
            r#"{{
            "item": "{}"
        }}"#,
            TEST_B64
        );

        let outer = Outer {
            item: TEST_VEC.to_vec(),
        };

        let result: Outer = serde_json::from_str(&model).expect("Oops!");
        assert_eq!(&outer, &result);
    }
}
