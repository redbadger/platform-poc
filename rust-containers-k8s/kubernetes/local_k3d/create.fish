#!/usr/bin/env fish

# Create a local k3d cluster with a private registry
k3d registry create platform-poc.localhost --port 5001

k3d cluster create platform-poc --agents 3 --registry-use k3d-platform-poc.localhost:5001

kubectl cluster-info
