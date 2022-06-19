# Exercise 1.01: Getting started

```shell
$ docker build . -t dogamak/hy-kubernetes-log-output-app
Sending build context to Docker daemon   51.2kB
Step 1/7 : FROM rust:1.61 AS builder
 ---> 51d2e62eb827
Step 2/7 : WORKDIR /usr/src/log-output-app
 ---> Running in f201faed6d08
Removing intermediate container f201faed6d08
 ---> 11fcb1a4f6d7
Step 3/7 : COPY . .
 ---> 4a112d38387e
Step 4/7 : RUN cargo install --path .
 ---> Running in 1d0358645ea9
  Installing log-output-app v0.1.0 (/usr/src/log-output-app)
    Updating crates.io index
 Downloading crates ...
  Downloaded getrandom v0.2.7
  Downloaded rand_chacha v0.3.1
  Downloaded rand v0.8.5
  Downloaded rand_core v0.6.3
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.16
  Downloaded libc v0.2.126
   Compiling libc v0.2.126
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling getrandom v0.2.7
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling log-output-app v0.1.0 (/usr/src/log-output-app)
    Finished release [optimized] target(s) in 1m 42s
  Installing /usr/local/cargo/bin/log-output-app
   Installed package `log-output-app v0.1.0 (/usr/src/log-output-app)` (executable `log-output-app`)
Removing intermediate container 1d0358645ea9
 ---> 5170e65c27fa
Step 5/7 : FROM debian:buster-slim
 ---> c2303a498941
Step 6/7 : COPY --from=builder /usr/local/cargo/bin/log-output-app /usr/local/bin/log-output-app
 ---> cb0f3e8a9ef9
Step 7/7 : CMD ["log-output-app"]
 ---> Running in d8a873802cc3
Removing intermediate container d8a873802cc3
 ---> 8dce1f7a1e03
Successfully built 8dce1f7a1e03
Successfully tagged dogamak/hy-kubernetes-log-output-app:latest

$ kubectl create deployment log-output-app-dep --image=dogamak/hy-kubernetes-log-output-app
deployment.apps/log-output-app-dep created

$ kubectl logs -f log-output-app-dep-776cd8969-hhxjk
CQTx8CTfmsdj2967
CQTx8CTfmsdj2967
CQTx8CTfmsdj2967
CQTx8CTfmsdj2967
CQTx8CTfmsdj2967
CQTx8CTfmsdj2967
...
```
