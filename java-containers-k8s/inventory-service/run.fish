#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

set -x DATABASE_URL jdbc:postgresql://localhost/inventory-service
set -x DATABASE_USERNAME inventory-service
set -x DATABASE_PASSWORD commerce
set -x PORT 8090

cd $SCRIPT_DIR/..
mvn spring-boot:run -pl inventory-service -e
