# Examples

### Pod list

Create a deployment:

```bash
kubectl create deployment hello-node --image=registry.k8s.io/e2e-test-images/agnhost:2.39 -- /agnhost netexec --http-port=8080
```

View the pod:

```bash
kubectl get pods

cargo run --example pod_list
```

Clean up:

```bash
kubectl delete deployment hello-node
```

