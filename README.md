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
- **Admin** (`/admin/login`) – sign in to list, search, add, edit (rich text story editor) and delete
  people, review recent-death candidates, and generate a story with OpenAI + web search.

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

## Configuration (`.env`)

Settings live in a `.env` file next to `docker-compose.yml` (on the server: `/var/www/mortalbook/.env`).
It is never committed and never overwritten by a deploy. Start from [.env.example](.env.example).

| Variable | Purpose |
| --- | --- |
| `POSTGRES_PASSWORD` | Database password. Do not change after the first start. |
| `ADMIN_USERNAME` / `ADMIN_PASSWORD` | Admin login. The password must be at least 16 characters. |
| `ADMIN_REVIEW_TOKEN` | Token for the recent-death review API. |
| `RECENT_DEATHS_INGEST_TOKEN` | Token used by the scheduled recent-deaths job. |
| `OPENAI_API_KEY` | Enables **Generate story** in the editor. Optional. |
| `OPENAI_MODEL` | OpenAI model with web search (default `gpt-4.1`). Optional. |

View or change it on the server:

```bash
cd /var/www/mortalbook
grep -v -E 'PASSWORD|TOKEN|KEY' .env      # safe view: hides secrets
grep -c . .env && cut -d= -f1 .env        # list the variable names only
nano .env                                 # edit (Ctrl+O save, Ctrl+X exit)
```

Set or replace one value without opening an editor:

```bash
sed -i '/^OPENAI_API_KEY=/d' .env
echo 'OPENAI_API_KEY=sk-your-key' >> .env
```

Changes only apply after the backend is recreated (a plain restart does not reload `.env`):

```bash
docker compose up -d --force-recreate backend
docker compose exec backend sh -c 'echo "user=[$ADMIN_USERNAME] pwlen=${#ADMIN_PASSWORD} openai=${OPENAI_API_KEY:+set}"'
```

Avoid `$`, quotes and spaces in values. Never paste secrets into chats or commit them.

## Deploy

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) (Contabo VPS, nginx, HTTPS, backups).

## Not built yet

Email/SMS reminders, visitor accounts/login, rate-limiting on public submissions.

The recent-death review queue uses `ADMIN_REVIEW_TOKEN` for the admin page and
`RECENT_DEATHS_INGEST_TOKEN` for the scheduled GitHub Actions workflow. See
[deployment setup](./docs/DEPLOYMENT.md#recent-death-review-queue) before enabling it.
