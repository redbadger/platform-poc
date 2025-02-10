#!/usr/bin/env fish
set SCRIPT_DIR (dirname (realpath (status -f)))

# STAGE 1
pushd $SCRIPT_DIR/../stage_1
    tofu init
    tofu apply --var-file=../terraform.tfvars
popd

# Authenticate with the cluster
gcloud container clusters get-credentials platform-poc-wasmcloud-cluster \
    --project platform-poc-wasmcloud \
    --location europe-west2

# STAGE 2
pushd $SCRIPT_DIR/../stage_2
    tofu init
    tofu apply --var-file=../terraform.tfvars
popd
