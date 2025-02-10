#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "starting redis"
brew services start redis

section "starting postgresql@15"
brew services start postgresql@15

section "starting local registry"
# local registry on port 5001
$SCRIPT_DIR/../registry.fish up

section "starting wasmcloud, NATS and wadm"
set -x WASMCLOUD_OCI_ALLOWED_INSECURE localhost:5001
wash up -d

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
    sleep 0.1
    cat {$name}.out
    popd
end

section "starting wash-ui"
daemon wash-ui wash ui -v 0.6.0
