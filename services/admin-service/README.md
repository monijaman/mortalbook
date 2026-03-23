# Admin Service (FastAPI)

A production-ready Python FastAPI microservice for admin panel backend. Provides authentication, user management, and administrative features.

## 🏗️ Architecture

### Project Structure

```
admin-service/
├── app/
│   ├── main.py              # FastAPI application entry
│   ├── core/
│   │   ├── config.py        # Configuration (12-factor)
│   │   ├── security.py      # Password hashing, JWT tokens
│   │   ├── logger.py        # Structured logging
│   │   └── dependencies.py   # Dependency injection
│   ├── api/
│   │   └── routes/
│   │       ├── auth.py      # Authentication endpoints
│   │       ├── admin.py     # Admin management
│   │       ├── users.py     # User management
│   │       ├── memorials.py # Memorial management
│   │       └── analytics.py # Analytics endpoints
│   └── db/
│       └── database.py      # SQLAlchemy models & ORM
├── tests/                   # Test suite
├── k8s/                     # Kubernetes manifests
│   ├── deployment.yaml
│   └── service.yaml
├── Dockerfile               # Multi-stage build
├── requirements.txt         # Python dependencies
└── README.md
```

### Design Patterns

- **Dependency Injection**: FastAPI Depends() for DI
- **Clean Architecture**: Layered structure (routes, services, models)
- **12-Factor App**: Environment-based configuration
- **Repository Pattern**: Database abstraction with SQLAlchemy
- **JWT Authentication**: Token-based security

## 🚀 Getting Started

### Prerequisites

- Python 3.11+
- PostgreSQL 16+
- Redis 7+
- Kafka 7.5+ (optional)

### Local Development

```bash
# Create virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Create .env file
cp .env.example .env

# Initialize database
python -m app.db.database

# Run application
uvicorn app.main:app --reload --host 0.0.0.0 --port 5002
```

### Environment Variables

```bash
# Server
PORT=5002
APP_ENV=development
DEBUG=True

# Database
DB_HOST=localhost
DB_PORT=5432
DB_NAME=mortalbook_db
DB_USER=postgres
DB_PASSWORD=postgres

# Cache
REDIS_HOST=localhost
REDIS_PORT=6379

# Security
JWT_SECRET=your-secret-key-here
ADMIN_SERVICE_SECRET_KEY=your-admin-secret

# Logging
LOG_LEVEL=debug
```

## 📡 API Endpoints

### Authentication

```http
POST   /api/v1/auth/login          # User login
POST   /api/v1/auth/register       # User registration
POST   /api/v1/auth/refresh        # Refresh token
```

### Admin Operations

```http
GET    /api/v1/admin/users          # List users (admin)
DELETE /api/v1/admin/users/:id      # Delete user (admin)
GET    /api/v1/admin/memorials      # List memorials (admin)
GET    /api/v1/admin/logs           # Get admin logs (admin)
```

### User Management

```http
GET    /api/v1/users/me             # Get current user profile
PUT    /api/v1/users/me             # Update user profile
```

### Analytics

```http
GET    /api/v1/analytics/dashboard  # Analytics dashboard
GET    /api/v1/analytics/events     # Analytics events
```

### Request/Response Examples

**User Login**

```bash
curl -X POST http://localhost:5002/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "securepassword"
  }'
```

Response:

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "bearer",
  "expires_in": 3600
}
```

**Register New User**

```bash
curl -X POST http://localhost:5002/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "username": "newuser",
    "password": "securepassword",
    "full_name": "John Doe"
  }'
```

## 🗄️ Database Schema

### Users Table

```sql
CREATE TABLE users (
  id VARCHAR(36) PRIMARY KEY,
  email VARCHAR(255) UNIQUE NOT NULL,
  username VARCHAR(255) UNIQUE NOT NULL,
  full_name VARCHAR(255),
  hashed_password VARCHAR(255) NOT NULL,
  is_active BOOLEAN DEFAULT TRUE,
  is_admin BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Admin Logs Table

```sql
CREATE TABLE admin_logs (
  id VARCHAR(36) PRIMARY KEY,
  user_id VARCHAR(36) NOT NULL,
  action VARCHAR(255) NOT NULL,
  resource_type VARCHAR(255),
  resource_id VARCHAR(36),
  description TEXT,
  ip_address VARCHAR(45),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## 🔐 Security

### Authentication

- **JWT Tokens**: Access tokens valid for 24 hours
- **Refresh Tokens**: Valid for 7 days
- **Password Hashing**: bcrypt with secure rounds
- **Bearer Scheme**: Standard Authorization header

### Authorization

- **Role-Based Access**: Admin vs Regular User
- **Dependency Injection**: Protected routes with Depends()
- **Token Validation**: Signature and expiration verification

### Best Practices

- Environment-based secrets (never in code)
- HTTPS in production
- Rate limiting (implement in production)
- CORS configuration
- Input validation with Pydantic

## 🐳 Docker

### Build Image

```bash
docker build -t mortalbook/admin-service:latest .
```

### Run Container

```bash
docker run -p 5002:5002 \
  -e DB_HOST=postgres \
  -e REDIS_HOST=redis \
  -e JWT_SECRET=your-secret \
  mortalbook/admin-service:latest
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
kubectl scale deployment admin-service --replicas=3
```

## 🧪 Testing

### Run Tests

```bash
# Unit tests
pytest tests/

# With coverage
pytest --cov=app tests/

# Specific test
pytest tests/test_auth.py::test_login
```

### Test Structure

```
tests/
├── test_auth.py         # Authentication tests
├── test_admin.py        # Admin management tests
└── test_users.py        # User management tests
```

## 📊 Features

### ✅ Implemented

- [x] User registration and login
- [x] JWT token-based authentication
- [x] Admin role management
- [x] User management (list, delete)
- [x] Admin action logging
- [x] Analytics tracking
- [x] Health checks
- [x] Structured logging
- [x] Database models
- [x] CORS support

### 🔄 Planned Enhancements

- [ ] Email verification
- [ ] Password reset functionality
- [ ] Two-factor authentication
- [ ] Rate limiting
- [ ] API key authentication
- [ ] Audit trails with detailed tracking
- [ ] Real-time notifications
- [ ] Export functionality

## 📈 Performance

### Optimization

- **Connection Pooling**: SQLAlchemy with pool_size=10
- **Query Caching**: Redis integration (to implement)
- **Async Support**: FastAPI async handlers
- **Pagination**: Default limit and offset

### Load Testing

```bash
# Using Apache Bench
ab -n 1000 -c 10 http://localhost:5002/api/v1/admin/users

# Using hey
hey -n 1000 -c 10 http://localhost:5002/health
```

## 🔧 Troubleshooting

### Database Connection Failed

```bash
# Check PostgreSQL is running
psql -h localhost -U postgres

# Verify connection string
echo $DB_URL
```

### Import Errors

```bash
# Reinstall dependencies
pip install -r requirements.txt --force-reinstall
```

### Port Already in Use

```bash
# Change port in environment or code
export PORT=5003
uvicorn app.main:app --port 5003
```

## 📚 Dependencies

- **fastapi** - Web framework
- **uvicorn** - ASGI server
- **sqlalchemy** - ORM
- **pydantic** - Data validation
- **python-jose** - JWT tokens
- **passlib** - Password hashing
- **redis** - Caching
- **confluent-kafka** - Kafka integration

## 📄 License

MIT

---

**Built with ❤️ for the Mortalbook project**
