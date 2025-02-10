#!/usr/bin/env fish

k3d cluster create platform-poc --agents 3

kubectl cluster-info
