# Deployment (Contabo VPS)

Server: `13.140.158.119` (hostname `vmi3360037`) · Domain: `mortalbook.com` · Path: `/var/www/mortalbook`

## How it is set up

```
Internet ─► nginx (host, :80/:443, TLS via certbot)
              └─► 127.0.0.1:3020 ─► frontend container (SvelteKit)
                                      └─► backend (Rust, :8080) ─► db (Postgres)
                                                              └─► translate (LibreTranslate, :5000)
```

Notes specific to this server:

- **nginx on the host already owns ports 80/443** for other sites (englishwind, guiter, kossti, software). Do **not** run Caddy or anything else on 80/443; mortalbook is just one more nginx `server` block.
- **Ports already used on the host:** `3000` (another Next.js app), `3010` (englishwind). Mortalbook's frontend is published on **`127.0.0.1:3020`**.
- Docker bypasses `ufw`, so the app port is bound to `127.0.0.1` only. Only nginx is public.
- Compose project name is the folder name, so volumes are `mortalbook_pgdata`, `mortalbook_uploads`, `mortalbook_ltmodels`. They do not clash with `englishwind_*`.
- Uploaded photos/videos live in the `uploads` volume, the database in `pgdata`. **Neither is inside the project folder**, so re-uploading code never touches data.

## Prerequisites

1. DNS: A records for `mortalbook.com` and `www.mortalbook.com` → `13.140.158.119`.
2. Docker + compose plugin installed (`curl -fsSL https://get.docker.com | sh`).
3. RAM: LibreTranslate needs roughly 2 GB+ on top of your other apps. Check with `free -h`.
   If RAM is tight, deploy without the `translate` service (the site then shows content in its original language).

## First-time deploy

### 1. Get the code onto the server

Option A — rsync from your computer (no GitHub access needed):

```bash
rsync -avz --exclude target --exclude node_modules --exclude .git \
  /media/monir/project/mortalbook/ root@13.140.158.119:/var/www/mortalbook/
```

Option B — git, using a read-only deploy key:

```bash
ssh-keygen -t ed25519 -f ~/.ssh/mortalbook_deploy -N ""
cat ~/.ssh/mortalbook_deploy.pub     # add in GitHub: repo → Settings → Deploy keys (no write access)
GIT_SSH_COMMAND="ssh -i ~/.ssh/mortalbook_deploy" \
  git clone git@github.com:monijaman/mortalbook.git /var/www/mortalbook
```

(`git clone ...` as plain `git@github.com` fails with `Permission denied (publickey)` unless a key is registered.)

### 2. Configure and start

```bash
cd /var/www/mortalbook
[ -f .env ] || echo "POSTGRES_PASSWORD=$(openssl rand -hex 24)" > .env   # never overwrite an existing .env
sed -i 's/"3000:3000"/"127.0.0.1:3020:3000"/' docker-compose.yml
grep -n 3020 docker-compose.yml
docker compose up -d --build
docker compose ps
curl -s http://127.0.0.1:3020/api/health        # {"status":"ok"}
```

> Do not change `POSTGRES_PASSWORD` after the first start. Postgres stores the password in the
> `pgdata` volume; a different value in `.env` will make the backend fail to log in.

On first start the backend runs the migrations, including `0002_seed_people.sql` (140 seeded people).

### 3. nginx

```bash
cat > /etc/nginx/sites-available/mortalbook.conf <<'CONF'
server {
    listen 80;
    server_name mortalbook.com www.mortalbook.com;

    client_max_body_size 300m;           # photo/video uploads

    location / {
        proxy_pass http://127.0.0.1:3020;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_request_buffering off;     # stream large uploads
        proxy_read_timeout 300s;
    }
}
CONF
ln -s /etc/nginx/sites-available/mortalbook.conf /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx       # reloads only if the config test passes
```

### 4. HTTPS

```bash
apt install -y certbot python3-certbot-nginx     # skip if already installed
certbot --nginx -d mortalbook.com -d www.mortalbook.com
```

Certbot installs a renewal timer; check with `systemctl list-timers | grep certbot`.

### 5. Verify

```bash
curl -I https://mortalbook.com
docker compose logs -f translate     # first start downloads language models (minutes); Ctrl+C to leave
```

## Updating

```bash
# rsync again (or `git pull` if using Option B), then on the server:
cd /var/www/mortalbook
docker compose up -d --build
```

New migrations run automatically when the backend starts.

## Operations

```bash
docker compose ps                          # status
docker compose logs -f backend             # API logs
docker compose logs -f frontend
docker compose restart backend
docker compose down                        # stop (data volumes are kept)
```

### Backup

```bash
# database
docker compose exec -T db pg_dump -U mortalbook mortalbook | gzip > backup-$(date +%F).sql.gz
# uploads
docker run --rm -v mortalbook_uploads:/data -v "$PWD":/backup alpine \
  tar czf /backup/uploads-$(date +%F).tgz -C /data .
```

### Restore the database

```bash
gunzip -c backup-YYYY-MM-DD.sql.gz | docker compose exec -T db psql -U mortalbook mortalbook
```

## Troubleshooting

| Symptom | Check |
|---|---|
| `bind: address already in use` on 3020 | `ss -tlnp \| grep 3020`; pick another free port in `docker-compose.yml` and in the nginx `proxy_pass`. |
| nginx `502 Bad Gateway` | `docker compose ps`; `curl http://127.0.0.1:3020/api/health`; `docker compose logs backend`. |
| Uploads fail with `413` | `client_max_body_size` in the nginx site (and certbot's 443 block, if it was edited). |
| Backend: password authentication failed | `.env` password differs from the one the `pgdata` volume was created with. Restore the old value, or (only if there is no data to keep) `docker compose down -v` and start again. |
| Seed migration failed halfway | Fix the cause, then reset with `docker compose down -v` (**deletes all data**) and start again. |
| Pages are not translated | `docker compose logs translate` — models may still be downloading. Without the service, content shows in its original language. |
| Backend `translation service unavailable` | `translate` container down or out of memory: `docker compose ps`, `free -h`, `docker compose logs translate`. |
| Seed photos missing | Photos are hotlinked from Wikimedia Commons; they need outbound internet from the visitor's browser, not the server. |

## Seed data and attribution

`backend/migrations/0002_seed_people.sql` holds 140 deceased Bangladeshi public figures. Names and dates come from
Wikidata, bios from English Wikipedia (CC BY-SA 4.0, each bio ends with a "Source: Wikipedia" link — keep it),
photos are hotlinked from Wikimedia Commons. Year-only birth dates are stored as NULL.

## Not covered yet

Rate limiting / spam protection on `/admin` (anyone can submit), accounts and moderation, email/SMS reminders,
automated backups (cron), monitoring.
