#!/usr/bin/env bash

set -euo pipefail

registry=europe-west2-docker.pkg.dev
project=platform-poc-rust
sha="$(git rev-parse --short HEAD)"

for service in *-service; do
  tag="${registry}/${project}/${service}:${sha}"

  pushd "$service"

  echo "Building $service"
  docker build --tag "$tag" .

  echo "Pushing $service"
  docker push "$tag"

  popd
done
