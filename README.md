# Mortalbook

A place to remember people who have passed away. Rust (axum + sqlx) API, SvelteKit frontend, PostgreSQL, Docker.

- **Today page** – everyone in the book who passed away on today's date (visitor's local date).
- **Detail page** – photos, videos (uploaded or YouTube/Vimeo links) and the person's story.
- **Translation** – the visitor's browser language is detected on first visit and all UI text and
  stories are translated into it. A language picker sits at the top. Stories are tagged with their
  detected source language when submitted. Translations are cached in Postgres.
- **Add a person** (`/admin`) – open to anyone for now (no login).

## Run everything

```bash
docker compose up --build
# http://localhost:3000
```

The first start of the `translate` service (LibreTranslate) downloads language models; translation
works once it is ready. Without it the site still works in the original language.

## Develop

```bash
docker compose -f docker-compose.dev.yml up -d
cd backend  && DATABASE_URL=postgres://mortalbook:mortalbook@localhost:5432/mortalbook cargo run
cd frontend && npm install && npm run dev      # http://localhost:3000
```

Layout: `backend/` (API, migrations in `backend/migrations`), `frontend/` (SvelteKit; `src/hooks.server.js`
proxies `/api` and `/uploads` to the backend).

## Deploy

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) (Contabo VPS, nginx, HTTPS, backups).

## Not built yet

Email/SMS reminders, accounts/login, moderation, rate-limiting on submissions.
