# Kubernetes Configurations

## Deployment

```bash
# Create namespace
kubectl apply -f namespace.yaml

# Create ConfigMaps and Secrets
kubectl apply -f configmap.yaml
kubectl apply -f secrets.yaml

# Deploy infrastructure
kubectl apply -f infrastructure/postgres/k8s/
kubectl apply -f infrastructure/redis/k8s/
kubectl apply -f infrastructure/kafka/k8s/

# Deploy services
kubectl apply -f services/memorial-service/k8s/
kubectl apply -f services/admin-service/k8s/
kubectl apply -f services/frontend/k8s/
kubectl apply -f services/admin-panel/k8s/

# Setup ingress
kubectl apply -f k8s/ingress.yaml
```

## Verify Deployment

```bash
kubectl get namespaces
kubectl get deployments -n mortalbook
kubectl get pods -n mortalbook
kubectl get services -n mortalbook
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
