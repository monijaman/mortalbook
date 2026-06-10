# Development Guide

Setup and guidelines for local Mortalbook development.

## Prerequisites

- **Go** 1.21+
- **Python** 3.11+
- **Node.js** 18+
- **Docker** and **Docker Compose**
- **PostgreSQL** 16 (or use Docker)
- **Redis** 7 (or use Docker)
- **Git**

## Quick Start

### 1. Clone Repository

```bash
git clone https://github.com/your-org/mortalbook.git
cd mortalbook
```

### 2. Environment Setup

```bash
# Copy environment template
cp .env.example .env

# Edit .env with local values (defaults work for docker-compose)
nano .env
```

### 3. Start Services with Docker Compose

```bash
# Build all services
docker-compose build

# Start all services
docker-compose up -d

# Initialize database
docker-compose exec postgres psql -U postgres -d mortalbook_db < infrastructure/postgres/init.sql

# View logs
docker-compose logs -f
```

### 4. Verify Setup

```bash
# Test Memorial Service
curl http://localhost:5001/health

# Test Admin Service
curl http://localhost:5002/health

# Open services in browser
# Frontend: http://localhost:3000
# Admin Panel: http://localhost:8080
```

## Local Development (Without Docker)

### Setup PostgreSQL

```bash
# On macOS with Homebrew
brew install postgresql@16
brew services start postgresql@16

# Create database
createdb mortalbook_db

# Initialize schema
psql mortalbook_db < infrastructure/postgres/init.sql
```

### Setup Redis

```bash
# On macOS with Homebrew
brew install redis
brew services start redis

# Verify connection
redis-cli ping
```

### Setup Kafka (Optional)

For local development, Kafka is optional. To set it up locally:

```bash
# Download and extract Kafka
wget https://archive.apache.org/dist/kafka/7.5.0/kafka_2.13-7.5.0.tgz
tar -xzf kafka_2.13-7.5.0.tgz
cd kafka_2.13-7.5.0

# Format the KRaft storage directory once
bin/kafka-storage.sh random-uuid
bin/kafka-storage.sh format -t <CLUSTER_ID> -c config/kraft/server.properties

# Start Kafka in KRaft mode
bin/kafka-server-start.sh config/kraft/server.properties &
```

## Memorial Service Development

### Setup

```bash
cd services/memorial-service

# Download dependencies
go mod download

# Install development tools
go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
```

### Run Locally

```bash
# Set environment variables
export APP_ENV=development
export DB_HOST=localhost
export DB_PORT=5432
export REDIS_HOST=localhost
export REDIS_PORT=6379
export KAFKA_BROKERS=localhost:9092

# Run service
go run cmd/main.go

# Service starts on:
# - REST API: http://localhost:5001
# - gRPC: localhost:50001
```

### Development Commands

```bash
# Run tests
go test ./...

# Run tests with coverage
go test -cover ./...

# Generate coverage report
go test -coverprofile=coverage.out ./...
go tool cover -html=coverage.out

# Lint code
golangci-lint run

# Format code
go fmt ./...

# Tidy dependencies
go mod tidy

# Build binary
go build -o bin/memorial-service cmd/main.go
```

### Debugging

```bash
# Install delve debugger
go install github.com/go-delve/delve/cmd/dlv@latest

# Debug application
dlv debug cmd/main.go

# In debugger prompt
(dlv) break main.main
(dlv) continue
(dlv) next
(dlv) print variable_name
(dlv) quit
```

## Admin Service Development

### Setup

```bash
cd services/admin-service

# Create virtual environment
python -m venv venv

# Activate virtual environment
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Install dev dependencies
pip install pytest pytest-cov black flake8 isort
```

### Run Locally

```bash
# Set environment variables
export APP_ENV=development
export DB_HOST=localhost
export DB_USER=postgres
export DB_PASSWORD=postgres
export REDIS_HOST=localhost
export JWT_SECRET=your-secret-key-for-dev

# Run service with uvicorn
uvicorn app.main:app --reload --host 0.0.0.0 --port 5002

# Service starts on: http://localhost:5002
# Auto API docs: http://localhost:5002/docs
# ReDoc: http://localhost:5002/redoc
```

### Development Commands

```bash
# Run tests
pytest

# Run tests with coverage
pytest --cov=app

# Format code
black app/

# Check imports
isort app/

# Lint code
flake8 app/ --count --select=E9,F63,F7,F82 --show-source --statistics
```

### Interactive API Testing

FastAPI provides interactive API docs:

```
http://localhost:5002/docs  (Swagger UI)
http://localhost:5002/redoc (ReDoc)
```

Click "Try it out" to test endpoints directly in browser.

## Frontend Development

### Setup

```bash
cd services/frontend

# Install dependencies
npm install

# Set environment variables
export NEXT_PUBLIC_API_URL=http://localhost:5001
export NEXT_PUBLIC_ADMIN_API_URL=http://localhost:5002
```

### Run Locally

```bash
# Development server with hot reload
npm run dev

# Open http://localhost:3000
```

### Development Commands

```bash
# Build for production
npm run build

# Start production build locally
npm start

# Run linter
npm run lint

# Format code
npm run format

# Type checking
npm run type-check
```

### Next.js Features

- **Hot Module Replacement (HMR)**: Changes auto-reload
- **API Routes**: Create backend routes in `pages/api/`
- **Image Optimization**: Use `next/image` component
- **TypeScript**: Full type support
- **Tailwind CSS**: Utility CSS framework

## Admin Panel Development

### Setup

```bash
cd services/admin-panel

# Install dependencies
npm install

# Set environment variables
export VITE_API_URL=http://localhost:5002
```

### Run Locally

```bash
# Development server with hot reload
npm run dev

# Open http://localhost:5173
```

### Development Commands

```bash
# Build for production
npm run build

# Preview production build
npm run preview

# Run linter
npm run lint

# Format code
npm run format
```

### Vue 3 with Vite

- **Hot Module Replacement**: Instant component updates
- **Vite Dev Server**: Ultra-fast startup
- **Composition API**: Modern Vue syntax
- **TypeScript**: Optional type support
- **Pinia State Management**: Centralized store

## Database Development

### Schema Migrations

For production deployments, use migration tools:

```bash
# Using golang-migrate (Go services)
migrate -path infrastructure/postgres/migrations -database "postgres://..." up

# Using Alembic (Python services)
cd services/admin-service
alembic revision --autogenerate -m "Add new table"
alembic upgrade head
```

For now, use the init.sql script:

```bash
psql mortalbook_db < infrastructure/postgres/init.sql
```

### Database Inspection

```bash
# Connect to database
psql mortalbook_db

# List tables
\dt

# Describe table
\d memorials

# View data
SELECT * FROM memorials LIMIT 10;

# Count records
SELECT COUNT(*) FROM memorials;
```

## Testing Strategy

### Unit Tests

**Go:**

```go
func TestMemorialRepository_GetByID(t *testing.T) {
    repo := NewMemorialRepository(db)
    memorial, err := repo.GetByID(context.Background(), "test-id")
    assert.NoError(t, err)
    assert.Equal(t, "test-id", memorial.ID)
}
```

**Python:**

```python
def test_create_user():
    user_data = {"email": "test@test.com", "password": "pass123"}
    response = client.post("/auth/register", json=user_data)
    assert response.status_code == 201
```

### Integration Tests

Test services with real databases:

```bash
# Go integration tests
go test -tags=integration ./...

# Python integration tests
pytest tests/integration/
```

### API Testing

Use tools like Postman, Insomnia, or REST Client:

```http
### Get memorials
GET http://localhost:5001/api/v1/memorials HTTP/1.1

### Create memorial
POST http://localhost:5001/api/v1/memorials HTTP/1.1
Authorization: Bearer {{token}}
Content-Type: application/json

{
  "name": "Test Person",
  "date_of_birth": "1950-01-01",
  "date_of_death": "2024-01-10",
  "biography": "Test biography"
}
```

## Code Style

### Go

- Follow [Effective Go](https://golang.org/doc/effective_go)
- Use `golangci-lint` for linting
- Run `gofmt` before committing
- Naming: camelCase for functions, PascalCase for exports

### Python

- Follow [PEP 8](https://www.python.org/dev/peps/pep-0008/)
- Use [Black](https://github.com/psf/black) for formatting
- Use [isort](https://github.com/PyCPA/isort) for imports
- Type hints with Python 3.11+

### JavaScript/TypeScript

- Follow [Airbnb JavaScript Style Guide](https://github.com/airbnb/javascript)
- Use ESLint for linting
- Use Prettier for formatting
- TypeScript strict mode

### Vue

- Use Vue 3 Composition API
- Single-File Components (.vue)
- Scoped styles with `<style scoped>`
- Kebab-case for component names

## Debugging Tips

### Memorial Service (Go)

```go
// Add debug logging
log.Printf("Debug: %v", variable)

// Use debugger
import _ "net/http/pprof"
// http://localhost:6060/debug/pprof/
```

### Admin Service (Python)

```python
# Add debug logging
import logging
logger = logging.getLogger(__name__)
logger.debug(f"Debug: {variable}")

# Use pdb debugger
import pdb; pdb.set_trace()
```

### Frontend/Admin Panel

```javascript
// Browser console
console.log(variable);
console.table(data);

// Vue devtools (install chrome extension)
// Pinia devtools

// Network tab to inspect requests
// Storage tab to check localStorage
```

## Troubleshooting

### Service Won't Start

```bash
# Check if port is in use
lsof -i :5001  # Go
lsof -i :5002  # Python
lsof -i :3000  # Frontend

# Kill process
kill -9 <PID>

# Try different port
PORT=5003 go run cmd/main.go
```

### Database Connection Issues

```bash
# Test connection
psql -h localhost -U postgres -d mortalbook_db

# Check environment variables
echo $DB_HOST $DB_PORT $DB_USER

# Reset database
dropdb mortalbook_db
createdb mortalbook_db
psql mortalbook_db < infrastructure/postgres/init.sql
```

### Module/Dependency Issues

```bash
# Go: Clean cache
go clean -modcache

# Python: Reinstall packages
pip install --force-reinstall -r requirements.txt

# Node: Clear cache
npm cache clean --force
npm install
```

## IDE Setup

### VS Code Recommended Extensions

```json
{
  "extensions": [
    "golang.Go",
    "ms-python.python",
    "ms-python.vscode-pylance",
    "Vue.volar",
    "dbaeumer.vscode-eslint",
    "esbenp.prettier-vscode",
    "charliermarsh.ruff",
    "GitHub.copilot"
  ]
}
```

### VS Code Settings

```json
{
  "[go]": {
    "editor.defaultFormatter": "golang.go",
    "editor.formatOnSave": true
  },
  "[python]": {
    "editor.defaultFormatter": "ms-python.black-formatter",
    "editor.formatOnSave": true
  },
  "[vue]": {
    "editor.defaultFormatter": "Vue.volar"
  }
}
```

## Useful Commands

```bash
# View all service logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f memorial-service

# Restart service
docker-compose restart memorial-service

# Execute command in container
docker-compose exec memorial-service go test ./...

# Stop all services
docker-compose down

# Remove all data (clean start)
docker-compose down -v
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Additional Resources

- [Project Architecture](../docs/ARCHITECTURE.md)
- [API Documentation](../docs/API.md)
- [Deployment Guide](../docs/DEPLOYMENT.md)
- [Go Documentation](https://golang.org/doc/)
- [FastAPI Documentation](https://fastapi.tiangolo.com/)
- [Next.js Documentation](https://nextjs.org/docs)
- [Vue 3 Documentation](https://vuejs.org/)
