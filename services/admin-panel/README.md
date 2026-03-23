# Vue 3 Admin Panel

A modern, production-ready Vue 3 admin dashboard for managing the Mortalbook memorial platform.

## 🏗️ Architecture

### Project Structure

```
admin-panel/
├── src/
│   ├── main.js                  # Application entry point
│   ├── App.vue                  # Root component
│   ├── components/              # Reusable components
│   │   ├── Header.vue
│   │   ├── Sidebar.vue
│   │   ├── Notifications.vue
│   │   └── LoginForm.vue
│   ├── views/                   # Page components
│   │   ├── DashboardView.vue
│   │   ├── LoginView.vue
│   │   ├── UsersView.vue
│   │   ├── MemorialsView.vue
│   │   ├── AnalyticsView.vue
│   │   └── NotFoundView.vue
│   ├── stores/                  # Pinia state management
│   │   ├── auth.js              # Authentication state
│   │   └── ui.js                # UI state
│   ├── services/                # API clients
│   │   └── api.js               # API service
│   ├── router/                  # Vue Router setup
│   │   └── index.js
│   └── styles/
│       └── index.css
├── public/                      # Static assets
├── k8s/                         # Kubernetes manifests
├── Dockerfile
├── vite.config.js
├── tailwind.config.js
├── package.json
├── index.html
└── README.md
```

## 🚀 Getting Started

### Prerequisites

- Node.js 18+
- npm or yarn

### Local Development

```bash
# Install dependencies
npm install

# Create .env file
VITE_API_URL=http://localhost:5002

# Run development server
npm run dev

# Access at http://localhost:8080
```

### Environment Variables

```bash
VITE_API_URL=http://localhost:5002
```

## 🎨 Features

- ✅ Modern Vue 3 Composition API
- ✅ Pinia state management
- ✅ Tailwind CSS styling
- ✅ JWT authentication
- ✅ Protected routes
- ✅ Responsive design
- ✅ API integration
- ✅ Notifications system

## 📋 Pages

### Login

- Email and password authentication
- Error handling
- Loading states

### Dashboard

- System statistics
- Recent activity
- System status monitoring

### Users Management

- List all users
- Delete users
- Admin status display

### Memorials Management

- List all memorials
- Delete memorials
- Status management

### Analytics

- Event tracking
- Period analysis
- Statistical overview

## 🔐 Authentication

### Login Flow

1. User enters credentials
2. API validates and returns tokens
3. Token stored in localStorage
4. Axios interceptor adds token to requests
5. Protected routes check authentication

### Token Management

```javascript
// Stored in localStorage
localStorage.setItem("token", accessToken);
localStorage.setItem("refreshToken", refreshToken);
```

## 🛠️ State Management (Pinia)

### Auth Store

```javascript
const auth = useAuthStore();
auth.login(email, password);
auth.logout();
auth.isAuthenticated();
```

### UI Store

```javascript
const ui = useUIStore();
ui.toggleSidebar();
ui.addNotification(message, type);
```

## 🐳 Docker

### Build Image

```bash
docker build -t mortalbook/admin-panel:latest .
```

### Run Container

```bash
docker run -p 8080:8080 \
  -e VITE_API_URL=http://localhost:5002 \
  mortalbook/admin-panel:latest
```

## 🛠️ Kubernetes Deployment

### Deploy

```bash
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml
```

## 📱 Build & Production

```bash
# Development build
npm run dev

# Production build
npm run build

# Preview production build
npm run preview
```

## 📚 Technologies

- **Vue 3** - Progressive JavaScript framework
- **Composition API** - Modern Vue development
- **Vite** - Next generation build tool
- **Pinia** - State management
- **Tailwind CSS** - Utility CSS framework
- **Axios** - HTTP client
- **Vue Router** - Routing

## 🎯 Features

- [x] Authentication system
- [x] Protected routes
- [x] User management
- [x] Memorial management
- [x] Analytics dashboard
- [x] Responsive design
- [x] Notification system
- [x] Sidebar navigation

## 🔄 Planned Enhancements

- [ ] User role management
- [ ] Two-factor authentication
- [ ] Audit logs viewer
- [ ] Content moderation queue
- [ ] Advanced analytics charts
- [ ] Export functionality
- [ ] Settings management
- [ ] Dark mode
- [ ] Search and filtering
- [ ] Bulk actions

## 📄 License

MIT

---

**Built with ❤️ for the Mortalbook project**
