#!/usr/bin/env fish

set -l name (basename (status -f))

argparse --name $name h/help -- $argv
or return 1

if test "$_flag_help"
    echo "Usage: $name up|down"
    return 0
end

if test "$argv[1]" = "up"
    docker run -d -p 5001:5000 --restart=always registry:2
else if test "$argv[1]" = "down"
    set --local container (docker ps -q --filter ancestor=registry:2)
    docker stop $container
    docker rm $container
else
    echo "Usage: $name up|down"
    return 1
end
