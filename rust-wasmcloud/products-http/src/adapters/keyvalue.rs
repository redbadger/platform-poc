use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use wasi::keyvalue::{
    batch, readwrite,
    types::{new_outgoing_value, open_bucket, outgoing_value_write_body_sync},
    wasi_cloud_error,
};

use crate::wasi::{self, keyvalue::types::incoming_value_consume_sync};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Serialization error")]
    Serde(#[from] serde_json::Error),
    #[error("HTTP error")]
    KeyValue(wasi::keyvalue::wasi_cloud_error::Error),
}

/// Unfortunately, wasmCloud doesn't yet support wasi::keyvalue::batch
/// so this won't work yet. I _might_ submit a PR to fix this.
pub fn get_all<T>(bucket: &str) -> Result<Vec<T>, Error>
where
    T: DeserializeOwned,
{
    // TODO improve error handling
    // fails with 3u32 if bucket doesn't exist
    let bucket = open_bucket(bucket)?;
    // no idea why this doesn't return a Result
    let keys = batch::get_keys(bucket);
    let values = batch::get_many(bucket, &keys)?;

    values
        .iter()
        .map(|value| {
            let bytes = incoming_value_consume_sync(*value)?;
            let t = serde_json::from_slice(bytes.as_slice()).unwrap();
            Ok(t)
        })
        .collect()
}

pub fn set<T>(bucket: &str, key: &str, value: &T) -> Result<(), Error>
where
    T: Serialize,
{
    let outgoing_value = new_outgoing_value();
    let bytes = serde_json::to_vec(value)?;
    outgoing_value_write_body_sync(outgoing_value, &bytes)?;

    // TODO improve error handling
    // fails with 3u32 if bucket doesn't exist
    let bucket = open_bucket(bucket)?;
    readwrite::set(bucket, &key.to_string(), outgoing_value)?;

    Ok(())
}

impl From<wasi_cloud_error::Error> for Error {
    fn from(value: wasi_cloud_error::Error) -> Self {
        Error::KeyValue(value)
    }
}
