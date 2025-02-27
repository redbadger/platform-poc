## wasmCloud

_Wasm components linked at runtime (via wRPC) and running in wasmCloud._

![deployed to wasmCloud](./platform-poc.webp)

### Deploy locally

## Setup

### wasmCloud

Install `wash`, e.g. with `brew install wasmcloud/wasmcloud/wash`.

```bash
wash --version
# wash          v0.39.0
# ├ nats-server v2.10.20
# ├ wadm        v0.20.2
# └ wasmcloud   v1.6.1
```

### Docker

We use `docker` to start a k3d cluster and a local registry.

## run

1. Create a platform environment.

   Starts:
   * a local OCI registry
   * a single wasmCloud host, with NATS and `wadm` running
   * the wash UI (http://localhost:3030/)

   ```bash
   ./local/create.fish
   ```

1. Start up external services.

   Starts:
   * a local redis server
   * a local postgres server

   ```bash
   ./local/up.fish
   ```

1. Build (and sign) the components, pushing them to the local registry

   ```bash
   ./build_and_push.fish
   ```

1. Deploy the components with `wadm`

   ```bash
   ./deploy.fish
   ```

1. Redeploy the components with `wadm`

   ```bash
   ./redeploy.fish
   ```

1. Delete the application with `wadm`

   ```bash
   ./undeploy.fish
   ```

1. Stop the external services

   ```bash
   ./local/down.fish
   ```

1. Destroy the registry and the wasmCloud host

   ```bash
   ./local/destroy.fish
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

### benchmark

```fish
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
```
