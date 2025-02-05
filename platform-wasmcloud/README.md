## wasmCloud

_Wasm components linked at runtime (via wRPC) and running in wasmCloud._

![deployed to wasmCloud](./platform-poc.webp)

### Deploy locally

## Setup

### wasmCloud

Install `wash` with `brew install wasmcloud/wasmcloud/wash`.

```bash
wash --version
# wash          v0.38.0
# ├ nats-server v2.10.20
# ├ wadm        v0.19.0
# └ wasmcloud   v1.4.2
```

## run

1. Build and sign the components

   ```bash
   ./build_and_sign.fish
   ```

1. Start the wasmCloud host

   ```bash
   ./local/up.fish
   ```

1. Deploy the components with `wadm`

   ```bash
   ./start.fish
   ```

1. Redeploy the components with `wadm`

   ```bash
   ./restart.fish
   ```

1. Delete the application with `wadm`

   ```bash
   ./stop.fish
   ```

1. Stop the wasmCloud host

   ```bash
   ./local/down.fish
   ```

1. Test

   ```bash
   # data init
   curl 'localhost:8080/data-init/all'

   # products
   curl 'localhost:8080/products'

   # inventory
   curl 'localhost:8080/inventory/?skus=ENG-V8-500'

   # orders
   curl 'localhost:8080/orders'

   # create order (fish shell)
   curl localhost:8080/orders -d '
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
   # 201 Created
   ```
