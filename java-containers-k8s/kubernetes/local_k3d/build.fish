#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

cd $SCRIPT_DIR/../..

mvn clean package
