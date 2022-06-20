# Exercise 1.06: Project v0.4

```shell
$ docker build . -t dogamak/hy-kubernetes-todo-app:v0.4
$ docker push dogamak/hy-kubernetes-todo-app:v0.4
$ kubectl apply -f manifests/deployment.yaml 
$ kubectl apply -f manifests/service.yaml 

$ curl http://localhost:8082/
<!doctype html><html lang="en"><head><meta charset="utf-8"/><link rel="icon" href="/favicon.ico"/><meta name="viewport" conte
nt="width=device-width,initial-scale=1"/><meta name="theme-color" content="#000000"/><meta name="description" content="Web si
te created using create-react-app"/><link rel="apple-touch-icon" href="/logo192.png"/><link rel="manifest" href="/manifest.js
on"/><title>React App</title><script defer="defer" src="/static/js/main.7097d855.js"></script><link href="/static/css/main.07
3c9b0a.css" rel="stylesheet"></head><body><noscript>You need to enable JavaScript to run this app.</noscript><div id="root"><
/div></body></html>%
```
