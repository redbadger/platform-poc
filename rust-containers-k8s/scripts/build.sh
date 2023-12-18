#!/usr/bin/env bash

set -euo pipefail

sha="$(git rev-parse --short HEAD)"

for service in *-service; do
  echo "Building $service"
  cd "$service"
  docker build --tag "gcr.io/${service}/${sha}" .
  cd -
done
