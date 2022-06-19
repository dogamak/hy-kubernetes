# Exercise 1.01: Getting started

```shell
$ kubectl apply -f log-output-app/manifests/deployment.yaml
deployment.apps/log-output-app created

$ kubectl get pods
NAME                                 READY   STATUS    RESTARTS   AGE
log-output-app-dep-776cd8969-hhxjk   1/1     Running   0          69m
todo-app-dep-54f85ffb9d-ttc54        1/1     Running   0          8m11s
log-output-app-66b998cc46-4cfvr      1/1     Running   0          13s

$ kubectl logs -f log-output-app-66b998cc46-4cfvr
UdTs046QAjdanNiK
UdTs046QAjdanNiK
UdTs046QAjdanNiK
...
```
