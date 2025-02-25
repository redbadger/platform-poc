#!/usr/bin/env fish

oha 'http://localhost:8080/orders' \
    -c 100 \
    -n 10000 \
    -m POST \
    -H 'Content-Type: application/json' \
    -d '
    [
      {
        "sku": "WND-WPR-AW",
        "price": 1000,
        "quantity": 1
      },
      {
        "sku": "TIR-SET-AS",
        "price": 20000,
        "quantity": 3
      }
    ]'
