# HTTP Wrapper for Products

This component adapts the [product interface](../wit/products/products.wit) to `wasi:http`.

## Post a product

```
curl --header "Content-Type: application/json" \
  --request POST \
  --data "{\"name\":\"Teddy bear\",\"description\":\"He's very fluffy\!\",\"price\":1200,\"skuCode\":\"teddy\"}" \
  http://localhost:8081/api/product
```
