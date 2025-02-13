#!/usr/bin/env fish

set --local host k3d-platform-poc.localhost:5001
set --local project platform-poc-rust
set --local semver 0.1.0

set --local SCRIPT_DIR (dirname (realpath (status -f)))

cd $SCRIPT_DIR/../..

set -l services (ls -d *-service)

if test -n "$argv[1]"
    set services $argv[1]
end

for service in $services
    set --local tag "$host/$project/$service:$semver"

    echo Building $tag
    tar -czv --exclude=target Cargo.lock Cargo.toml *-service .sqlx \
        | docker build -f $service/Dockerfile --tag $tag -
    or echo "Failed to build $tag" && return 1

    echo Pushing $tag
    docker push "$tag"
end
