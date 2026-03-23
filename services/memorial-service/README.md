# Memorial Service (Go gRPC)

A production-ready Go microservice for managing memorial entries. Implements gRPC and REST APIs with clean architecture principles.

## 🏗️ Architecture

### Project Structure

```
memorial-service/
├── cmd/
│   └── main.go              # Application entry point
├── pkg/
│   ├── db/                  # Database & infrastructure connections
│   │   ├── postgres.go      # PostgreSQL connection
│   │   ├── redis.go         # Redis connection
│   │   └── kafka.go         # Kafka connection
│   ├── domain/              # Domain models (SOLID)
│   │   └── memorial.go      # Memorial entity
│   ├── repository/          # Data access layer
│   │   ├── memorial.go      # Memorial repository
│   │   └── media.go         # Media repository
│   ├── service/             # Business logic (clean architecture)
│   │   └── memorial.go      # Memorial service
│   └── handler/             # HTTP/gRPC handlers
│       └── http.go          # REST API handlers
├── k8s/                     # Kubernetes manifests
│   ├── deployment.yaml
│   └── service.yaml
├── Dockerfile               # Multi-stage build
├── go.mod & go.sum         # Dependencies
└── README.md
```

### Design Patterns

- **Clean Architecture**: Separation of concerns (domain, repository, service, handler)
- **Dependency Injection**: Injected through constructors
- **Interface Segregation**: Repository pattern with interfaces
- **Single Responsibility**: Each package has one reason to change

## 🚀 Getting Started

### Prerequisites

- Go 1.21+
- PostgreSQL 16+
- Redis 7+
- Kafka 7.5+

### Local Development

```bash
# Install dependencies
go mod download

# Run application
go run ./cmd/main.go

# Run tests
go test ./...

# Build binary
go build -o memorial-service ./cmd/main.go
```

### Environment Variables

```bash
# Server
PORT=5001
GRPC_PORT=50001

# Database
DB_HOST=localhost
DB_PORT=5432
DB_NAME=mortalbook_db
DB_USER=postgres
DB_PASSWORD=postgres

# Cache
REDIS_HOST=localhost
REDIS_PORT=6379

# Message Queue
KAFKA_BROKERS=localhost:9092

# Logging
LOG_LEVEL=debug
```

## 📡 API Endpoints

### REST API

```http
GET    /api/v1/memorials              # List all memorials
GET    /api/v1/memorials/:id          # Get memorial details
POST   /api/v1/memorials              # Create memorial
PUT    /api/v1/memorials/:id          # Update memorial
DELETE /api/v1/memorials/:id          # Delete memorial
GET    /api/v1/memorials/:id/media    # Get memorial media
GET    /health                        # Health check
```

### Request/Response Examples

**List Memorials**

```bash
curl http://localhost:5001/api/v1/memorials
```

**Create Memorial**

```bash
curl -X POST http://localhost:5001/api/v1/memorials \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "biography": "A beloved father and friend",
    "date_of_birth": "1950-01-15",
    "date_of_death": "2024-01-10"
  }'
```

**Get Memorial**

```bash
curl http://localhost:5001/api/v1/memorials/{id}
```

## 🗄️ Database Schema

### Memorials Table

```sql
CREATE TABLE memorials (
  id VARCHAR(36) PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  date_of_birth DATE NOT NULL,
  date_of_death DATE NOT NULL,
  biography TEXT,
  created_by VARCHAR(36),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  status VARCHAR(50) DEFAULT 'active',
  deleted_at TIMESTAMP
);

CREATE INDEX idx_memorials_status ON memorials(status);
CREATE INDEX idx_memorials_created_at ON memorials(created_at DESC);
```

### Memorial Media Table

```sql
CREATE TABLE memorial_media (
  id VARCHAR(36) PRIMARY KEY,
  memorial_id VARCHAR(36) NOT NULL,
  url VARCHAR(2048) NOT NULL,
  media_type VARCHAR(50), -- photo, video, document
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (memorial_id) REFERENCES memorials(id) ON DELETE CASCADE
);

CREATE INDEX idx_memorial_media_memorial_id ON memorial_media(memorial_id);
```

## 🔄 Service Layer

The service layer implements business logic and coordinates between repositories and handlers.

### MemorialService Interface

```go
type MemorialService interface {
    GetMemorial(ctx context.Context, id string) (*domain.Memorial, error)
    ListMemorials(ctx context.Context, filter *domain.MemorialFilter) ([]*domain.Memorial, int64, error)
    CreateMemorial(ctx context.Context, memorial *domain.Memorial) (string, error)
    UpdateMemorial(ctx context.Context, memorial *domain.Memorial) error
    DeleteMemorial(ctx context.Context, id string) error
    GetMemorialMedia(ctx context.Context, memorialID string) ([]*domain.MemorialMedia, error)
}
```

## 📊 Features

### ✅ Implemented

- [x] RESTful API endpoints
- [x] PostgreSQL integration
- [x] Redis caching layer
- [x] Kafka event publishing
- [x] Health checks
- [x] Structured logging (zap)
- [x] Context-based request handling
- [x] Error handling
- [x] CORS support
- [x] Database connection pooling

### 🔄 Event Publishing

The service publishes events to Kafka for:

- `memorial.created` - When a new memorial is created
- `memorial.updated` - When a memorial is updated
- `memorial.deleted` - When a memorial is deleted

### 🔐 Security

- JWT authentication headers (X-User-ID)
- CORS policies
- Input validation
- SQL injection prevention (parameterized queries)

## 🐳 Docker

### Build Image

```bash
docker build -t mortalbook/memorial-service:latest .
```

### Run Container

```bash
docker run -p 5001:5001 -p 50001:50001 \
  -e DB_HOST=postgres \
  -e REDIS_HOST=redis \
  -e KAFKA_BROKERS=kafka:9092 \
  mortalbook/memorial-service:latest
```

## 🛠️ Kubernetes Deployment

### Prerequisites

- kubectl configured
- Kubernetes cluster running

### Deploy

```bash
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# Verify
kubectl get deployments
kubectl get pods
kubectl get services
```

### Scale Service

```bash
kubectl scale deployment memorial-service --replicas=3
```

## 📈 Performance

### Optimization Techniques

- **Database Connection Pooling**: Max 10 open connections
- **Redis Caching**: Frequently accessed memorials cached
- **Query Optimization**: Proper indexing on frequently queried columns
- **Pagination**: Default limit of 20 results with offset
- **Lazy Loading**: Media loaded separately from memorials

### Load Testing

```bash
# Using Apache Bench
ab -n 1000 -c 10 http://localhost:5001/api/v1/memorials

# Using hey
hey -n 1000 -c 10 http://localhost:5001/api/v1/memorials
```

## 🧪 Testing

### Unit Tests

```bash
go test -v ./pkg/...
```

### Integration Tests

```bash
go test -v -tags=integration ./tests/...
```

### Test Coverage

```bash
go test -cover ./...
```

## 🔧 Troubleshooting

### Connection Issues

```bash
# Check PostgreSQL connection
psql -h localhost -U postgres -d mortalbook_db

# Check Redis connection
redis-cli ping

# Check Kafka brokers
kafka-broker-api-versions.sh --bootstrap-server localhost:9092
```

### Logs

```bash
# View application logs
docker logs mortalbook-memorial-service

# For Kubernetes
kubectl logs deployment/memorial-service -f
```

## 📚 Dependencies

- `google.golang.org/grpc` - gRPC framework
- `google.golang.org/protobuf` - Protocol Buffers
- `github.com/lib/pq` - PostgreSQL driver
- `github.com/redis/go-redis` - Redis client
- `github.com/segmentio/kafka-go` - Kafka client
- `go.uber.org/zap` - Structured logging
- `github.com/google/uuid` - UUID generation

## 🎯 Future Enhancements

- [ ] Implement gRPC service definitions
- [ ] Add GraphQL resolver
- [ ] Implement caching strategies
- [ ] Add metrics collection (Prometheus)
- [ ] Implement circuit breaker pattern
- [ ] Add comprehensive integration tests
- [ ] Implement audit logging
- [ ] Add request validation middleware

## 📄 License

MIT

---

**Built with ❤️ for the Mortalbook project**
