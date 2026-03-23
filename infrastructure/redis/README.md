# Redis Infrastructure

Redis 7 for caching and pub/sub.

## Setup

```bash
kubectl apply -f pvc.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
```

## Connection

```bash
Host: redis
Port: 6379
```

## Verification

```bash
kubectl exec -it deployment/redis -- redis-cli ping
```
