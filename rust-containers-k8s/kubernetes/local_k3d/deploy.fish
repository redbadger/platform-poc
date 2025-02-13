#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "deploying platform-poc"
kubectl apply -k $SCRIPT_DIR/../manifests/overlays/local
