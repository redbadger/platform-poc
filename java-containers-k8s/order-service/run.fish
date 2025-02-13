#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

set -x PORT 8092

set -x INVENTORY_URL http://localhost:8090/api/inventory

set -x POSTGRES_URL jdbc:postgresql://localhost/order-service
set -x POSTGRES_USERNAME order-service
set -x POSTGRES_PASSWORD commerce

set -x NATS_URL nats://localhost:4222


cd $SCRIPT_DIR/..
mvn spring-boot:run -pl order-service -e
