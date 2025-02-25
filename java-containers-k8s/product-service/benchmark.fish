#!/usr/bin/env fish

set -l endpoint "http://localhost:8081"

oha "$endpoint/api/product" \
    -c 100 \
    -n 10000
