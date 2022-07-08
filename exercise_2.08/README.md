# Exercise 2.08: Project v1.2

```shell
$ kubectl create namespace todo-ns
$ docker build frontend -t dogamak/hy-kubernetes-todo-frontend:v1.2
$ docker push dogamak/hy-kubernetes-todo-frontend:v1.2
$ docker build backend -t dogamak/hy-kubernetes-todo-backend:v1.2
$ docker push dogamak/hy-kubernetes-todo-backend:v1.2
$ find manifests -iname '*.yaml' -not -iname '*.enc.yaml' | xargs -I{} kubectl apply -f {}
$ sops --decrypt manifests/postgres-secret.env.yaml | kubectl apply -f -
$ firefox http://localhost:8081/
```
