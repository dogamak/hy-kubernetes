# Exercise 2.10: Project v1.3

```shell
$ kubectl create namespace todo-ns
$ docker build frontend -t dogamak/hy-kubernetes-todo-frontend:v1.3
$ docker push dogamak/hy-kubernetes-todo-frontend:v1.3
$ docker build backend -t dogamak/hy-kubernetes-todo-backend:v1.3 --target todo-app
$ docker push dogamak/hy-kubernetes-todo-backend:v1.3
$ docker build backend -t dogamak/hy-kubernetes-todo-daily-todo-job:v1.3 --target daily-todo-job
$ docker push dogamak/hy-kubernetes-todo-daily-todo-job:v1.3
$ find manifests -iname '*.yaml' -not -iname '*.enc.yaml' | xargs -I{} kubectl apply -f {}
$ sops --decrypt manifests/postgres-secret.env.yaml | kubectl apply -f -
$ firefox http://localhost:8081/
```
