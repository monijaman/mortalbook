# Mortalbook - Production-Ready Memorial Microservices Platform

A comprehensive, Kubernetes-ready microservices platform for creating, managing, and honoring memorials. Built with Go (gRPC + REST), FastAPI (Python), Next.js, and Vue 3, following SOLID principles and clean architecture patterns.

![Build Status](https://github.com/your-org/mortalbook/workflows/CI/badge.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Go](https://img.shields.io/badge/Go-1.21+-00ADD8?logo=go)
![Python](https://img.shields.io/badge/Python-3.11+-3670A0?logo=python)
![Node.js](https://img.shields.io/badge/Node.js-18+-339933?logo=node.js)

## Quick Start

### Prerequisites

- Docker & Docker Compose (or Go 1.21+, Python 3.11+, Node.js 18+)
- PostgreSQL 16, Redis 7, Kafka 7.5 (included in Docker Compose)

### Run All Services

```bash
# Clone repository
git clone https://github.com/your-org/mortalbook.git
cd mortalbook

# Copy environment file
cp .env.example .env

# Start all services
docker-compose up -d

# Initialize database
docker-compose exec postgres psql -U postgres -d mortalbook_db < infrastructure/postgres/init.sql

# Access services
# Frontend: http://localhost:3000
# Admin Panel: http://localhost:8080
# Memorial Service API: http://localhost:5001
# Admin Service API: http://localhost:5002
```

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│         React Frontend (Next.js SSR) + Vue Admin        │
│         TypeScript, Tailwind CSS, Responsive            │
└──────────────────────┬──────────────────────────────────┘
                       │
         ┌─────────────▼──────────────┐
         │  API Gateway / Ingress     │
         │  (Kubernetes/Nginx)        │
         └──────────────┬─────────────┘
                        │
        ┌───────────────┴───────────────┐
        │                               │
    ┌───▼────────────┐         ┌───────▼────────┐
    │ Memorial       │         │ Admin Service  │
    │ Service (Go)   │         │ (FastAPI)      │
    │ gRPC + REST    │         │ Authentication │
    │ 5001/50001     │         │ Users, Logs    │
    │                │         │ 5002           │
    └────┬───────────┘         └────┬───────────┘
         │                           │
         └────────────┬──────────────┘
                      │ Kafka Events
    ┌─────────────────▼──────────────┐
    │   Infrastructure Services      │
    │                                │
    │  PostgreSQL 16 (5432)         │
    │  Redis 7 (6379)               │
    │  Kafka 7.5 (KRaft, 9092)      │
    └────────────────────────────────┘
```

### System Components

| Service              | Technology          | Port       | Purpose                                          |
| -------------------- | ------------------- | ---------- | ------------------------------------------------ |
| **Memorial Service** | Go 1.21             | 5001/50001 | Memorial CRUD, Media, gRPC/REST APIs             |
| **Admin Service**    | FastAPI/Python 3.11 | 5002       | Authentication, User/Admin Management, Analytics |
| **Frontend**         | Next.js 14/React    | 3000       | Public memorial site, SSR, SEO-optimized         |
| **Admin Panel**      | Vue 3/Vite          | 8080       | Admin dashboard, Memorial/User management        |
| **PostgreSQL**       | 16                  | 5432       | Primary data store                               |
| **Redis**            | 7                   | 6379       | Caching, pub/sub                                 |
| **Kafka**            | 7.5                 | 9092       | Event streaming, async messaging                 |

## Key Features

### Memorial Service

- ✅ Memorial CRUD operations with gRPC and REST
- ✅ Media management (photos, videos, documents)
- ✅ Event publishing (Kafka)
- ✅ Caching with Redis
- ✅ Connection pooling for databases
- ✅ Health checks and metrics
- ✅ Structured logging with zap

### Admin Service

- ✅ JWT authentication (HS256)
- ✅ Bcrypt password hashing
- ✅ User management and RBAC
- ✅ Admin audit logging
- ✅ Analytics API
- ✅ Dependency injection
- ✅ Structured JSON logging

### Frontend

- ✅ Server-side rendering (SSR)
- ✅ Static site generation (SSG) with ISR
- ✅ Image optimization
- ✅ Security headers
- ✅ TypeScript for type safety
- ✅ Tailwind CSS responsive design
- ✅ API client with error handling

### Admin Panel

- ✅ Authentication with JWT tokens
- ✅ Protected routes with role-based access
- ✅ User and memorial management tables
- ✅ Responsive dashboard with stats
- ✅ Notifications/toast system
- ✅ Vue 3 Composition API
- ✅ Pinia state management

## SOLID Principles & Clean Architecture

### Design Patterns

```
Domain Models → Repositories (Data Access)
    ↓
Services (Business Logic) → Handlers (HTTP/API)
    ↓
Clients (Frontend)
```

All services follow:

- **Single Responsibility**: Each layer has one purpose
- **Open/Closed**: Extensible without modification
- **Liskov Substitution**: Interface-based contracts
- **Interface Segregation**: Minimal interfaces
- **Dependency Inversion**: Depend on abstractions

## Project Structure

```
mortalbook/
├── services/
│   ├── memorial-service/          # Go gRPC + REST
│   │   ├── cmd/main.go
│   │   ├── pkg/domain/
│   │   ├── pkg/repository/
│   │   ├── pkg/service/
│   │   ├── pkg/handler/
│   │   ├── pkg/db/
│   │   ├── Dockerfile
│   │   ├── go.mod
│   │   ├── k8s/
│   │   └── README.md
│   │
│   ├── admin-service/             # FastAPI Python
│   │   ├── app/main.py
│   │   ├── app/core/
│   │   ├── app/db/
│   │   ├── app/api/routes/
│   │   ├── Dockerfile
│   │   ├── requirements.txt
│   │   ├── k8s/
│   │   └── README.md
│   │
│   ├── frontend/                  # Next.js React
│   │   ├── pages/
│   │   ├── components/
│   │   ├── lib/
│   │   ├── styles/
│   │   ├── package.json
│   │   ├── next.config.js
│   │   ├── tailwind.config.js
│   │   ├── k8s/
│   │   └── README.md
│   │
│   └── admin-panel/               # Vue 3
│       ├── src/
│       │   ├── main.js
│       │   ├── App.vue
│       │   ├── views/
│       │   ├── components/
│       │   ├── stores/
│       │   ├── services/
│       │   ├── router/
│       │   └── styles/
│       ├── index.html
│       ├── package.json
│       ├── vite.config.js
│       ├── tailwind.config.js
│       ├── k8s/
│       └── README.md
│
├── infrastructure/
│   ├── postgres/
│   │   ├── init.sql
│   │   ├── k8s/
│   │   └── README.md
│   ├── redis/
│   │   ├── k8s/
│   │   └── README.md
│   └── kafka/
│       ├── k8s/
│       └── README.md
│
├── k8s/
│   ├── namespace.yaml
│   ├── configmap.yaml
│   ├── secrets.yaml
│   ├── ingress.yaml
│   ├── network-policy.yaml
│   └── README.md
│
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── deploy-staging.yml
│       └── deploy-prod.yml
│
├── docs/
│   ├── ARCHITECTURE.md
│   ├── API.md
│   ├── DEPLOYMENT.md
│   ├── DEVELOPMENT.md
│   └── CONTRIBUTING.md
│
├── docker-compose.yml
├── Makefile
├── .env.example
└── README.md
```

## Installation & Setup

### Development (Local)

```bash
# Step 1: Prerequisites
# - Go 1.21+
# - Python 3.11+
# - Node.js 18+
# - Docker & Docker Compose

# Step 2: Clone and setup
git clone https://github.com/your-org/mortalbook.git
cd mortalbook
cp .env.example .env

# Step 3: Start services
docker-compose up -d

# Step 4: Initialize database
docker-compose exec postgres psql -U postgres -d mortalbook_db < infrastructure/postgres/init.sql

# Step 5: Run services independently (optional)
# Memorial Service
cd services/memorial-service && go run cmd/main.go

# Admin Service (in another terminal)
cd services/admin-service && python -m uvicorn app.main:app --reload

# Frontend (in another terminal)
cd services/frontend && npm run dev

# Admin Panel (in another terminal)
cd services/admin-panel && npm run dev
```

### Local Kubernetes with Minikube

**1. Install minikube** (if not installed):

```bash
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
mkdir -p ~/.local/bin
install minikube-linux-amd64 ~/.local/bin/minikube
rm minikube-linux-amd64
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**2. Start the cluster:**

```bash
minikube start --driver=docker
```

**3. Build images directly into minikube's Docker daemon** (no registry needed):

```bash
eval $(minikube docker-env)
make build-services VERSION=1.0.0
```

**4. Fill in secrets** — edit `k8s/secrets.yaml` and replace placeholder values:

```yaml
DB_PASSWORD: "your-password"      # must match k8s/vendor/infrastructure/postgres/k8s/secret.yaml
JWT_SECRET: "your-32+-char-secret"
ADMIN_SERVICE_SECRET_KEY: "your-admin-secret"
```

**5. Deploy everything:**

```bash
kubectl apply -k k8s/
# or, for ordered deploy that waits for infra readiness:
make k8s-apply
```

**6. Verify all pods are running:**

```bash
make k8s-status
# or
kubectl get pods -n mortalbook
```

**7. Access the app:**

```bash
# Option A — port forward
kubectl port-forward svc/frontend 3000:3000 -n mortalbook
kubectl port-forward svc/admin-panel 8080:8080 -n mortalbook

# Option B — minikube service tunnel (opens browser automatically)
minikube service frontend -n mortalbook
minikube service admin-panel -n mortalbook
```

> **Note for MicroK8s users:** If you have MicroK8s installed alongside minikube, its `kubectl`
> wrapper takes priority in PATH and may show a permissions error. Either fix the permissions:
> ```bash
> sudo usermod -a -G microk8s $USER && sudo chown -R $USER ~/.kube
> newgrp microk8s
> ```
> Or use minikube's own kubectl to bypass it:
> ```bash
> minikube kubectl -- port-forward svc/frontend 3000:3000 -n mortalbook
> ```

**Tear down:**

```bash
make k8s-delete
# or stop the cluster entirely
minikube stop
```

### Production (Kubernetes)

```bash
# Step 1: Push images to registry
docker-compose push

# Step 2: Deploy to Kubernetes
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/secrets.yaml

# Step 3: Deploy infrastructure
kubectl apply -f infrastructure/postgres/k8s/
kubectl apply -f infrastructure/redis/k8s/
kubectl apply -f infrastructure/kafka/k8s/

# Step 4: Deploy services
kubectl apply -f services/*/k8s/

# Step 5: Setup ingress
kubectl apply -f k8s/ingress.yaml

# Verify deployment
kubectl get deployments -n mortalbook
kubectl get pods -n mortalbook
```

See [DEPLOYMENT.md](docs/DEPLOYMENT.md) for detailed instructions.

## API Documentation

### Memorial Service (REST & gRPC)

```http
GET    /api/v1/memorials              # List memorials
GET    /api/v1/memorials/{id}         # Get memorial
POST   /api/v1/memorials              # Create memorial
PUT    /api/v1/memorials/{id}         # Update memorial
DELETE /api/v1/memorials/{id}         # Delete memorial

POST   /api/v1/memorials/{id}/media   # Upload media
DELETE /api/v1/memorials/{id}/media/{mid}  # Delete media
```

### Admin Service (FastAPI)

```http
POST   /api/v1/auth/login              # User login
POST   /api/v1/auth/register           # User registration

GET    /api/v1/admin/users             # List users (admin)
DELETE /api/v1/admin/users/{id}        # Delete user (admin)

GET    /api/v1/analytics/dashboard     # Dashboard stats
GET    /api/v1/analytics/events        # Analytics events

GET    /api/v1/admin/logs              # Admin logs (admin)
```

See [API.md](docs/API.md) for complete specifications.

## Development Guide

### Running Services Locally

See [DEVELOPMENT.md](docs/DEVELOPMENT.md) for:

- Environment setup
- Database migrations
- Running tests
- Debugging tips
- IDE recommendations

### Code Style & Standards

- **Go**: Follow [Effective Go](https://golang.org/doc/effective_go), use golangci-lint
- **Python**: Follow [PEP 8](https://pep8.org/), use Black + isort
- **JavaScript/Vue**: Use ESLint + Prettier, TypeScript strict mode
- **All**: Meaningful variable names, proper error handling, comprehensive tests

### Testing Strategy

```bash
# Go tests
cd services/memorial-service
go test -v -race -cover ./...

# Python tests
cd services/admin-service
pytest -v --cov=app

# Frontend tests
cd services/frontend
npm run test

# Admin Panel tests
cd services/admin-panel
npm run test
```

## Docker & Kubernetes

### Docker Compose (Development)

```bash
docker-compose build      # Build all images
docker-compose up -d      # Start all services
docker-compose down       # Stop all services
docker-compose logs -f    # View logs
docker-compose restart    # Restart services
```

### Kubernetes (Production)

```bash
# Deploy all services
kubectl apply -f k8s/
kubectl apply -f infrastructure/

# Monitor deployment
kubectl get deployments -n mortalbook
kubectl describe pod <pod-name> -n mortalbook
kubectl logs deployment/memorial-service -n mortalbook

# Scale service
kubectl scale deployment memorial-service --replicas=3 -n mortalbook

# Update image
kubectl set image deployment/memorial-service \
  memorial-service=ghcr.io/your-org/memorial-service:v1.1.0 \
  -n mortalbook

# Port forwarding
kubectl port-forward svc/frontend 3000:3000 -n mortalbook
```

## CI/CD Pipelines

### GitHub Actions Workflows

| Workflow              | Trigger           | Action                                               |
| --------------------- | ----------------- | ---------------------------------------------------- |
| **CI**                | Push/PR           | Lint, test, build images                             |
| **Deploy Staging**    | Push to `develop` | Build, push images, deploy to staging                |
| **Deploy Production** | Manual trigger    | Blue-green deploy to production, rollback on failure |

See `.github/workflows/` for implementation details.

## Security

### Authentication & Authorization

- JWT tokens (HS256) for stateless authentication
- Bcrypt hashing for passwords (cost=12)
- Role-based access control (is_admin flag)
- Admin audit logging

### Data Protection

- Passwords hashed before storage
- Secrets managed via environment variables and Kubernetes Secrets
- HTTPS/TLS for all traffic
- CORS configured per environment
- SQL injection protection (parameterized queries)
- CSRF token validation

### Network Security

- NetworkPolicy restricts traffic to mortalbook namespace
- Ingress TLS with automatic cert provisioning
- Service-to-service mTLS (optional)
- Non-root containers, read-only filesystems

## Monitoring & Observability

### Application Metrics

- HTTP request count/latency (Prometheus-compatible)
- Database query duration
- Cache hit/miss ratio
- Kafka producer/consumer lag
- Application-level business metrics

### Logging

All services export structured JSON logs:

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "service": "memorial-service",
  "request_id": "uuid",
  "action": "memorial.created",
  "duration_ms": 125,
  "status": 201
}
```

### Health Checks

All services expose health endpoints:

- `GET /health` - Liveness probe
- `GET /ready` - Readiness probe

## Performance Optimizations

- **Connection Pooling**: Database and Redis
- **Caching**: Redis with 5-minute TTL for frequent queries
- **Database Indexes**: On created_at, status, user_id fields
- **Pagination**: 50 records per page by default
- **Image Optimization**: Next.js Image component with CDN
- **Code Splitting**: Route-based chunks for frontend
- **Compression**: gzip for HTTP responses

## Database Schema

### Core Tables

- **users** - User accounts with authentication
- **memorials** - Memorial entries
- **memorial_media** - Photos, videos, documents
- **comments** - Comments on memorials
- **admin_logs** - Audit trail of admin actions
- **analytics_events** - User behavior tracking

See [ARCHITECTURE.md](docs/ARCHITECTURE.md#database-schema) for detailed schema.

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Follow [CONTRIBUTING.md](docs/CONTRIBUTING.md)
4. Commit with clear messages: `git commit -am 'feat(service): add new feature'`
5. Push to branch: `git push origin feature/your-feature`
6. Submit Pull Request with description

## Roadmap

- [ ] GraphQL API layer
- [ ] Elasticsearch integration for full-text search
- [ ] Service mesh (Istio) for advanced traffic management
- [ ] Machine learning (recommendations, content moderation)
- [ ] Mobile app (React Native)
- [ ] Real-time notifications (WebSocket)
- [ ] Advanced analytics dashboard
- [ ] Multi-language support

## Support & Community

- 📖 [Documentation](docs/)
- 🐛 [Report Issues](https://github.com/your-org/mortalbook/issues)
- 💬 [Discussions](https://github.com/your-org/mortalbook/discussions)
- 📧 [Email Support](mailto:support@mortalbook.com)

## Troubleshooting

### Service Won't Start

```bash
# Check logs
docker-compose logs memorial-service

# Check port availability
lsof -i :5001

# Restart service
docker-compose restart memorial-service
```

### Database Connection Issues

```bash
# Test connection
psql -h localhost -U postgres -d mortalbook_db

# Reinitialize database
docker-compose exec postgres psql -U postgres -d mortalbook_db < infrastructure/postgres/init.sql
```

### Kubernetes Deployment Issues

```bash
# Check pod status
kubectl get pods -n mortalbook
kubectl describe pod <pod-name> -n mortalbook
kubectl logs <pod-name> -n mortalbook

# Debug pod
kubectl debug pod/<pod-name> -it -n mortalbook
```

See [DEPLOYMENT.md](docs/DEPLOYMENT.md#troubleshooting) for more troubleshooting tips.

## License

MIT License - see LICENSE file for details.

## Acknowledgments

Built with inspiration from:

- Kubernetes best practices
- SOLID principles
- Clean architecture patterns
- Industry-standard microservices design

---

**[📚 Architecture](docs/ARCHITECTURE.md) | [📡 API Docs](docs/API.md) | [🚀 Deployment](docs/DEPLOYMENT.md) | [💻 Development](docs/DEVELOPMENT.md) | [🤝 Contributing](docs/CONTRIBUTING.md)**

Made with ❤️ for honoring those we've lost.
