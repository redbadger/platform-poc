#!/usr/bin/env fish

wash up -d

pushd /tmp
    status job-control full
    nohup wash ui > wash-ui.out &
    sleep 1
    cat wash-ui.out
    set PID $last_pid
    echo $PID > wash-ui.pid
popd
