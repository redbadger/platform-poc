## Local K3d cluster

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/)
- [K3d](https://k3d.io/#installation)


### Create a local K3d cluster

```bash
./create.fish
./up.fish
```

### Install wasmCloud operator

see [wasmCloud operator](../operator/README.md)

### Install redis

```bash
helm install redis oci://registry-1.docker.io/bitnamicharts/redis
```

### Delete the local K3d cluster

```bash
./down.fish
./destroy.fish
```
