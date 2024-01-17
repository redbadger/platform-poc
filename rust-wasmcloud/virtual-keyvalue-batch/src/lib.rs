struct KeyValueAdapter;

wit_bindgen::generate!({
    world: "virtual-keyvalue-batch",
    exports: {
        "wasi:keyvalue/readwrite": KeyValueAdapter,
        "wasi:keyvalue/batch": KeyValueAdapter,
    }
});

use exports::wasi::keyvalue::{batch::Guest as BatchExport, readwrite::Guest as ReadWriteExport};
use wasi::keyvalue::types::{Bucket, Error, IncomingValue, Key, Keys, OutgoingValue};

impl BatchExport for KeyValueAdapter {
    fn get_many(
        bucket: &Bucket,
        keys: Keys,
    ) -> Result<wit_bindgen::rt::vec::Vec<IncomingValue>, Error> {
        todo!()
    }

    fn get_keys(bucket: &Bucket) -> Keys {
        todo!()
    }

    fn set_many(
        bucket: &Bucket,
        key_values: wit_bindgen::rt::vec::Vec<(Key, &OutgoingValue)>,
    ) -> Result<(), Error> {
        todo!()
    }

    fn delete_many(bucket: &Bucket, keys: Keys) -> Result<(), Error> {
        todo!()
    }
}

impl ReadWriteExport for KeyValueAdapter {
    fn get(bucket: &Bucket, key: Key) -> Result<IncomingValue, Error> {
        todo!()
    }

    fn set(bucket: &Bucket, key: Key, outgoing_value: &OutgoingValue) -> Result<(), Error> {
        todo!()
    }

    fn delete(bucket: &Bucket, key: Key) -> Result<(), Error> {
        todo!()
    }

    #[doc = " Check if the key exists in the bucket."]
    fn exists(bucket: &Bucket, key: Key) -> Result<bool, Error> {
        todo!()
    }
}
