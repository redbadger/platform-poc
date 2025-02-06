# Orders service

Query and create orders

## Get orders

Send a GET request to `/orders`

## Create orders

```shell
curl -X POST localhost:8080/orders -d '[{"sku": "WND-WPR-AW", "price": 1000, "quantity": 1}, {"sku": "TIR-SET-AS", "price": 20000, "quantity": 3}, {"sku": "ALT-HO-ENH", "price": 13000, "quantity": 1}]'
```

This one will not create the entry since the last item is out of stock

```shell
curl -X POST localhost:8080/orders -d '[{"sku": "WND-WPR-AW", "price": 1000, "quantity": 1}, {"sku": "TIR-SET-AS", "price": 20000, "quantity": 3}]'
```

This will create an order - see products-service for all available skus
