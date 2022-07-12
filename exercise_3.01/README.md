# Exercise 2.07: Stateful applications

```shell
$ kubectl create namespace log-ping-pong
$ docker build log-output-app -t dogamak/hy-kubernetes-log-output-app:v4
$ docker push dogamak/hy-kubernetes-log-output-app:v4
$ docker build ping-pong-app -t dogamak/hy-kubernetes-ping-pong-app:v3
$ docker push dogamak/hy-kubernetes-ping-pong-app:v3
$ sops --decrypt manifests/postgres-secret.enc.yaml | kubectl apply -f -
$ find manifests -iname '*.yaml' -not -iname '*.enc.yaml' | xargs -I{} kubectl apply -f {}
$ kubectl get svc --namespace log-ping-pong
NAME             TYPE           CLUSTER-IP      EXTERNAL-IP    PORT(S)          AGE
log-output-svc   LoadBalancer   10.116.12.91    34.88.42.58    8080:32179/TCP   73m
ping-pong-svc    LoadBalancer   10.116.15.225   34.88.41.144   8080:31683/TCP   73m
postgres-svc     ClusterIP      None            <none>         5432/TCP         73m
$ curl http://34.88.42.58:8081/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 0
$ curl http://34.88.42.58:8081/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 0
$ curl http://34.88.41.144:8081/pingpong
pong 0
$ curl http://34.88.41.144:8081/pingpong
pong 1
$ curl http://34.88.41.144:8081/pingpong
pong 2
$ curl http://34.88.42.58:8081/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 3
$ curl http://34.88.42.58:8081/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 3
...
```
