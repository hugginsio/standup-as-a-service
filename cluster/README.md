# cluster

```
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml
```

```
helm install redis oci://registry-1.docker.io/bitnamicharts/redis -f redis.yaml
```

```
helm repo add redisinsight https://mrnim94.github.io/redisinsight
helm install redisinsight redisinsight/redisinsight
```

- `http://redisinsight.default.svc.cluster.local:5540`
- `redis-master.default.svc.cluster.local:6379`
