#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

brew services start redis
brew services start postgresql@15

# local registry on port 5001
$SCRIPT_DIR/../registry.fish up


set -x WASMCLOUD_OCI_ALLOWED_INSECURE localhost:5001
wash up -d

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

daemon wash-ui wash ui -v 0.6.0
