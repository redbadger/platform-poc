#!/usr/bin/env fish

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

stop http
stop nats
stop wash-ui

k3d cluster stop platform-poc
