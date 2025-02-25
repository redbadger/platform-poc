#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

set -x PORT 8093
set -x REDIS_URL redis://localhost:6379

cd $SCRIPT_DIR/..
cargo run --release --bin product-service
