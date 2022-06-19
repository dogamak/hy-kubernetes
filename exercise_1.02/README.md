# Exercise 1.02: Project v0.1

```shell
$ docker build . -t dogamak/hy-kubernetes-todo-app
Sending build context to Docker daemon  974.8MB
Step 1/7 : FROM rust:1.61 AS builder
 ---> 51d2e62eb827
Step 2/7 : WORKDIR /usr/src/todo-app
 ---> Running in 070a09fde7f6
Removing intermediate container 070a09fde7f6
 ---> 166f7cf86994
Step 3/7 : COPY . .
 ---> ac073801578a
Step 4/7 : RUN cargo install --path .
 ---> Running in 44578715c911
  Installing todo-app v0.1.0 (/usr/src/todo-app)
    Updating crates.io index
 Downloading crates ...
  Downloaded futures-sink v0.3.21
  Downloaded futures-task v0.3.21
  Downloaded rustc_version v0.4.0
         ...
   Compiling actix-http v3.1.0
   Compiling actix-web v4.1.0
   Compiling todo-app v0.1.0 (/usr/src/todo-app)
    Finished release [optimized] target(s) in 5m 18s
  Installing /usr/local/cargo/bin/todo-app
   Installed package `todo-app v0.1.0 (/usr/src/todo-app)` (executable `todo-app`)
Removing intermediate container 44578715c911
 ---> 41434277106c
Step 5/7 : FROM debian:buster-slim
 ---> c2303a498941
Step 6/7 : COPY --from=builder /usr/local/cargo/bin/todo-app /usr/local/bin/todo-app
 ---> 2eef0e7fb6bd
Step 7/7 : CMD ["todo-app"]
 ---> Running in b46f928c93cc
Removing intermediate container b46f928c93cc
 ---> 84b976c85039
Successfully built 84b976c85039
Successfully tagged dogamak/hy-kubernetes-todo-app:latest

$ docker push dogamak/hy-kubernetes-todo-app
Using default tag: latest
The push refers to repository [docker.io/dogamak/hy-kubernetes-todo-app]
b7a58067a4fb: Pushed
10e6bc6fdee2: Mounted from dogamak/hy-kubernetes-log-output-app
latest: digest: sha256:f9cda57e2927359e86a9df4a0149d9f577d2ef254763877dc69d177f62fa0839 size: 740

$ kubectl create deployment todo-app-dep --image=dogamak/hy-kubernetes-todo-app
deployment.apps/todo-app-dep created

$ kubectl logs -f todo-app-dep-54f85ffb9d-ttc54
HTTP server listening on port :8080
```
