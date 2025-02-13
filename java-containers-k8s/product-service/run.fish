#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

set -x PORT 8093

cd $SCRIPT_DIR/..
mvn spring-boot:run -pl product-service -e
