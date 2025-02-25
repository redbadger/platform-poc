#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

set -x PORT 8092
set -x INVENTORY_URL http://localhost:8090/api/inventory
set -x DATABASE_URL postgresql://order-service:commerce@localhost/order-service
set -x NATS_URL nats://localhost:4222

cd $SCRIPT_DIR/..
cargo run --release --bin order-service
