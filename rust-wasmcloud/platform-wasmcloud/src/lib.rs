wit_bindgen::generate!({
    world: "platform-wasmcloud",
    exports: {
        "platform-poc:keyvalue/keyvalue": KeyValueAdapter,
    }
});

use exports::platform_poc::keyvalue::keyvalue::Guest as KeyValueExport;
use exports::platform_poc::keyvalue::keyvalue::{Bucket, Error, Key, KeyValue, Value};
use wasi::keyvalue;

struct KeyValueAdapter;

const SEPARATOR: &str = " ";

impl KeyValueExport for KeyValueAdapter {
    fn get_all(bucket: Bucket) -> Result<Vec<KeyValue>, Error> {
        todo!()
    }

    fn set(bucket: Bucket, key: Key, value: Value) -> Result<(), Error> {
        todo!()
    }
}
