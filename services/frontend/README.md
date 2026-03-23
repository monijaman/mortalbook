# Next.js Frontend

Production-ready Next.js 14 application with Server-Side Rendering (SSR), optimized for SEO and performance.

## 🏗️ Architecture

### Project Structure

```
frontend/
├── pages/                   # Next.js pages & routes
│   ├── index.tsx           # Homepage
│   ├── memorials/
│   │   └── [id].tsx        # Memorial detail page
│   ├── about.tsx           # About page
│   ├── _app.tsx            # App wrapper
│   └── _document.tsx       # HTML document
├── components/             # Reusable React components
│   ├── Layout.tsx          # Layout wrapper
│   ├── Header.tsx          # Header component
│   ├── Footer.tsx          # Footer component
│   ├── MemorialCard.tsx    # Memorial card
│   └── SearchBar.tsx       # Search component
├── lib/
│   ├── api/
│   │   └── memorials.ts    # API client
│   └── types/
│       └── index.ts        # TypeScript types
├── styles/
│   └── globals.css         # Global styles
├── public/                 # Static assets
├── k8s/                    # Kubernetes manifests
├── Dockerfile              # Multi-stage build
├── next.config.js          # Next.js config
├── tailwind.config.js      # Tailwind CSS config
├── package.json            # Dependencies
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

# Create environment file
cp .env.example .env.local

# Run development server
npm run dev

# Application accessible at http://localhost:3000
```

### Environment Variables

```bash
NEXT_PUBLIC_API_URL=http://localhost:5001
NEXT_PUBLIC_ADMIN_API_URL=http://localhost:5002
```

## 📄 Pages

### Homepage (`/`)

- Displays list of all memorials
- Search functionality
- Memorial cards with preview
- SEO optimized

### Memorial Detail (`/memorials/:id`)

- Full memorial information
- Media gallery
- Biography display
- SEO optimized with metadata

### About (`/about`)

- Information about Mortalbook
- Mission statement
- Feature list

## 🎨 Components

### MemorialCard

Displays memorial summary with name, dates, and biography preview.

### SearchBar

Full-width search input for filtering memorials.

### Header & Footer

Navigation and site branding.

### Layout

Main layout wrapper with header and footer.

## 🔍 SEO Features

- **Next.js Image**: Optimized image loading
- **next-seo**: Automatic meta tags and Open Graph
- **Server-Side Rendering**: HTML rendered on server
- **Sitemap**: XML sitemap generation (add sitemap.xml)
- **Structured Data**: JSON-LD for memorials (to implement)

## 🎨 Styling

- **Tailwind CSS**: Utility-first CSS framework
- **Responsive Design**: Mobile-first approach
- **Dark Mode**: Ready for dark mode support (add)

## 📱 API Integration

### Memorial Service API

```typescript
// Get all memorials
const memorials = await memorialService.getMemorials(skip, limit);

// Get memorial by ID
const memorial = await memorialService.getMemorialById(id);

// Get memorial media
const media = await memorialService.getMemorialMedia(memorialId);

// Search memorials
const results = await memorialService.searchMemorials(query);
```

## 🐳 Docker

### Build Image

```bash
docker build -t mortalbook/frontend:latest .
```

### Run Container

```bash
docker run -p 3000:3000 \
  -e NEXT_PUBLIC_API_URL=http://localhost:5001 \
  mortalbook/frontend:latest
```

## 🛠️ Kubernetes Deployment

### Deploy

```bash
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# Verify
kubectl get deployments
kubectl get pods
kubectl get services
```

## 🧪 Testing

```bash
npm run test
npm run test:watch
```

## 📈 Performance Optimization

- Next.js automatic code splitting
- Image optimization with next/image
- CSS minification with Tailwind
- HTTP/2 server push
- Gzip compression

### Build & Start Metrics

```bash
npm run build    # Production build
npm start        # Start production server
```

## 🔐 Security

- HTTPS ready
- Security headers (X-Frame-Options, CSP, etc.)
- CORS configured
- Input validation

## 📚 Technologies

- **Next.js 14** - React framework with SSR
- **React 18** - UI library
- **TypeScript** - Type safety
- **Tailwind CSS** - Styling
- **Axios** - HTTP client
- **next-seo** - SEO optimization

## 🎯 Features

- [x] Server-Side Rendering (SSR)
- [x] Static Generation (SSG) ready
- [x] SEO optimization
- [x] Mobile responsive
- [x] API integration
- [x] Component-based architecture
- [x] TypeScript support
- [x] Tailwind CSS styling

## 🔄 Planned Enhancements

- [ ] Pagination component
- [ ] Commenting system
- [ ] User authentication
- [ ] Memorial creation
- [ ] Admin dashboard access
- [ ] Image lazy loading
- [ ] Search autocomplete
- [ ] Filtering options
- [ ] Dark mode
- [ ] Analytics integration

## 📄 License

MIT

---

**Built with ❤️ for the Mortalbook project**
