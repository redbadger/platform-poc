#!/usr/bin/env bash

set -euo pipefail

host=europe-west2-docker.pkg.dev
project=platform-poc-rust
sha="$(git rev-parse --short HEAD)"

for service in *-service; do
  tag="${host}/${project}/${service}/${service}:${sha}"

  pushd "$service"

  echo "Building $service"
  docker build --tag "$tag" .

  echo "Pushing $service"
  docker push "$tag"

  popd
done
