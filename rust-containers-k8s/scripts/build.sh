#!/usr/bin/env bash

set -euo pipefail

host=europe-west2-docker.pkg.dev
project=platform-poc-rust
repository=registry
sha="$(git rev-parse --short HEAD)"

for service in *-service; do
  tag="${host}/${project}/${repository}/${service}:${sha}"

  pushd "$service"

  echo "Building $service"
  docker build --tag "$tag" .

  popd
done
