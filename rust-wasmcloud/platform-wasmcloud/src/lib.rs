wit_bindgen::generate!({
    world: "platform-wasmcloud",
    exports: {
        "platform-poc:keyvalue/keyvalue": KeyValueAdapter,
    }
});

use exports::platform_poc::keyvalue::keyvalue::Guest as KeyValueExport;
use exports::platform_poc::keyvalue::keyvalue::{Bucket, Error, Key, KeyValue, Value};
use wasi::keyvalue::readwrite;
use wasi::keyvalue::types as wasi_types;

struct KeyValueAdapter;

const SEPARATOR: &str = " ";

impl KeyValueExport for KeyValueAdapter {
    fn get_all(bucket: Bucket) -> Result<Vec<KeyValue>, Error> {
        todo!()
    }

    fn set(bucket: Bucket, key: Key, value: Value) -> Result<(), Error> {
        let outgoing_value = wasi_types::new_outgoing_value();
        wasi_types::outgoing_value_write_body_sync(outgoing_value, &value)?;

        let bucket = wasi_types::open_bucket("TEST")?; // FIXME

        readwrite::set(bucket, &key, outgoing_value)?;

        Ok(())
    }
}

impl From<wasi_types::Error> for Error {
    fn from(value: wasi_types::Error) -> Self {
        Error::Internal(value.to_string())
    }
}
