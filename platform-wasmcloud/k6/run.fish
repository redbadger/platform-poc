#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

cd $SCRIPT_DIR

k6 run script.js --out influxdb='http://localhost:8181/k6'
