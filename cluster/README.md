# cluster

```
minikube start --nodes=1 --memory=8192 --addons=dashboard,default-storageclass,metrics-server,storage-provisioner
helmfile apply --skip-diff-on-install --suppress-diff
kubectl get secret -n standup rmq-rabbitmq -o jsonpath='{.data.rabbitmq-password}' | base64 -d
```

- [http://rmq-rabbitmq.standup.svc.cluster.local:15672]
