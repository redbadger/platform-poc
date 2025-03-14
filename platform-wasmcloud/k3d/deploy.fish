#!/usr/bin/env fish

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "deploying components"
kubectl apply -f ./wadm.yaml
