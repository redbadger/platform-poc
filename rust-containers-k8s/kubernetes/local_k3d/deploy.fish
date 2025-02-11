#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

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

section "deploying platform-poc"
kubectl apply -k $SCRIPT_DIR/../manifests/overlays/local

function forward
    set -l resource deployment/$argv[1]
    set -l port $argv[2]
    kubectl wait --for=condition=available --timeout=600s $resource
    daemon inventory kubectl port-forward $resource $port
end

section "starting port forwarders"
forward inventory-service 8082
