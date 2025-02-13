#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

set -x PORT 8091
set -x NATS_URL nats://localhost:4222

cd $SCRIPT_DIR/..
mvn spring-boot:run -pl notification-service -e
