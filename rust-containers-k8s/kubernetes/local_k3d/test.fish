#!/usr/bin/env fish

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "get products"
curl -vv 'localhost:8081/api/product'

section "get inventory for iphone_13"
curl -vv 'localhost:8081/api/inventory?skuCode=iphone_13'

section "get inventory for iphone_13_red"
curl -vv 'localhost:8081/api/inventory?skuCode=iphone_13_red'

section "place an order"
curl -vv 'localhost:8081/api/order' \
    -H 'content-type: application/json' \
    -d '{"items":[{"sku":"iphone_13","price":1,"quantity":1}]}'
