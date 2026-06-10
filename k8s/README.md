# Kubernetes Configurations

## Deployment

```bash
# Build and publish images first, then apply the full stack bundle
kubectl apply -k k8s/
```

## Verify Deployment

```bash
kubectl get namespaces
kubectl get deployments -n mortalbook
kubectl get pods -n mortalbook
kubectl get services -n mortalbook
kubectl get ingress -n mortalbook
```

## Scaling

```bash
kubectl scale deployment memorial-service --replicas=3 -n mortalbook
kubectl scale deployment admin-service --replicas=3 -n mortalbook
```

## Monitoring

```bash
# View logs
kubectl logs -f deployment/memorial-service -n mortalbook

# Port forwarding
kubectl port-forward svc/frontend 3000:3000 -n mortalbook
kubectl port-forward svc/postgres 5432:5432 -n mortalbook
```
