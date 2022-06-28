# Exercise 2.02: Project v1.0

```shell
$ docker build frontend -t dogamak/hy-kubernetes-todo-frontend:v1.0
$ docker push dogamak/hy-kubernetes-todo-frontend:v1.0
$ docker build backend -t dogamak/hy-kubernetes-todo-backend:v1.0
$ docker push dogamak/hy-kubernetes-todo-backend:v1.0
$ kubectl apply -f manifests
$ firefox http://localhost:8081/
```
