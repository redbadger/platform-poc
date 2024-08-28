#!/usr/bin/env fish


set REDIS_PASSWORD (kubectl get secret --namespace default redis -o jsonpath="{.data.redis-password}" | base64 -d)

echo "!!! Press <shift-r> to get the redis cli prompt"

kubectl run redis-client --rm --tty -i \
    --namespace default \
    --restart Never \
    --image docker.io/bitnami/redis:7.4.0-debian-12-r2 \
    --env "REDISCLI_AUTH=$REDIS_PASSWORD" \
    --command -- \
        redis-cli -h redis-master
