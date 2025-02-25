#!/usr/bin/env fish

set -l endpoint http://localhost:8081

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "get products"
curl -vv "$endpoint/api/product"

section "create product"
curl -vv "$endpoint/api/product" \
    -H 'Content-Type: application/json' \
    -d '{
        "name": "iPhone 14",
        "description": "New iPhone 14",
        "price": 1100,
        "skuCode": "iphone_14"
    }'

section "get inventory for iphone_13"
curl -vv "$endpoint/api/inventory?skuCode=iphone_13"

section "get inventory for iphone_13_red"
curl -vv "$endpoint/api/inventory?skuCode=iphone_13_red"

section "place an order"
curl -vv "$endpoint/api/order" \
    -H 'content-type: application/json' \
    -d '{
      "orderLineItemsDtoList": [
          {
              "id": "123",
              "skuCode": "iphone_13",
              "price": 200,
              "quantity": 1
          }
      ]
  }'
