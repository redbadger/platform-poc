#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

$SCRIPT_DIR/undeploy.fish
$SCRIPT_DIR/deploy.fish
