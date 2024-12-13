# cluster

## Create

```sh
minikube start --nodes=1 --memory=8192 --addons=dashboard,default-storageclass,metrics-server,storage-provisioner
helmfile apply --skip-diff-on-install --suppress-diff
kubectl apply -f mongodb.yaml -n standup
cd standup-operator
operator-sdk olm install
```

## Expose

```sh
minikube service -n kubernetes-dashboard kubernetes-dashboard
minikube service -n standup rabbitmq
minikube service -n standup grafana
```

RabbitMQ: `user`/`rabbit`
MongoDB: `standup`/`mongodb` (direct connection)
Grafana: `admin`/`admin`

## URLs

`http://standup-api.standup.svc.cluster.local:3000/work-items/all`

## Destroy

```sh
helmfile destroy
```
