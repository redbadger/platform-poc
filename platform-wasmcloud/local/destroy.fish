#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "undeploying wasm components"
$SCRIPT_DIR/undeploy.fish

section "stopping wasmcloud, NATS and wadm"
wash down --all

section "deleting registry"
k3d registry delete platform-poc.localhost

function stop
    set -l name $argv[1]
    pushd /tmp
    if test -f {$name}.pid
        set -l PID (cat {$name}.pid)
        rm -f {$name}.pid {$name}.out
        if test -n "$PID"
            echo "Killing $name with PID $PID"
            kill $PID
        end
    end
    popd
end

section "stopping wash-ui"
stop wash-ui

section "draining caches"
wash drain all
