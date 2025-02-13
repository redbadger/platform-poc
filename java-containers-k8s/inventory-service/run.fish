#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

set -x POSTGRES_URL jdbc:postgresql://localhost/inventory-service
set -x POSTGRES_USERNAME inventory-service
set -x POSTGRES_PASSWORD commerce
set -x PORT 8090

cd $SCRIPT_DIR/..
mvn spring-boot:run -pl inventory-service -e
