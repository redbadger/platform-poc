#!/usr/bin/env fish

set POSTGRES_PASSWORD (kubectl get secret --namespace default postgres-postgresql -o jsonpath="{.data.postgres-password}" | base64 -d)
echo "Password: $POSTGRES_PASSWORD"

kubectl run postgres-postgresql-client --rm --tty -i \
    --namespace default \
    --restart Never \
    --image docker.io/bitnami/postgresql:16.4.0-debian-12-r4 \
    --env "PGPASSWORD=$POSTGRES_PASSWORD" \
    --command -- \
        psql --host postgres-postgresql -U postgres -d postgres -p 5432
