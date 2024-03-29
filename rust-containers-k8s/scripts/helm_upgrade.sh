#!/bin/bash
set -e

COMMIT_SHA=$1

cd ./deployment

for service in notification-service order-service product-service inventory-service; do
  pushd $service
  echo "deploying $service..."
  helm upgrade $service -f values.yaml --set=image.tag="$COMMIT_SHA" .
  echo "$service deployed"
  popd
done
