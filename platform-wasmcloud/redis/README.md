### Install redis

```fish
helm install redis oci://registry-1.docker.io/bitnamicharts/redis
```

### Accessing the Redis&reg; cluster

Get the Redis&reg; password:

```fish
set REDIS_PASSWORD (kubectl get secret --namespace default redis -o jsonpath="{.data.redis-password}" | base64 -d)
```

Create a client pod:

```fish
./client.fish
```

Connect to your database using the Redis&reg; CLI:

```bash
REDISCLI_AUTH="$REDIS_PASSWORD" redis-cli -h redis-master
REDISCLI_AUTH="$REDIS_PASSWORD" redis-cli -h redis-replicas
```

To connect to your database from outside the cluster execute the following commands:

```fish
set REDIS_PASSWORD (kubectl get secret --namespace default redis -o jsonpath="{.data.redis-password}" | base64 -d)

kubectl port-forward --namespace default svc/redis-master 6379:6379 &
REDISCLI_AUTH="$REDIS_PASSWORD" redis-cli -h 127.0.0.1 -p 6379
```
