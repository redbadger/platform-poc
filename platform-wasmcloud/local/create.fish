#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "starting registry"
k3d registry create platform-poc.localhost --port 5001

section "stopping NATS in case it's running locally"
brew services stop nats-server

section "starting wasmcloud, NATS and wadm"
wash up \
    --detached \
    --allow-latest \
    --allowed-insecure "localhost:5001"

function daemon
    pushd /tmp
    status job-control full
    set -l name $argv[1]
    if test -f {$name}.pid
        set -l PID (cat {$name}.pid)
        rm -f {$name}.pid {$name}.out
        if test -n "$PID"
            echo "Killing $name with PID $PID"
            kill $PID
        end
    end
    set -l command $argv[2..-1]
    command nohup $command >{$name}.out 2>&1 &
    echo {$name}...
    echo $last_pid >{$name}.pid
    sleep 0.5
    cat {$name}.out
    popd
end

section "starting wash-ui"
daemon wash-ui wash ui -v 0.6.0
