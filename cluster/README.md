# cluster

## Create

```sh
minikube start --nodes=1 --memory=8192 --addons=dashboard,default-storageclass,metrics-server,storage-provisioner
helmfile apply --skip-diff-on-install --suppress-diff
kubectl apply -f mongodb.yaml -n standup
# patch dockerfiles to x86
# `skaffold run` each microservice component
cd standup-operator
make docker-build
minikube image load controller:latest
make install
make deploy
# load grafana datasource
# import dashboard
```

## Expose

```sh
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
