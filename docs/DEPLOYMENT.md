# Deployment Guide

Complete guide for deploying Mortalbook to Kubernetes.

## Prerequisites

- Kubernetes cluster 1.24+ (EKS, GKE, AKS, or local minikube)
- `kubectl` configured with cluster access
- Docker registry credentials
- Container images built and pushed to registry

## Local Development with Docker Compose

### Quick Start

```bash
cd /path/to/mortalbook

# Build all services
docker-compose build

# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Access Services

- Frontend: http://localhost:3000
- Admin Panel: http://localhost:8080
- Memorial Service: http://localhost:5001
- Admin Service: http://localhost:5002
- PostgreSQL: localhost:5432
- Redis: localhost:6379
- Kafka: localhost:9092

### Initialize Database

```bash
# Connect to PostgreSQL
docker-compose exec postgres psql -U postgres -d mortalbook_db

# Or run initialization script
docker-compose exec postgres psql -U postgres -d mortalbook_db < infrastructure/postgres/init.sql
```

## Kubernetes Deployment

### 1. Prepare Cluster

#### Create Namespace

```bash
kubectl apply -f k8s/namespace.yaml

# Verify
kubectl get namespaces | grep mortalbook
```

#### Create ConfigMaps

```bash
kubectl apply -f k8s/configmap.yaml

# Verify
kubectl get configmaps -n mortalbook
```

#### Create Secrets

```bash
# Edit secrets with production values
kubectl apply -f k8s/secrets.yaml

# Verify
kubectl get secrets -n mortalbook
```

### 2. Deploy Infrastructure

#### PostgreSQL

```bash
# Apply all PostgreSQL resources
kubectl apply -f infrastructure/postgres/k8s/

# Verify deployment
kubectl get deployment postgres -n default
kubectl get service postgres -n default

# Wait for pod to be ready
kubectl wait --for=condition=ready pod -l app=postgres --timeout=300s

# Initialize database
kubectl exec -it deployment/postgres -- psql -U postgres -d mortalbook_db < infrastructure/postgres/init.sql
```

#### Redis

```bash
kubectl apply -f infrastructure/redis/k8s/

kubectl get deployment redis -n default
kubectl get service redis -n default

kubectl wait --for=condition=ready pod -l app=redis --timeout=300s

# Test connection
kubectl exec -it deployment/redis -- redis-cli ping
```

#### Kafka & Zookeeper

```bash
kubectl apply -f infrastructure/kafka/k8s/zookeeper.yaml
kubectl apply -f infrastructure/kafka/k8s/kafka.yaml

# Wait for both to be ready
kubectl wait --for=condition=ready pod -l app=zookeeper --timeout=300s
kubectl wait --for=condition=ready pod -l app=kafka --timeout=300s

# Create Kafka topics
kubectl exec -it deployment/kafka -- kafka-topics.sh --create \
  --bootstrap-server kafka:9092 \
  --topic memorial-events \
  --partitions 3 \
  --replication-factor 1

kubectl exec -it deployment/kafka -- kafka-topics.sh --create \
  --bootstrap-server kafka:9092 \
  --topic analytics-events \
  --partitions 3 \
  --replication-factor 1
```

### 3. Deploy Application Services

#### Memorial Service

```bash
# Update image in deployment
sed -i 's|IMAGE_PLACEHOLDER|ghcr.io/your-org/memorial-service:latest|g' \
  services/memorial-service/k8s/deployment.yaml

# Apply manifests
kubectl apply -f services/memorial-service/k8s/

# Verify
kubectl get deployment memorial-service -n mortalbook
kubectl get service memorial-service -n mortalbook

# Wait for rollout
kubectl rollout status deployment/memorial-service -n mortalbook --timeout=5m

# Check logs
kubectl logs -f deployment/memorial-service -n mortalbook
```

#### Admin Service

```bash
sed -i 's|IMAGE_PLACEHOLDER|ghcr.io/your-org/admin-service:latest|g' \
  services/admin-service/k8s/deployment.yaml

kubectl apply -f services/admin-service/k8s/

kubectl rollout status deployment/admin-service -n mortalbook --timeout=5m
```

#### Frontend

```bash
sed -i 's|IMAGE_PLACEHOLDER|ghcr.io/your-org/frontend:latest|g' \
  services/frontend/k8s/deployment.yaml

kubectl apply -f services/frontend/k8s/

kubectl rollout status deployment/frontend -n mortalbook --timeout=5m
```

#### Admin Panel

```bash
sed -i 's|IMAGE_PLACEHOLDER|ghcr.io/your-org/admin-panel:latest|g' \
  services/admin-panel/k8s/deployment.yaml

kubectl apply -f services/admin-panel/k8s/

kubectl rollout status deployment/admin-panel -n mortalbook --timeout=5m
```

### 4. Setup Ingress

#### Install Nginx Ingress Controller (if not installed)

```bash
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm repo update

helm install ingress-nginx ingress-nginx/ingress-nginx \
  --namespace ingress-nginx \
  --create-namespace
```

#### Install Cert-Manager (for HTTPS)

```bash
helm repo add jetstack https://charts.jetstack.io
helm repo update

helm install cert-manager jetstack/cert-manager \
  --namespace cert-manager \
  --create-namespace \
  --set installCRDs=true
```

#### Create SSL Certificate Issuer

```bash
cat <<EOF | kubectl apply -f -
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    server: https://acme-v02.api.letsencrypt.org/directory
    email: admin@mortalbook.com
    privateKeySecretRef:
      name: letsencrypt-prod
    solvers:
    - http01:
        ingress:
          class: nginx
EOF
```

#### Apply Ingress

```bash
# Update hostname in ingress.yaml
sed -i 's|mortalbook.local|mortalbook.com|g' k8s/ingress.yaml

kubectl apply -f k8s/ingress.yaml

# Verify ingress
kubectl get ingress -n mortalbook

# Wait for certificate
kubectl get certificate -n mortalbook -w
```

### 5. Network Policy

```bash
kubectl apply -f k8s/network-policy.yaml

# Verify
kubectl get networkpolicies -n mortalbook
```

## Verification

### Check All Deployments

```bash
# View all resources
kubectl get all -n mortalbook

# View pods status
kubectl get pods -n mortalbook

# Detailed pod status
kubectl describe pods -n mortalbook
```

### Service Discovery

```bash
# From within cluster
kubectl run -it --rm debug --image=curlimages/curl --restart=Never -- sh

# Test service connectivity
curl http://memorial-service:5001/health
curl http://admin-service:5002/health
curl http://frontend:3000
curl http://admin-panel:8080
```

### Database Connectivity

```bash
# Connect to PostgreSQL
kubectl exec -it deployment/postgres -- psql -U postgres -d mortalbook_db

# Verify tables
\dt

# Exit
\q
```

### Cache and Message Queue

```bash
# Test Redis
kubectl exec -it deployment/redis -- redis-cli ping

# Check Kafka topics
kubectl exec -it deployment/kafka -- kafka-topics.sh --list --bootstrap-server kafka:9092

# Monitor Kafka broker
kubectl exec -it deployment/kafka -- kafka-broker-api-versions.sh --bootstrap-server kafka:9092
```

## Scaling

### Horizontal Scaling

```bash
# Scale memorial service to 3 replicas
kubectl scale deployment memorial-service --replicas=3 -n mortalbook

# Scale admin service to 2 replicas
kubectl scale deployment admin-service --replicas=2 -n mortalbook

# Verify
kubectl get deployment -n mortalbook
```

### Update Deployment

```bash
# Update image
kubectl set image deployment/memorial-service \
  memorial-service=ghcr.io/your-org/memorial-service:v1.1.0 \
  -n mortalbook

# Monitor rollout
kubectl rollout status deployment/memorial-service -n mortalbook

# Rollback if needed
kubectl rollout undo deployment/memorial-service -n mortalbook
```

## Monitoring & Logs

### View Service Logs

```bash
# Single pod
kubectl logs pod/<pod-name> -n mortalbook

# All pods of service
kubectl logs -f deployment/memorial-service -n mortalbook

# Previous logs (after restart)
kubectl logs deployment/memorial-service -n mortalbook --previous

# Tail logs from all containers
kubectl logs -f deployment/memorial-service --all-containers=true -n mortalbook
```

### Port Forwarding

```bash
# Forward frontend to localhost:3000
kubectl port-forward svc/frontend 3000:3000 -n mortalbook

# Forward database to localhost:5432
kubectl port-forward svc/postgres 5432:5432

# Forward admin service to localhost:5002
kubectl port-forward svc/admin-service 5002:5002 -n mortalbook
```

### Pod Debugging

```bash
# Execute command in pod
kubectl exec -it deployment/memorial-service -- /bin/sh -n mortalbook

# Describe pod details
kubectl describe pod <pod-name> -n mortalbook

# Get pod events
kubectl get events -n mortalbook --sort-by='.lastTimestamp'
```

## Backup & Recovery

### Backup PostgreSQL Data

```bash
# Create backup
kubectl exec -it deployment/postgres -- pg_dump -U postgres mortalbook_db > backup.sql

# Restore backup
kubectl exec -i deployment/postgres -- psql -U postgres mortalbook_db < backup.sql
```

### Backup Redis Data

```bash
# Create snapshot
kubectl exec -it deployment/redis -- redis-cli BGSAVE

# Copy RDB file
kubectl cp default/redis-<pod>:/data/dump.rdb ./redis-dump.rdb
```

## Troubleshooting

### Pod Not Starting

```bash
# Check pod status
kubectl get pods -n mortalbook
kubectl describe pod <pod-name> -n mortalbook

# Check events
kubectl get events -n mortalbook --sort-by='.lastTimestamp'

# Check logs
kubectl logs <pod-name> -n mortalbook
```

### Service Connectivity Issues

```bash
# Test service resolution
kubectl run -it --rm debug --image=nicolaka/netshoot --restart=Never -- nslookup memorial-service

# Test service connectivity
kubectl run -it --rm debug --image=curlimages/curl --restart=Never -- curl http://memorial-service:5001/health
```

### Database Issues

```bash
# Check PostgreSQL pod
kubectl exec -it deployment/postgres -- pg_isready -U postgres

# Check connections
kubectl exec -it deployment/postgres -- psql -U postgres -c "SELECT count(*) FROM pg_stat_activity;"
```

### Disk Space Issues

```bash
# Check PVC usage
kubectl get pvc -n mortalbook
kubectl exec -it deployment/postgres -- df -h

# Expand PVC (if supported by storage class)
kubectl patch pvc postgres-pvc -p '{"spec":{"resources":{"requests":{"storage":"15Gi"}}}}' -n mortalbook
```

## Production Checklist

- [ ] Change all default passwords in `app-secrets`
- [ ] Update JWT_SECRET to secure random value
- [ ] Configure backup strategy for PostgreSQL
- [ ] Enable TLS for all services
- [ ] Set up monitoring and alerting
- [ ] Configure replicas for HA (minimum 2 replicas)
- [ ] Test disaster recovery procedures
- [ ] Document runbooks for common issues
- [ ] Set resource requests and limits
- [ ] Enable pod autoscaling (HPA)
- [ ] Configure network policies
- [ ] Set up log aggregation (ELK, Loki, etc.)
- [ ] Enable RBAC policies
- [ ] Regular security scanning
- [ ] Load testing before launch

## Continuous Deployment

### GitHub Actions Integration

Deployments are triggered automatically:

1. **CI Pipeline** runs on every push:
   - Linting (golangci-lint, black, eslint)
   - Unit tests
   - Build verification
   - Docker image build and push

2. **Staging Deployment** on `develop` branch:
   - Builds images with :develop and :sha tags
   - Deploys to staging cluster
   - Runs smoke tests

3. **Production Deployment** manual trigger:
   - Requires semver tag (v1.2.3)
   - Blue-green deployment strategy
   - Health checks before marking ready
   - Automatic rollback on failure

## Disaster Recovery

### Point-in-Time Recovery

```bash
# Find backup timestamp
kubectl exec -it deployment/postgres -- ls -la /var/lib/postgresql/backups/

# Restore from backup
kubectl exec -i deployment/postgres -- psql -U postgres mortalbook_db < /var/lib/postgresql/backups/backup.sql
```

### Cluster Recovery

```bash
# Export cluster state
kubectl get all -n mortalbook -o yaml > cluster-backup.yaml

# Restore on new cluster
kubectl apply -f cluster-backup.yaml
```

### Service Recovery

```bash
# Check failed deployments
kubectl get deployments -n mortalbook --field-selector spec.replicas!=status.replicas

# Reset deployment
kubectl rollout restart deployment/memorial-service -n mortalbook

# Force pod recreation
kubectl delete pod <pod-name> -n mortalbook
```
