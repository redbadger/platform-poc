#!/usr/bin/env fish

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "deleting platform-poc cluster"
k3d cluster delete platform-poc

section "deleting registry"
k3d registry delete platform-poc.localhost
