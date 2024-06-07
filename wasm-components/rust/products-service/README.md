# Products service

List and create products for the product catalog

## List products

Send a GET request to `/products` endpoint

## Create products

Send a POST request to `/products` endpoint.

Sample curl request:

```shell
curl -X POST localhost:8080/products -d '{"sku": "WF-PP-STR", "id": "159cd2a4-9635-43e4-939d-396bb2df078e", "name": "Washer fluid", "description": "Scented washer fluid", "price": 1000}'
```
