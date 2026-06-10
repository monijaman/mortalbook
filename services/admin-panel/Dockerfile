# ─────────────────────────────────────────────────────────────────────────────
# Stage 1: build Vue 3 SPA with Vite
# ─────────────────────────────────────────────────────────────────────────────
FROM node:20-alpine AS builder

WORKDIR /app

COPY package.json package-lock.json* ./
RUN npm install

COPY . .
RUN npm run build

# ─────────────────────────────────────────────────────────────────────────────
# Stage 2: serve with nginx (~8 MB vs ~130 MB Node runtime)
# ─────────────────────────────────────────────────────────────────────────────
FROM nginx:1.25-alpine AS runtime

# Remove default nginx config
RUN rm /etc/nginx/conf.d/default.conf

# SPA-friendly nginx config
RUN printf '%s\n' \
    'server {' \
    '    listen       8080;' \
    '    server_name  _;' \
    '    root         /usr/share/nginx/html;' \
    '    index        index.html;' \
    '' \
    '    gzip on;' \
    '    gzip_types text/plain text/css application/json application/javascript text/xml application/xml;' \
    '    gzip_min_length 1000;' \
    '' \
    '    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff2?)$ {' \
    '        expires 1y;' \
    '        add_header Cache-Control "public, immutable";' \
    '    }' \
    '' \
    '    location / {' \
    '        try_files $uri $uri/ /index.html;' \
    '    }' \
    '' \
    '    location /nginx-health {' \
    '        access_log off;' \
    '        return 200 '"'"'ok'"'"';' \
    '        add_header Content-Type text/plain;' \
    '    }' \
    '}' \
    > /etc/nginx/conf.d/spa.conf

# Copy Vite build output (dist/)
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
