# Virtual wasi:keyvalue/batch

Provides `wasi:keyvalue/batch` functionality on top of `wasi:keyvalue/readwrite`. With obvious limitations, especially the lack of atomicity. This is a POC after all.

## Why?

WasmCloud nativey supports `wasi:keyvalue/readwrite`, but the [products service](../products/) needs to be able to list all keys, which is a batch operation.

## How?

This simply keeps a list of all keys in a separate well known key and whenever a key is written, it also updates the well known key. It can then return the value of this key whenever `wasi:keyvaly/batch/get-keys` is called.
