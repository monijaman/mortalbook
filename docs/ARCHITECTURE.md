# Architecture Documentation

## System Overview

Mortalbook is a production-ready microservices platform for creating, managing, and honoring memorials. The system is designed using SOLID principles and clean architecture patterns to ensure scalability, maintainability, and resilience.

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Client Layer                                 │
├──────────────────┬────────────────────────┬──────────────────────────┤
│  Web Browser     │    Admin Dashboard     │    Mobile App            │
│  (Next.js)       │    (Vue 3 Admin)       │    (API Clients)         │
└────────┬─────────┴──────────┬─────────────┴──────────────┬───────────┘
         │                    │                            │
         └────────────────────┼────────────────────────────┘
                              │
         ┌────────────────────▼────────────────────┐
         │    API Gateway / Ingress Controller     │
         │    (Nginx/Kubernetes Ingress)           │
         └────────────┬──────────────────┬─────────┘
                      │                  │
         ┌────────────▼────┐  ┌──────────▼──────────┐
         │   Frontend      │  │  Admin Panel       │
         │   (Next.js 14)  │  │  (Vue 3)           │
         │   3000          │  │  8080              │
         └────────┬────────┘  └──────────┬─────────┘
                  │                      │
         ┌────────▼──────────────────────▼─────────┐
         │       API Services Layer                │
         ├──────────────────────────────────────────┤
         │                                          │
         │  ┌──────────────────┐  ┌──────────────┐ │
         │  │ Memorial Service │  │ Admin Service│ │
         │  │ (Go gRPC/REST)   │  │ (FastAPI)    │ │
         │  │ 5001/50001       │  │ 5002         │ │
         │  └──────────────────┘  └──────────────┘ │
         │         ▲                       ▲        │
         │         │                       │        │
         │         └───────┬───────────────┘        │
         │                 │ Events                 │
         └─────────────────┼──────────────────────┘
                           │
         ┌─────────────────▼────────────────────┐
         │  Infrastructure Services              │
         ├──────────────────────────────────────┤
         │                                       │
         │  ┌──────────────┐  ┌──────────────┐  │
         │  │ PostgreSQL   │  │  Redis       │  │
         │  │ 5432         │  │  6379        │  │
         │  └──────────────┘  └──────────────┘  │
         │                                       │
         │  ┌──────────────────────────────┐    │
         │  │  Kafka + Zookeeper           │    │
         │  │  9092 (Kafka) / 2181 (ZK)    │    │
         │  └──────────────────────────────┘    │
         │                                       │
         └───────────────────────────────────────┘
```

## Design Principles

### SOLID Principles

1. **Single Responsibility Principle (SRP)**
   - Each service handles one domain (memorials, authentication, admin)
   - Separated into layers: domain → repository → service → handler
   - Example: MemorialRepository handles only data access for memorials

2. **Open/Closed Principle (OCP)**
   - Services accept interface types, not concrete implementations
   - Easy to extend with new repository implementations
   - Example: `MemorialService` depends on `MemorialRepository` interface

3. **Liskov Substitution Principle (LSP)**
   - Repositories implement consistent interfaces
   - Services can work with any memory, database, or cache backend
   - Error handling consistent across all repository implementations

4. **Interface Segregation Principle (ISP)**
   - Clients depend on specific interfaces, not bulky ones
   - `MemorialRepository` interface only exposes memorial operations
   - Handlers only receive dependencies they need

5. **Dependency Inversion Principle (DIP)**
   - High-level modules depend on abstractions (interfaces)
   - Low-level modules (PostgreSQL, Redis) implement those abstractions
   - Configuration injects concrete implementations at startup

### Clean Architecture Layers

```
┌─────────────────────────────────┐
│    HTTP Handlers / GraphQL       │  (Controllers)
│    REST Routes, Request/Response │
└────────────────┬────────────────┘
                 │ depends on
┌────────────────▼────────────────┐
│    Service/Business Logic        │  (Use Cases)
│    Orchestration, Validation     │
└────────────────┬────────────────┘
                 │ depends on
┌────────────────▼────────────────┐
│    Repository/Data Abstraction   │  (Interfaces)
│    Database Operations, Queries  │
└────────────────┬────────────────┘
                 │ implements
┌────────────────▼────────────────┐
│    Domain Models / Entities      │  (Core)
│    Business Rules, Relationships │
└─────────────────────────────────┘
```

## Service Architecture

### Memorial Service (Go gRPC + REST)

**Port:** 5001 (REST) / 50001 (gRPC)

**Responsibilities:**

- Manage memorial entries (CRUD)
- Handle memorial media (photos, videos)
- Publish memorial events
- Expose gRPC and REST APIs

**Key Components:**

```
memorial-service/
├── cmd/main.go              # Entry point, server initialization
├── pkg/
│   ├── db/                  # Connection managers
│   │   ├── postgres.go      # PostgreSQL setup
│   │   ├── redis.go         # Redis caching
│   │   └── kafka.go         # Kafka producer
│   ├── domain/              # Domain models
│   │   └── memorial.go      # Memorial, Media entities
│   ├── repository/          # Data access layer
│   │   ├── memorial.go      # Memorial queries
│   │   ├── media.go         # Media queries
│   │   └── interfaces.go    # Repository contracts
│   ├── service/             # Business logic
│   │   ├── memorial.go      # Memorial service
│   │   └── interfaces.go    # Service contracts
│   └── handler/             # HTTP handlers
│       └── http.go          # REST endpoints
└── Dockerfile               # Multi-stage build
```

**Data Flow:**

```
HTTP Request → Handler → Service → Repository → Database
                              ↓
                          Cache (Redis)
                              ↓
                          Events (Kafka)
```

### Admin Service (FastAPI Python)

**Port:** 5002

**Responsibilities:**

- User authentication (JWT)
- Admin operations (user/memorial management)
- Analytics and logging
- REST API only

**Key Components:**

```
admin-service/
├── app/
│   ├── main.py              # FastAPI app initialization
│   ├── core/
│   │   ├── config.py        # Environment configuration
│   │   ├── security.py      # JWT, password hashing
│   │   ├── logger.py        # Structured logging
│   │   └── dependencies.py  # Dependency injection
│   ├── db/
│   │   └── database.py      # SQLAlchemy models
│   └── api/routes/
│       ├── auth.py          # Login/register
│       ├── admin.py         # Admin operations
│       ├── users.py         # User management
│       ├── memorials.py     # Memorial management
│       └── analytics.py     # Analytics endpoints
└── Dockerfile               # Multi-stage build
```

**Authentication Flow:**

```
POST /login → Security.verify_password() → JWT.create_token()
   ↓
POST /protected → JWT.verify_token() → current_user dependency
   ↓
Authorization check → Business logic → Response
```

### Frontend (Next.js SSR)

**Port:** 3000

**Responsibilities:**

- Public memorial site
- Server-side rendering (SEO)
- Client-side interactivity
- Memorial search and browsing

**Features:**

- Static generation with ISR (Incremental Static Regeneration)
- Image optimization
- Security headers
- TypeScript for type safety

### Admin Panel (Vue 3)

**Port:** 8080

**Responsibilities:**

- Dashboard and admin interface
- User and memorial management
- Analytics visualization
- Authentication

**State Management:**

```
Pinia Store
├── Auth Store
│   ├── currentUser
│   ├── token
│   └── login/logout methods
└── UI Store
    ├── sidebarOpen
    ├── notifications
    └── toggle methods
```

## Database Schema

### Core Tables

**users**

- id (UUID)
- email (unique)
- username (unique)
- full_name
- hashed_password
- is_active
- is_admin
- created_at, updated_at, deleted_at (soft delete)

**memorials**

- id (UUID)
- name
- date_of_birth
- date_of_death
- biography (TEXT)
- created_by (FK → users)
- status (active/archived)
- created_at, updated_at, deleted_at (soft delete)

**memorial_media**

- id (UUID)
- memorial_id (FK → memorials)
- url (S3/cloud storage)
- media_type (photo/video/document)
- description
- created_at

**comments**

- id (UUID)
- memorial_id (FK → memorials)
- user_id (FK → users)
- content
- created_at, updated_at, deleted_at (soft delete)

**admin_logs**

- id (UUID)
- user_id (FK → users)
- action (create/update/delete)
- resource_type
- resource_id
- ip_address
- created_at

**analytics_events**

- id (UUID)
- event_type (view/search/share)
- user_id (FK → users, nullable)
- data (JSON)
- created_at

## Communication Patterns

### Synchronous Communication

**Between Services:**

- Frontend ↔ Memorial Service (REST)
- Frontend ↔ Admin Service (REST)
- Admin Panel ↔ Admin Service (REST)
- Admin Service → Memorial Service (REST for cross-service calls)

**Within Services:**

- Handlers call Services
- Services call Repositories
- Repositories query Database/Cache

### Asynchronous Communication

**Kafka Topics:**

- `memorial-events`: memorial.created, memorial.updated, memorial.deleted
- `analytics-events`: user.viewed, user.searched, user.shared
- `admin-events`: user.created, user.deleted, policy.updated

**Consumers:**

- Admin Service listens to memorial events for audit logging
- Analytics service processes events for dashboards
- Notification service (future) sends user notifications

## Deployment Architecture

### Kubernetes Structure

```
mortalbook namespace:
├── Deployments
│   ├── memorial-service (2 replicas)
│   ├── admin-service (2 replicas)
│   ├── frontend (2 replicas)
│   ├── admin-panel (1 replica)
│   ├── postgres (1 replica)
│   ├── redis (1 replica)
│   ├── kafka (1 replica)
│   └── zookeeper (1 replica)
├── Services
│   ├── memorial-service (ClusterIP:5001/50001)
│   ├── admin-service (ClusterIP:5002)
│   ├── frontend (ClusterIP:3000)
│   ├── admin-panel (ClusterIP:8080)
│   ├── postgres (ClusterIP:5432)
│   ├── redis (ClusterIP:6379)
│   ├── kafka (ClusterIP:9092)
│   └── zookeeper (ClusterIP:2181)
├── Ingress
│   └── mortalbook-ingress → frontend/admin/api routing
├── ConfigMaps
│   └── app-config (shared configuration)
├── Secrets
│   └── app-secrets (passwords, keys)
├── PersistentVolumeClaims
│   ├── postgres-pvc (10Gi)
│   └── redis-pvc (5Gi)
└── NetworkPolicies
    └── allow-traffic (namespace isolation)
```

### Resource Limits

**Services:**

```
memorial-service:
  requests: cpu=100m, memory=128Mi
  limits: cpu=500m, memory=512Mi

admin-service:
  requests: cpu=100m, memory=128Mi
  limits: cpu=500m, memory=512Mi

frontend:
  requests: cpu=100m, memory=256Mi
  limits: cpu=500m, memory=1Gi

admin-panel:
  requests: cpu=100m, memory=256Mi
  limits: cpu=500m, memory=1Gi
```

**Infrastructure:**

```
postgres:
  requests: cpu=250m, memory=256Mi
  limits: cpu=1000m, memory=1Gi

redis:
  requests: cpu=100m, memory=128Mi
  limits: cpu=500m, memory=512Mi

kafka:
  requests: cpu=250m, memory=512Mi
  limits: cpu=1000m, memory=1Gi
```

## Security Architecture

### Authentication & Authorization

1. **Frontend Login**
   - User credentials → Admin Service
   - JWT token returned
   - Token stored in localStorage

2. **Protected Routes**
   - Frontend: Vue Router checks isAuthenticated()
   - Backend: Auth dependency checks JWT validity

3. **Role-Based Access Control (RBAC)**
   - is_admin flag in users table
   - Admin routes check is_admin before processing
   - Audit logs track admin actions

### Data Protection

- **Passwords**: Bcrypt hashing (cost=12)
- **JWT Tokens**: HS256 (symmetric), 24-hour expiration
- **Database**: Encrypted connections only
- **HTTPS**: TLS certificates via cert-manager
- **CORS**: Configured per environment

### Network Security

- **NetworkPolicy**: Restrict traffic to mortalbook namespace
- **Ingress TLS**: Automatic cert provisioning
- **Service Mesh**: Optional mTLS between services
- **Pod Security**: Non-root containers, read-only filesystems

## Monitoring & Observability

### Logging

- **Go**: Structured JSON with zap logger
- **Python**: JSON format via python-jsonlogger
- **Frontend**: Console logs with error tracking (optional Sentry)

**Log Fields:**

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "service": "memorial-service",
  "request_id": "uuid",
  "user_id": "uuid",
  "action": "memorial.created",
  "duration_ms": 125,
  "status": 201,
  "message": "Memorial created successfully"
}
```

### Metrics

**Prometheus metrics exported:**

- HTTP request count/latency (by endpoint)
- Database query duration
- Cache hit/miss ratio
- Kafka producer/consumer lag
- Active connections

### Health Checks

**Kubernetes probes:**

```
Liveness probe:  /health (every 10s, timeout 5s)
Readiness probe: /ready (every 5s, timeout 5s)
```

## Deployment Strategies

### Development

- `docker-compose up` orchestrates all services locally
- Hot reload enabled for code changes
- In-memory or local database (SQLite optional)

### Staging

- GitHub Actions on `develop` branch push
- Builds and pushes Docker images
- Deploys to staging cluster via `kubectl apply`
- Runs smoke tests automatically

### Production

- Manual trigger via GitHub Actions
- Blue-green deployment strategy
- Health checks before marking ready
- Automatic rollback on failure
- Zero-downtime updates

## Performance Considerations

### Caching Strategy

- **Redis**: Cache frequent queries (memorials list, user profiles)
- **TTL**: 5 minutes for aggregate data, 1 hour for static content
- **Cache invalidation**: On create/update events

### Database Optimization

- **Connection pooling**: 10 open, 20 overflow connections
- **Indexes**: On frequently queried fields (status, created_at, user_id)
- **Pagination**: Limit 50 records per page

### Frontend Optimization

- **SSR**: Next.js generates static pages for memorials
- **Image optimization**: Next/Image component with CDN
- **Code splitting**: Route-based chunks with lazy loading

## Disaster Recovery

### Backup Strategy

- PostgreSQL: Daily snapshots to S3
- Redis: RDB persistence (appendonly.aof)
- Kafka: Retention policy 7 days minimum

### Failure Scenarios

- **Database down**: ReadOnly replica queries, queue writes
- **Service down**: Kubernetes auto-restarts failed pods
- **Network partition**: Graceful degradation, cached responses
- **Data corruption**: Point-in-time recovery from backups

## Future Enhancements

1. **Service Mesh (Istio)**
   - Traffic management and security
   - Circuit breaking and retries
   - Distributed tracing

2. **GraphQL Layer**
   - Single endpoint for flexible queries
   - Reduce over-fetching and under-fetching

3. **Elasticsearch**
   - Full-text memorial search
   - Analytics dashboards

4. **gRPC Web**
   - Browser clients calling gRPC services directly

5. **Message Queue**
   - Replace Kafka with multi-tenancy support

6. **Machine Learning**
   - Recommendation engine
   - Content moderation
   - Sentiment analysis on comments
