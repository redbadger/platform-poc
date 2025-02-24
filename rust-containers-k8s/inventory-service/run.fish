#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

set -x PORT 8090
set -x DATABASE_URL postgresql://inventory-service:commerce@localhost/inventory-service

cd $SCRIPT_DIR/..
cargo run --release --bin inventory-service
