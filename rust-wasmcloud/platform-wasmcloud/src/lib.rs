wit_bindgen::generate!({
    world: "platform-wasmcloud",
    exports: {
        "platform-poc:keyvalue/keyvalue/bucket": BucketResource,
    }
});

use wit_bindgen::Resource;

use exports::platform_poc::keyvalue::keyvalue as platform_kv;
use wasi::keyvalue::{readwrite, types as wasi_kv, wasi_cloud_error::trace};

const KEYS_KEY: &str = "__keys";

pub struct BucketResource {
    bucket_name: String, // virtualised by prefixing all keys
    wasi_handle: u32,
}

impl BucketResource {
    fn format_key(&self, key: &str) -> platform_kv::Key {
        format!("{}:{}", self.bucket_name, key)
    }
}

impl platform_kv::GuestBucket for BucketResource {
    fn open(name: String) -> Result<Resource<BucketResource>, platform_kv::Error> {
        let bucket = wasi_kv::open_bucket("")?; // Buckets are not yet supported in wasmcloud

        Ok(Resource::new(BucketResource {
            bucket_name: name,
            wasi_handle: bucket,
        }))
    }

    fn name(&self) -> String {
        self.bucket_name.clone()
    }

    fn get_all(&self) -> Result<Vec<platform_kv::KeyValue>, platform_kv::Error> {
        let all_keys: Vec<platform_kv::Key> =
            readwrite::get(self.wasi_handle, &self.format_key(KEYS_KEY))
                .and_then(wasi_kv::incoming_value_consume_sync)
                .map(|bytes| serde_json::from_slice(&bytes).unwrap_or_else(|_e| vec![]))
                .unwrap_or_else(|_e| vec![]);

        let mut all_values = Vec::new();
        for key in all_keys {
            let bytes = readwrite::get(self.wasi_handle, &self.format_key(&key))
                .and_then(wasi_kv::incoming_value_consume_sync)?;

            all_values.push((key.clone(), bytes));
        }

        Ok(all_values)
    }

    fn set(
        &self,
        key: platform_kv::Key,
        value: platform_kv::Value,
    ) -> Result<(), platform_kv::Error> {
        let mut all_keys = readwrite::get(self.wasi_handle, &self.format_key(KEYS_KEY))
            .and_then(wasi_kv::incoming_value_consume_sync)
            .map(|bytes| serde_json::from_slice(&bytes).unwrap_or_else(|_e| vec![]))
            .unwrap_or_else(|_e| vec![]);

        if !all_keys.contains(&key) {
            all_keys.push(key.clone());
        }

        let outgoing_value = wasi_kv::new_outgoing_value();
        wasi_kv::outgoing_value_write_body_sync(outgoing_value, &value)?;
        readwrite::set(self.wasi_handle, &self.format_key(&key), outgoing_value)?;

        let outgoing_value = wasi_kv::new_outgoing_value();
        let bytes = serde_json::to_vec(&all_keys)?;
        wasi_kv::outgoing_value_write_body_sync(outgoing_value, &bytes)?;
        readwrite::set(self.wasi_handle, &self.format_key(KEYS_KEY), outgoing_value)?;

        Ok(())
    }
}

impl From<wasi_kv::Error> for platform_kv::Error {
    fn from(value: wasi_kv::Error) -> Self {
        Self::Internal(format!(
            "WASI keyvalue error: {}, trace: {}",
            value,
            trace(value)
        ))
    }
}

impl From<serde_json::error::Error> for platform_kv::Error {
    fn from(value: serde_json::error::Error) -> Self {
        Self::Internal(format!("Error parsing JSON: {}", value))
    }
}
