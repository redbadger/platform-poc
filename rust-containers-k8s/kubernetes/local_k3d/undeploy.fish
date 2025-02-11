#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

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

section "undeploying platform-poc"
kubectl delete -k $SCRIPT_DIR/../manifests/overlays/local

section "starting port forwarders"
stop inventory
