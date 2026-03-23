# API Documentation

Complete API reference for Mortalbook services.

## Base URLs

- **Production**: `https://api.mortalbook.com`
- **Staging**: `https://staging-api.mortalbook.com`
- **Development**: `http://localhost:5001` (memorial), `http://localhost:5002` (admin)

## Authentication

All protected endpoints require a JWT token in the Authorization header:

```
Authorization: Bearer <jwt_token>
```

### Obtain JWT Token

**Request:**

```http
POST /api/v1/auth/login HTTP/1.1
Host: api.mortalbook.com
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response:** 200 OK

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "bearer",
  "expires_in": 86400,
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "username",
    "is_admin": false
  }
}
```

**Response:** 401 Unauthorized

```json
{
  "detail": "Invalid credentials"
}
```

---

## Memorial Service API

Base URL: `/api/v1`

### Get All Memorials

**Request:**

```http
GET /memorials?page=1&limit=50&status=active HTTP/1.1
```

**Query Parameters:**

- `page` (int): Page number, default 1
- `limit` (int): Records per page, default 50, max 100
- `status` (string): Filter by status (active/archived)
- `search` (string): Search memorial names/biographies
- `sort` (string): Sort field (created_at, name, date_of_death)
- `order` (string): Sort order (asc, desc)

**Response:** 200 OK

```json
{
  "data": [
    {
      "id": "memorial-uuid",
      "name": "John Doe",
      "date_of_birth": "1950-01-15",
      "date_of_death": "2024-01-10",
      "biography": "John was a loving father...",
      "status": "active",
      "created_by": "user-uuid",
      "created_at": "2024-01-10T10:00:00Z",
      "updated_at": "2024-01-10T10:00:00Z",
      "media_count": 5
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 250,
    "total_pages": 5
  }
}
```

### Get Memorial by ID

**Request:**

```http
GET /memorials/{id} HTTP/1.1
```

**Response:** 200 OK

```json
{
  "id": "memorial-uuid",
  "name": "John Doe",
  "date_of_birth": "1950-01-15",
  "date_of_death": "2024-01-10",
  "biography": "John was a loving father...",
  "status": "active",
  "created_by": "user-uuid",
  "created_at": "2024-01-10T10:00:00Z",
  "updated_at": "2024-01-10T10:00:00Z",
  "media": [
    {
      "id": "media-uuid",
      "url": "https://cdn.mortalbook.com/memorial-uuid/photo1.jpg",
      "media_type": "photo",
      "description": "Family photo",
      "created_at": "2024-01-10T10:00:00Z"
    }
  ],
  "comments_count": 12
}
```

**Response:** 404 Not Found

```json
{
  "error": "Memorial not found"
}
```

### Create Memorial

**Request:**

```http
POST /memorials HTTP/1.1
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Jane Smith",
  "date_of_birth": "1960-03-20",
  "date_of_death": "2024-01-08",
  "biography": "Jane was a teacher and community volunteer..."
}
```

**Required Fields:**

- `name` (string, 1-255 chars)
- `date_of_birth` (date, YYYY-MM-DD)
- `date_of_death` (date, YYYY-MM-DD)

**Optional Fields:**

- `biography` (string, max 5000 chars)

**Response:** 201 Created

```json
{
  "id": "memorial-uuid",
  "name": "Jane Smith",
  "date_of_birth": "1960-03-20",
  "date_of_death": "2024-01-08",
  "biography": "Jane was a teacher...",
  "status": "active",
  "created_by": "user-uuid",
  "created_at": "2024-01-10T14:30:00Z",
  "updated_at": "2024-01-10T14:30:00Z"
}
```

**Response:** 400 Bad Request

```json
{
  "errors": {
    "name": "Name is required",
    "date_of_death": "Date of death must be after date of birth"
  }
}
```

### Update Memorial

**Request:**

```http
PUT /memorials/{id} HTTP/1.1
Authorization: Bearer <token>
Content-Type: application/json

{
  "biography": "Updated biography..."
}
```

**Response:** 200 OK

```json
{
  "id": "memorial-uuid",
  "name": "Jane Smith",
  "date_of_birth": "1960-03-20",
  "date_of_death": "2024-01-08",
  "biography": "Updated biography...",
  "status": "active",
  "created_by": "user-uuid",
  "created_at": "2024-01-10T14:30:00Z",
  "updated_at": "2024-01-10T14:35:00Z"
}
```

**Response:** 403 Forbidden

```json
{
  "error": "Only the creator or admin can update this memorial"
}
```

### Delete Memorial

**Request:**

```http
DELETE /memorials/{id} HTTP/1.1
Authorization: Bearer <token>
```

**Response:** 204 No Content

**Response:** 404 Not Found

```json
{
  "error": "Memorial not found"
}
```

---

## Media Management

### Upload Memorial Media

**Request:**

```http
POST /memorials/{memorial_id}/media HTTP/1.1
Authorization: Bearer <token>
Content-Type: multipart/form-data

file: <binary file>
media_type: photo
description: "Family photo from 1985"
```

**Supported Media Types:**

- photo (JPEG, PNG, WebP)
- video (MP4, WebM)
- document (PDF)

**Response:** 201 Created

```json
{
  "id": "media-uuid",
  "memorial_id": "memorial-uuid",
  "url": "https://cdn.mortalbook.com/memorial-uuid/photo-abc123.jpg",
  "media_type": "photo",
  "description": "Family photo from 1985",
  "created_at": "2024-01-10T15:00:00Z"
}
```

### Delete Media

**Request:**

```http
DELETE /memorials/{memorial_id}/media/{media_id} HTTP/1.1
Authorization: Bearer <token>
```

**Response:** 204 No Content

---

## Admin Service API

Base URL: `/api/v1`

### User Authentication

#### Register

**Request:**

```http
POST /auth/register HTTP/1.1
Content-Type: application/json

{
  "email": "user@example.com",
  "username": "username",
  "password": "SecurePassword123!",
  "full_name": "User Name"
}
```

**Response:** 201 Created

```json
{
  "id": "user-uuid",
  "email": "user@example.com",
  "username": "username",
  "full_name": "User Name",
  "is_admin": false,
  "is_active": true,
  "created_at": "2024-01-10T16:00:00Z"
}
```

**Response:** 400 Bad Request

```json
{
  "detail": "Email already registered"
}
```

#### Login

**Request:**

```http
POST /auth/login HTTP/1.1
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePassword123!"
}
```

**Response:** 200 OK

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "bearer",
  "expires_in": 86400,
  "user": {
    "id": "user-uuid",
    "email": "user@example.com",
    "username": "username",
    "is_admin": false
  }
}
```

### User Management (Admin Only)

#### Get All Users

**Request:**

```http
GET /admin/users?page=1&limit=50 HTTP/1.1
Authorization: Bearer <admin_token>
```

**Response:** 200 OK

```json
{
  "data": [
    {
      "id": "user-uuid",
      "email": "user@example.com",
      "username": "username",
      "full_name": "User Name",
      "is_admin": false,
      "is_active": true,
      "created_at": "2024-01-10T16:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 100,
    "total_pages": 2
  }
}
```

#### Delete User

**Request:**

```http
DELETE /admin/users/{user_id} HTTP/1.1
Authorization: Bearer <admin_token>
```

**Response:** 204 No Content

**Response:** 403 Forbidden

```json
{
  "detail": "Only administrators can delete users"
}
```

### Analytics API

#### Get Dashboard Stats

**Request:**

```http
GET /analytics/dashboard HTTP/1.1
Authorization: Bearer <token>
```

**Response:** 200 OK

```json
{
  "total_memorials": 1250,
  "total_views": 45678,
  "total_comments": 890,
  "new_memorials_today": 12,
  "active_users": 234
}
```

#### Get Analytics Events

**Request:**

```http
GET /analytics/events?event_type=view&period=7d HTTP/1.1
Authorization: Bearer <token>
```

**Query Parameters:**

- `event_type` (string): Event type (view, search, share, comment)
- `period` (string): Time period (24h, 7d, 30d, all)

**Response:** 200 OK

```json
{
  "events": [
    {
      "id": "event-uuid",
      "event_type": "view",
      "user_id": "user-uuid",
      "memorial_id": "memorial-uuid",
      "created_at": "2024-01-10T12:00:00Z"
    }
  ],
  "summary": {
    "total": 1234,
    "average_per_day": 176.29
  }
}
```

### Admin Logs

#### Get Admin Logs (Admin Only)

**Request:**

```http
GET /admin/logs?action=create&days=7 HTTP/1.1
Authorization: Bearer <admin_token>
```

**Query Parameters:**

- `action` (string): Action type (create, update, delete, login)
- `user_id` (string): Filter by user
- `days` (int): Last N days, default 7

**Response:** 200 OK

```json
{
  "logs": [
    {
      "id": "log-uuid",
      "user_id": "user-uuid",
      "action": "memorial.created",
      "resource_type": "memorial",
      "resource_id": "memorial-uuid",
      "ip_address": "192.168.1.1",
      "created_at": "2024-01-10T10:00:00Z"
    }
  ]
}
```

---

## Error Responses

### Common Error Codes

| Status | Code                | Message                                 |
| ------ | ------------------- | --------------------------------------- |
| 400    | INVALID_INPUT       | Invalid request parameters              |
| 401    | UNAUTHORIZED        | Missing or invalid authentication token |
| 403    | FORBIDDEN           | Insufficient permissions                |
| 404    | NOT_FOUND           | Resource not found                      |
| 409    | CONFLICT            | Duplicate resource or conflict          |
| 500    | INTERNAL_ERROR      | Server error                            |
| 503    | SERVICE_UNAVAILABLE | Service temporarily unavailable         |

### Error Response Format

```json
{
  "error": "Error type",
  "detail": "Detailed error message",
  "code": "ERROR_CODE",
  "request_id": "uuid",
  "timestamp": "2024-01-10T10:00:00Z"
}
```

---

## Rate Limiting

- **Unauthenticated**: 100 requests/hour per IP
- **Authenticated**: 1000 requests/hour per user
- **Admin**: 10000 requests/hour

Headers:

```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1704858000
```

---

## Pagination

All list endpoints support pagination:

- `page` (default: 1)
- `limit` (default: 50, max: 100)

Response includes:

```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 500,
    "total_pages": 10,
    "has_next": true,
    "has_prev": false
  }
}
```

---

## Filtering & Sorting

### Filtering

```
GET /memorials?status=active&search=john
```

### Sorting

```
GET /memorials?sort=created_at&order=desc
```

Supported sort fields:

- `created_at`
- `updated_at`
- `name`
- `date_of_death`

---

## Webhook Events

Services can subscribe to Kafka topics for event-driven updates:

### Memorial Events

- `memorial.created`
- `memorial.updated`
- `memorial.deleted`

### User Events

- `user.registered`
- `user.activated`
- `user.deleted`

### Analytics Events

- `memorial.viewed`
- `memorial.searched`
- `memorial.shared`
- `comment.created`

**Event Payload:**

```json
{
  "event_id": "uuid",
  "event_type": "memorial.created",
  "timestamp": "2024-01-10T10:00:00Z",
  "data": {
    "memorial_id": "uuid",
    "name": "John Doe",
    "created_by": "user-uuid"
  }
}
```

---

## Code Examples

### JavaScript/TypeScript

```typescript
const API_URL = process.env.REACT_APP_API_URL || "http://localhost:5001";

// Get memorials
async function getMemorials(page = 1, limit = 50) {
  const response = await fetch(
    `${API_URL}/api/v1/memorials?page=${page}&limit=${limit}`,
  );
  return response.json();
}

// Create memorial
async function createMemorial(token, memorial) {
  const response = await fetch(`${API_URL}/api/v1/memorials`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(memorial),
  });
  return response.json();
}

// Login
async function login(email, password) {
  const response = await fetch(`${API_URL}/api/v1/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ email, password }),
  });
  const data = await response.json();
  localStorage.setItem("token", data.access_token);
  return data;
}
```

### Python

```python
import requests

API_URL = os.getenv('API_URL', 'http://localhost:5001')

def get_memorials(page=1, limit=50):
    response = requests.get(
        f'{API_URL}/api/v1/memorials',
        params={'page': page, 'limit': limit}
    )
    return response.json()

def create_memorial(token, memorial):
    response = requests.post(
        f'{API_URL}/api/v1/memorials',
        headers={'Authorization': f'Bearer {token}'},
        json=memorial
    )
    return response.json()

def login(email, password):
    response = requests.post(
        f'{API_URL}/api/v1/auth/login',
        json={'email': email, 'password': password}
    )
    data = response.json()
    return data['access_token']
```

### cURL

```bash
# Get memorials
curl -X GET 'http://localhost:5001/api/v1/memorials?page=1&limit=50'

# Create memorial (authenticated)
curl -X POST 'http://localhost:5001/api/v1/memorials' \
  -H 'Authorization: Bearer <token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "Jane Smith",
    "date_of_birth": "1960-03-20",
    "date_of_death": "2024-01-08",
    "biography": "Jane was a teacher..."
  }'

# Login
curl -X POST 'http://localhost:5002/api/v1/auth/login' \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```
