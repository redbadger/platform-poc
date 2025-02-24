#!/usr/bin/env fish

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "starting registry"
k3d registry create platform-poc.localhost --port 5001

section "starting platform-poc cluster"
k3d cluster create platform-poc \
    --agents 2 \
    --registry-use k3d-platform-poc.localhost:5001 \
    --api-port 6550 \
    -p "8081:80@loadbalancer"

section configuration
kubectl cluster-info
