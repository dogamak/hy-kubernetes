# Exercise 3.02: Back to ingress

```shell
$ kubectl create namespace log-ping-pong
$ docker build log-output-app -t dogamak/hy-kubernetes-log-output-app:v4
$ docker push dogamak/hy-kubernetes-log-output-app:v4
$ docker build ping-pong-app -t dogamak/hy-kubernetes-ping-pong-app:v4
$ docker push dogamak/hy-kubernetes-ping-pong-app:v4
$ sops --decrypt manifests/postgres-secret.enc.yaml | kubectl apply -f -
$ find manifests -iname '*.yaml' -not -iname '*.enc.yaml' | xargs -I{} kubectl apply -f {}
$ kubectl get svc --namespace log-ping-pong --watch
NAME                 CLASS    HOSTS   ADDRESS          PORTS   AGE
log-output-ingress   <none>   *       34.107.148.191   80      24h
$ curl http://34.107.148.191/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 0
$ curl http://34.107.148.191/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 0
$ curl http://34.107.148.191/pingpong
pong 0
$ curl http://34.107.148.191/pingpong
pong 1
$ curl http://34.107.148.191/pingpong
pong 2
$ curl http://34.107.148.191/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 3
$ curl http://34.107.148.191/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 3
...
```
