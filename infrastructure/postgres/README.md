# PostgreSQL Infrastructure

PostgreSQL 16 database for Mortalbook application.

## Setup

```bash
kubectl apply -f configmap.yaml
kubectl apply -f secret.yaml
kubectl apply -f pvc.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
```

## Connection

```bash
Host: postgres
Port: 5432
Database: mortalbook_db
User: postgres
Password: postgres_dev_password
```

## Verification

```bash
kubectl exec -it deployment/postgres -- psql -U postgres -d mortalbook_db
```
