#!/usr/bin/env fish

k3d cluster delete platform-poc

k3d registry delete platform-poc.localhost
