#!/usr/bin/env fish

brew services start redis
brew services start postgresql@15

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
