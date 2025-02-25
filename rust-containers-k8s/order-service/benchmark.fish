#!/usr/bin/env fish

set -l endpoint "http://localhost:8081"
# place orders
oha "$endpoint/api/order" \
    -c 100 \
    -n 10000 \
    -m POST \
    -H 'content-type: application/json' \
    -d '{"items":[{"sku":"iphone_13","price":1,"quantity":1}]}'

# list orders
# oha "$endpoint/api/order" \
#     -c 100 \
#     -n 10000
