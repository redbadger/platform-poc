#!/usr/bin/env fish

set -l ADDRESS localhost:8081
# set -l ADDRESS localhost:8092

oha "http://$ADDRESS/api/inventory?skuCode=iphone_13" \
    -c 100 \
    -n 10000
