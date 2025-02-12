#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

cd $SCRIPT_DIR

for service in *-service
    echo Building $service
    tar -czv --exclude=target Cargo.lock Cargo.toml *-service .sqlx \
        | docker build -f $service/Dockerfile -
    or echo "Failed to build $tag" && return 1
end
