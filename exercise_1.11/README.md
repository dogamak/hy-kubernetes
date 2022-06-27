# Exercise 1.11: Persisting data

```shell
$ docker build reader -t dogamak/hy-kubernetes-log-reader:v1
$ docker push dogamak/hy-kubernetes-log-reader:v1
$ docker build writer -t dogamak/hy-kubernetes-log-writer:v1
$ docker push dogamak/hy-kubernetes-log-writer:v1
$ docker exec k3d-k3s-default-agent-0 mkdir /tmp/kube
$ kubectl apply -f manifests/
$ kubectl logs deployment/log-output-app -c reader -f
2022-06-27 16:43:49.282240954 UTC ee629b2802f4d94444bf1ef43763072e461f44a006adfb7e2679ed2a432d8402
2022-06-27 16:43:54.282430698 UTC f1ac93e897dd31b40af6e3b068ffc96a33c9cd1fadb0feaffcdd146d054e4819
2022-06-27 16:43:59.282647697 UTC 89cb48b3677ed284b2e58012c53b0d1f3b638d2be8e652b32e85913f9d4cd81b
2022-06-27 16:44:04.282980202 UTC e618878259cbff84f4a56aea5da4d0088880eac02db95d71c51e054642787392
2022-06-27 16:44:09.283244193 UTC 9199943b6ebf9feea774aa895ba0a2b58f393e5be06c24520a31cfac28365bda
...
```
