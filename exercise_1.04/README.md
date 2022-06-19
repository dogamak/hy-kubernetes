# Exercise 1.02: Project v0.1

```shell
➜  kubernetes-course git:(main) ✗ kubectl apply -f exercise_1.04/todo-app/manifests/deployment.yaml
deployment.apps/todo-app created
➜  kubernetes-course git:(main) ✗ kubectl get pods
NAME                                 READY   STATUS    RESTARTS   AGE
log-output-app-dep-776cd8969-hhxjk   1/1     Running   0          75m
todo-app-dep-54f85ffb9d-ttc54        1/1     Running   0          14m
log-output-app-66b998cc46-4cfvr      1/1     Running   0          6m17s
todo-app-fc778866-b6kmb              1/1     Running   0          6s
➜  kubernetes-course git:(main) ✗ kubectl logs todo-app-fc778866-b6kmb
HTTP server listening on port :8080
```
