#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "installing wasmcloud-operator"
helm upgrade --install \
    wasmcloud-platform \
    --values https://raw.githubusercontent.com/wasmCloud/wasmcloud/main/charts/wasmcloud-platform/values.yaml \
    oci://ghcr.io/wasmcloud/charts/wasmcloud-platform \
    --dependency-update

section "waiting for NATS to be available"
kubectl rollout status deploy,sts -l app.kubernetes.io/name=nats

section "waiting for wadm to be available"
kubectl wait --for=condition=available --timeout=600s deploy -l app.kubernetes.io/name=wadm

section "waiting for wasmcloud-operator to be available"
kubectl wait --for=condition=available --timeout=600s deploy -l app.kubernetes.io/name=wasmcloud-operator

section "apply wasmcloud-host config"
kubectl apply -f $SCRIPT_DIR/wasmcloud-host.yaml

helm upgrade --install \
    wasmcloud-platform \
    --values https://raw.githubusercontent.com/wasmCloud/wasmcloud/main/charts/wasmcloud-platform/values.yaml \
    oci://ghcr.io/wasmcloud/charts/wasmcloud-platform \
    --dependency-update \
    --set "hostConfig.enabled=true"

kubectl describe wasmcloudhostconfig wasmcloud-host
