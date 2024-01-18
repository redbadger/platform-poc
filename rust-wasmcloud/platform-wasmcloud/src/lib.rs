wit_bindgen::generate!({
    world: "platform-wasmcloud",
    exports: {
        "platform-poc:keyvalue/keyvalue": KeyValueAdapter,
    }
});

use exports::platform_poc::keyvalue::keyvalue::Guest as KeyValueExport;
use exports::platform_poc::keyvalue::keyvalue::{Bucket, Error, Key, KeyValue, Value};
use serde::{Deserialize, Serialize};
use wasi::keyvalue::{readwrite, types as wasi_kv};

const ALL_KEYS: &str = "__keys";

struct KeyValueAdapter;

impl KeyValueExport for KeyValueAdapter {
    fn get_all(bucket: Bucket) -> Result<Vec<KeyValue>, Error> {
        let all_keys = readwrite::get(bucket.wasi_kv_handle(), &ALL_KEYS.to_string()).unwrap();
        let bytes = wasi_kv::incoming_value_consume_sync(all_keys).unwrap();
        let all_keys: KeyList = match serde_json::from_slice(bytes.as_slice()) {
            Ok(keys) => keys,
            Err(_) => KeyList(Vec::new()),
        };

        let mut all_values = Vec::new();
        for key in all_keys.0.iter() {
            let value = readwrite::get(bucket.wasi_kv_handle(), &key.to_string()).unwrap();
            let bytes = wasi_kv::incoming_value_consume_sync(value).unwrap();
            let value: Value = match serde_json::from_slice(bytes.as_slice()) {
                Ok(value) => value,
                Err(_) => continue,
            };
            all_values.push((key.clone(), value));
        }

        Ok(all_values)
    }

    fn set(bucket: Bucket, key: Key, value: Value) -> Result<(), Error> {
        let all_keys = readwrite::get(bucket.wasi_kv_handle(), &ALL_KEYS.to_string()).unwrap();
        let bytes = wasi_kv::incoming_value_consume_sync(all_keys).unwrap();
        let mut all_keys: KeyList = match serde_json::from_slice(bytes.as_slice()) {
            Ok(keys) => keys,
            Err(_) => KeyList(Vec::new()),
        };
        if !all_keys.0.contains(&key) {
            all_keys.0.push(key.clone());
        }

        let outgoing_value = wasi_kv::new_outgoing_value();
        let bytes = serde_json::to_vec(&value).unwrap();
        wasi_kv::outgoing_value_write_body_sync(outgoing_value, &bytes).unwrap();
        readwrite::set(bucket.wasi_kv_handle(), &key.to_string(), outgoing_value).unwrap();

        let outgoing_value = wasi_kv::new_outgoing_value();
        let bytes = serde_json::to_vec(&all_keys).unwrap();
        wasi_kv::outgoing_value_write_body_sync(outgoing_value, &bytes).unwrap();
        readwrite::set(
            bucket.wasi_kv_handle(),
            &ALL_KEYS.to_string(),
            outgoing_value,
        )
        .unwrap();

        Ok(())
    }

    fn open_bucket(name: String) -> Result<Bucket, Error> {
        // TODO improve error handling
        // fails with 3u32 if bucket doesn't exist
        let bucket = wasi_kv::open_bucket(&name).unwrap();
        Ok(Bucket::new(&name, bucket))
    }
}

#[derive(Serialize, Deserialize)]
struct KeyList(Vec<Key>);
