#!/usr/bin/env fish

function daemon
    pushd /tmp
        status job-control full
        set -l name $argv[1]
        set -l command $argv[2..-1]
        command nohup $command > {$name}.out 2>&1 &
        echo {$name}...
        echo $last_pid > {$name}.pid
        sleep 0.1
        cat {$name}.out
    popd
end

k3d cluster start platform-poc

set wasmcloud_host (
    kubectl get pod --selector app.kubernetes.io/instance=wasmcloud-host -o name
)

daemon http     kubectl port-forward $wasmcloud_host 8080
daemon nats     kubectl port-forward svc/nats 4222 4223
daemon wash-ui  wash ui
