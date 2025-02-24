#!/usr/bin/env fish

# set -l ADDRESS localhost:8081
set -l ADDRESS localhost:8092

oha "http://$ADDRESS/api/order" \
    -c 100 \
    -n 10000 \
    -m POST \
    -H 'content-type: application/json' \
    -d '{"items":[{"sku":"iphone_13","price":1,"quantity":1}]}'
