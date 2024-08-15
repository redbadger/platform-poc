#!/usr/bin/env bash
set -eu

echo setting up configs...
wash config put default-nats subscriptions="platform-poc.order-notification"
wash config put default-redis url="redis://127.0.0.1:6379"
wash config put default-http address="127.0.0.1:8080"
wash config put default-pg \
    POSTGRES_HOST=localhost \
    POSTGRES_PORT=5432 \
    POSTGRES_USERNAME=${POSTGRES_USERNAME:-postgres} \
    POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-postgres} \
    POSTGRES_TLS_REQUIRED=false \
    POSTGRES_DATABASE=${POSTGRES_DATABASE:-postgres}

echo
echo "printing configs..."

echo "default-nats: $(wash config get default-nats)"
echo "default-redis: $(wash config get default-redis)"
echo "default-http: $(wash config get default-http)"
echo "default-pg: $(wash config get default-pg)"
