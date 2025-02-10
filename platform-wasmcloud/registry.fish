#!/usr/bin/env fish

set -l name (basename (status -f))
set -l container_name oci_registry

argparse --name $name h/help -- $argv
or return 1

if test "$_flag_help"
    echo "Usage: $name up|down"
    return 0
end

function get_container
    docker ps -q --filter ancestor=registry:2
end

if test "$argv[1]" = up
    if test -z (get_container)
        docker run -d -p 5001:5000 --restart=always --name $container_name registry:2
    else
        echo "Registry is already running"
        return 1
    end
else if test "$argv[1]" = down
    set --local container (get_container)
    docker stop $container
    docker rm $container
else
    echo "Usage: $name up|down"
    return 1
end
