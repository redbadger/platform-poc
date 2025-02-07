#!/usr/bin/env fish
set SCRIPT_DIR (dirname (realpath (status -f)))

# STAGE 2
pushd $SCRIPT_DIR/../stage_2
    tofu init
    tofu destroy --var-file=../terraform.tfvars
popd

# STAGE 1
pushd $SCRIPT_DIR/../stage_1
    tofu init
    tofu destroy --var-file=../terraform.tfvars
popd
