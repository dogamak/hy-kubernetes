# Exercise 2.07: Stateful applications

```shell
$ kubectl create namespace log-ping-pong
$ docker build log-output-app -t dogamak/hy-kubernetes-log-output-app:v4
$ docker push dogamak/hy-kubernetes-log-output-app:v4
$ docker build ping-pong-app -t dogamak/hy-kubernetes-ping-pong-app:v3
$ docker push dogamak/hy-kubernetes-ping-pong-app:v3
$ kubectl apply -f log-output-app/manifests
$ curl http://localhost:8081/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 0
$ curl http://localhost:8081/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 0
$ curl http://localhost:8081/pingpong
pong 0
$ curl http://localhost:8081/pingpong
pong 1
$ curl http://localhost:8081/pingpong
pong 2
$ curl http://localhost:8081/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 3
$ curl http://localhost:8081/status
Hello
2022-06-20 21:08:08.341582034 UTC pBdR8LpiufXEJrvO
Ping / Pongs: 3
...
```
