#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "stopping platform-poc wasm components"
$SCRIPT_DIR/../stop.fish

section "stopping wasmcloud, NATS and wadm"
wash down --all

section "stopping local registry"
# local registry on port 5001
$SCRIPT_DIR/../registry.fish down

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

section "stopping redis"
brew services stop redis

section "stopping postgresql@15"
brew services stop postgresql@15

section "draining caches"
wash drain all
