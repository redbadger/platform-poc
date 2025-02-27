# Platform PoC Wasm components in wasmCloud on a local K3d cluster

1. Create the cluster and local registry

   ```fish
   ./create.fish
   ```

1. Install the wasmCloud operator

   ```fish
    ./install-operator.fish
   ```

1. Start redis, postgres, wash ui and port forwarding

   ```fish
   ./up.fish
   ```

1. Build the platform-poc application

   ```fish
    ../build_and_push.fish
   ```

1. Install our platform-poc application

   ```fish
   kubectl apply -f ./wadm.yaml
   ```

1. Delete the application

   ```fish
   kubectl delete -f ./wadm.yaml
   ```

1. Stop the cluster

   ```fish
    ./down.fish
   ```

1. Delete the cluster and registry

   ```fish
     ./destroy.fish
   ```
