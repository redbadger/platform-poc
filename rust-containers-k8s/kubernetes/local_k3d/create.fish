#!/usr/bin/env fish

# Create a local k3d cluster with a private registry
k3d registry create platform-poc.localhost --port 5001

k3d cluster create platform-poc \
    --agents 2 \
    --registry-use k3d-platform-poc.localhost:5001 \
    --api-port 6550 \
    -p "8081:80@loadbalancer"

kubectl cluster-info
