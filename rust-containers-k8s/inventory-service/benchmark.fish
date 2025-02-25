#!/usr/bin/env fish

set -l endpoint "http://localhost:8081"

oha "$endpoint/api/inventory?skuCode=iphone_13" \
    -c 100 \
    -n 10000
