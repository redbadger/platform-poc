# Platform PoC Rust containers in a local k3d kubernetes cluster

1. Create the cluster with a registry

   ```fish
   ./create.fish
   ```

1. Add the registry to your local /etc/hosts

   ```fish
   # edit /etc/hosts and add the following line
   # 127.0.0.1 k3d-platform-poc.localhost
   sudo vim /etc/hosts
   ```

1. Start redis, postgres, NATS

   ```fish
   ./up.fish
   ```

1. Build the platform-poc application, and push to local registry

   ```fish
    ./build.sh
   ```

1. Install the platform-poc application

   ```fish
   ./deploy.fish
   ```

1. Delete the application

   ```fish
   ./undeploy.fish
   ```

1. Stop the redis, postgres, NATS

   ```fish
    ./down.fish
   ```

1. Delete the cluster

   ```fish
     ./destroy.fish
   ```

## Test

1. Run the test

   ```fish
   ./test.fish
   ```
