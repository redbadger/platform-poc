#!/usr/bin/env fish

wash down --all

pushd /tmp
    if test -f wash-ui.pid
        set PID (cat wash-ui.pid)
        rm -f wash-ui.pid wash-ui.out
        if test -n "$PID"
            kill $PID
        end
    end
popd
