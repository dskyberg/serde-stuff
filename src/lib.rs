//! # serde-stuff
//! Why `serde-stuff`?  Because `serde-utils` was already taken!
//!
//! This crate contains some very common serde (de)serializing utilities.
//! See the the module tests in the repo for examples of out to use these.
//!
//! # About serde_with
//! The module tests all use [serde_with](https://docs.rs/serde_with) to reference a module:`#[serde(with = "serde_stuff::string_or_struct")]`.
//! This provides a shorthand, especially for modules that support both serialization and deserialization.
//!
//! But this crate does not depend on it.  If you don't want to use it, just use the
//! mod's `serialize` and `deserialize` functions.  Such as `#[serde(deserialize_with = "serde_stuff::string_or_struct::deserialize")]`.
pub mod base64;
pub mod option_base64;
pub mod option_string_or_struct;
pub mod string_or_struct;
pub mod vec_or_one;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
