# Mortalbook

A place to remember people who have passed away. Rust (axum + sqlx) API, SvelteKit frontend, PostgreSQL, Docker.

- **Today page** – everyone in the book who passed away on today's date (visitor's local date).
- **Country-aware browsing** – the browser locale suggests a country, which is saved locally and can
  be changed in the search selector. Sparse country results are supplemented by nearby and then
  worldwide people.
- **Detail page** – photos, videos (uploaded or YouTube/Vimeo links) and the person's story.
- **Translation** – the visitor's browser language is detected on first visit and all UI text and
  stories are translated into it. A language picker sits at the top. Stories are tagged with their
  detected source language when submitted. Translations are cached in Postgres.
- **Recent-death review** (`/admin/review`) – scheduled public-figure candidates are held privately
  until an authenticated admin verifies, edits, and approves them.
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

Email/SMS reminders, visitor accounts/login, rate-limiting on public submissions.

The recent-death review queue uses `ADMIN_REVIEW_TOKEN` for the admin page and
`RECENT_DEATHS_INGEST_TOKEN` for the scheduled GitHub Actions workflow. See
[deployment setup](./docs/DEPLOYMENT.md#recent-death-review-queue) before enabling it.
