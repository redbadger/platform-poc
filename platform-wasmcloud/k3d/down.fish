#!/usr/bin/env fish

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

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

section "stopping redis"
brew services stop redis

section "stopping postgresql@15"
brew services stop postgresql@15

section "stopping forwarding http"
stop http

section "stopping forwarding nats"
stop nats

section "stopping wash-ui"
stop wash-ui

section "stopping cluster"
k3d cluster stop platform-poc
