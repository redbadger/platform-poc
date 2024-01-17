#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use exports::wasi::keyvalue::{
    batch::Guest as BatchExport, readwrite::Guest as ReadWriteExport,
};
use wasi::keyvalue::types::{Bucket, Error, IncomingValue, Key, Keys, OutgoingValue};
struct KeyValueAdapter;
impl BatchExport for KeyValueAdapter {
    /// Get the values associated with the keys in the bucket. It returns a list of
    /// incoming-values that can be consumed to get the values.
    ///
    /// If any of the keys do not exist in the bucket, it returns an error.
    fn get_many(
        bucket: &Bucket,
        keys: Keys,
    ) -> Result<wit_bindgen::rt::vec::Vec<IncomingValue>, Error> {
        ::core::panicking::panic("not yet implemented")
    }
    /// Get all the keys in the bucket. It returns a list of keys.
    fn get_keys(bucket: &Bucket) -> Keys {
        ::core::panicking::panic("not yet implemented")
    }
    /// Set the values associated with the keys in the bucket. If the key already
    /// exists in the bucket, it overwrites the value.
    ///
    /// If any of the keys do not exist in the bucket, it creates a new key-value pair.
    /// If any other error occurs, it returns an error.
    fn set_many(
        bucket: &Bucket,
        key_values: wit_bindgen::rt::vec::Vec<(Key, &OutgoingValue)>,
    ) -> Result<(), Error> {
        ::core::panicking::panic("not yet implemented")
    }
    /// Delete the key-value pairs associated with the keys in the bucket.
    ///
    /// If any of the keys do not exist in the bucket, it skips the key.
    /// If any other error occurs, it returns an error.
    fn delete_many(bucket: &Bucket, keys: Keys) -> Result<(), Error> {
        ::core::panicking::panic("not yet implemented")
    }
}
impl ReadWriteExport for KeyValueAdapter {
    /// Get the value associated with the key in the bucket. It returns a incoming-value
    /// that can be consumed to get the value.
    ///
    /// If the key does not exist in the bucket, it returns an error.
    fn get(bucket: &Bucket, key: Key) -> Result<IncomingValue, Error> {
        ::core::panicking::panic("not yet implemented")
    }
    /// Set the value associated with the key in the bucket. If the key already
    /// exists in the bucket, it overwrites the value.
    ///
    /// If the key does not exist in the bucket, it creates a new key-value pair.
    /// If any other error occurs, it returns an error.
    fn set(
        bucket: &Bucket,
        key: Key,
        outgoing_value: &OutgoingValue,
    ) -> Result<(), Error> {
        ::core::panicking::panic("not yet implemented")
    }
    /// Delete the key-value pair associated with the key in the bucket.
    ///
    /// If the key does not exist in the bucket, it returns an error.
    fn delete(bucket: &Bucket, key: Key) -> Result<(), Error> {
        ::core::panicking::panic("not yet implemented")
    }
    /// Check if the key exists in the bucket.
    fn exists(bucket: &Bucket, key: Key) -> Result<bool, Error> {
        ::core::panicking::panic("not yet implemented")
    }
}
