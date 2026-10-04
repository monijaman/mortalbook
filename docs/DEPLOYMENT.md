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

1. DNS zone for `mortalbook.com` with A records pointing to `13.140.158.119` (see **DNS** below).
2. Docker + compose plugin installed (`curl -fsSL https://get.docker.com | sh`).
3. RAM: LibreTranslate needs roughly 2 GB+ on top of your other apps. Check with `free -h`.
   If RAM is tight, deploy without the `translate` service (the site then shows content in its original language).

## DNS (Contabo)

The registrar delegates `mortalbook.com` to `ns1/ns2/ns3.contabo.net`, so the zone must exist **in the Contabo
customer panel → DNS Zone Management**. If it does not, Contabo's servers answer `REFUSED`, public resolvers
return `SERVFAIL`, and Let's Encrypt cannot issue a certificate.

Records in use (TTL 86400):

| Type | Name | Value |
|---|---|---|
| A | `@` (mortalbook.com) | `13.140.158.119` |
| A | `www` | `13.140.158.119` |
| A | `*` | `13.140.158.119` |
| A | `mail` | `13.140.158.119` |
| MX | `@` | `mail.mortalbook.com` (not used yet; for future email reminders) |

Do not add an AAAA record that points elsewhere.

Verify before running certbot (all must print `13.140.158.119`):

```bash
dig A mortalbook.com @ns1.contabo.net +norecurse +short
dig A mortalbook.com @8.8.8.8 +short
dig A www.mortalbook.com @1.1.1.1 +short
```

If a public resolver still shows `SERVFAIL` right after a fix, its failure answer is cached for a few minutes; wait,
or purge at https://dns.google/cache and https://one.one.one.one/purge-cache/.
**Do not retry certbot repeatedly** while DNS is broken: Let's Encrypt rate-limits failed validations.

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
grep -n -E '3020|ORIGIN' docker-compose.yml     # repo already binds 127.0.0.1:3020 and sets ORIGIN
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

Certbot installs a renewal timer; check with `systemctl list-timers | grep certbot`. Choose the redirect to HTTPS
when asked.

**Why this matters:** every site on this host has its own `listen 443 ssl` block. Until mortalbook has one, nginx
serves an HTTPS request for `mortalbook.com` from the *first* 443 block it loaded (`english.kossti.com`), so
visitors see the wrong site.

Optional safeguard so unknown domains never land on a real site over HTTPS:

```bash
cat > /etc/nginx/sites-available/000-https-reject <<'CONF'
server {
    listen 443 ssl default_server;
    listen [::]:443 ssl default_server;
    server_name _;
    ssl_reject_handshake on;
}
CONF
ln -s /etc/nginx/sites-available/000-https-reject /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx
```

### 5. Verify

```bash
curl -I https://mortalbook.com
docker compose logs -f translate     # first start downloads language models (minutes); Ctrl+C to leave
```

## CI/CD (GitHub Actions)

Workflow: `.github/workflows/ci-cd.yml`.

- **Every push and pull request:** `backend` job (`cargo check`) and `frontend` job (`npm install && npm run build`).
- **Push to `main` only, after both pass:** `deploy` job — rsyncs the repo to `/var/www/mortalbook` (excluding `.git`,
  `.env`, `target`, `node_modules`), runs `docker compose up -d --build`, then polls
  `http://127.0.0.1:3020/api/health` for up to 60 s and fails the run (printing backend logs) if it never answers.
- `.env`, the database and uploads are never touched by a deploy (`.env` is excluded; data lives in Docker volumes).
- The repo's `docker-compose.yml` is what runs in production (port `127.0.0.1:3020`, `ORIGIN=https://mortalbook.com`).
  Do not edit the server copy by hand: the next deploy overwrites it.

### One-time setup

1. Create a dedicated deploy key and authorize it on the server:

```bash
ssh-keygen -t ed25519 -f ~/mortalbook_deploy -N "" -C "github-actions-deploy"
ssh-copy-id -i ~/mortalbook_deploy.pub root@13.140.158.119
# or append the .pub line to /root/.ssh/authorized_keys (use >>, never >)
ssh -i ~/mortalbook_deploy root@13.140.158.119 "echo it works"     # test
```

`ssh-copy-id` only appends a line to `authorized_keys`; it does not touch existing keys.

2. Add three secrets in GitHub (**Settings → Secrets and variables → Actions → New repository secret**):

| Secret | Value |
|---|---|
| `DEPLOY_SSH_KEY` | the whole **private** key: `cat ~/mortalbook_deploy`, from `-----BEGIN OPENSSH PRIVATE KEY-----` to `-----END OPENSSH PRIVATE KEY-----` inclusive |
| `DEPLOY_HOST` | `13.140.158.119` |
| `DEPLOY_USER` | `root` |

   A server has no clipboard (`xclip` fails with "Can't open display"): print the key with `cat` and select it with the
   mouse, then **Ctrl+Shift+C**. Check the file is valid first with `ssh-keygen -y -f ~/mortalbook_deploy`.

3. After one successful deploy, remove the private key from the server (it only needs the public key):
   `shred -u ~/mortalbook_deploy`. Never paste the private key anywhere except the GitHub secret.

4. Re-run a failed workflow with **Actions → the run → Re-run all jobs**.

Revoking access: delete the `github-actions-deploy` line from `/root/.ssh/authorized_keys`.

Known limits: the Rust build runs on the VPS (can use 1–2 GB RAM next to the other apps). If a deploy ever runs out
of memory, build the images in Actions, push them to GHCR and only pull on the server. CI currently only compiles; add
`cargo test` and frontend checks when tests exist.

## Updating (manual)

```bash
# normally CI/CD does this on every push to main. To deploy by hand: rsync (or `git pull`), then on the server:
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
| `https://mortalbook.com` shows another site (englishwind) | No 443 server block for mortalbook yet: fix DNS, then `certbot --nginx -d mortalbook.com -d www.mortalbook.com`. |
| certbot: `SERVFAIL looking up A` | DNS zone missing or not live at Contabo (`dig A mortalbook.com @ns1.contabo.net +norecurse`; `REFUSED` = no zone). See **DNS**. |
| White page in the browser but `curl -H 'Host: mortalbook.com' http://127.0.0.1/` works | HTTPS not set up (curl `-k` to https returns `000`). Use plain `http://` to test until the certificate exists; then check the browser console (F12). |
| Actions: `Load key ... error in libcrypto` then `Permission denied (publickey)` | `DEPLOY_SSH_KEY` is malformed (public key pasted, BEGIN/END lines missing, line breaks lost). Re-copy the whole private key. |
| Actions: `Permission denied (publickey)` with a valid secret | The `.pub` line is not in `/root/.ssh/authorized_keys`; run `ssh-copy-id` again. |
| Actions: old jobs `test-go`, `test-python`, `build-admin-panel` | Leftover workflows from the old stack; only `ci-cd.yml` should exist in `.github/workflows/`. |
| `bind: address already in use` on 3020 | `ss -tlnp \| grep 3020`; pick another free port in `docker-compose.yml` and in the nginx `proxy_pass`. |
| nginx `502 Bad Gateway` | `docker compose ps`; `curl http://127.0.0.1:3020/api/health`; `docker compose logs backend`. |
| Uploads fail with `413` | `client_max_body_size` in the nginx site (and certbot's 443 block, if it was edited). |
| Backend: password authentication failed | `.env` password differs from the one the `pgdata` volume was created with. Restore the old value, or (only if there is no data to keep) `docker compose down -v` and start again. |
| Seed migration failed halfway | Fix the cause, then reset with `docker compose down -v` (**deletes all data**) and start again. |
| Pages are not translated | `docker compose logs translate` — models may still be downloading. Without the service, content shows in its original language. |
| Backend `translation service unavailable` | `translate` container down or out of memory: `docker compose ps`, `free -h`, `docker compose logs translate`. |
| Seed photos missing | Photos are hotlinked from Wikimedia Commons; they need outbound internet from the visitor's browser, not the server. |

## Seed data and attribution

Two migrations seed deceased Bangladeshi public figures (389 people):

- `0002_seed_people.sql` — the first 140.
- `0003_more_people_and_details.sql` — adds `occupation`, `birth_place` and `death_place` columns, fills them (and fuller
  biographies) for the first 140, and inserts 249 more.

Names, dates, occupations and places come from Wikidata; biographies from English Wikipedia (CC BY-SA 4.0 — each bio ends
with a "Source: Wikipedia" link, keep it); photos are hotlinked from Wikimedia Commons. Year-only birth dates are stored as
NULL. Migrations run once at backend start and each runs in a transaction, so a failure leaves the database unchanged.
Existing data and submissions are kept; only people with a matching name and death date from the first seed are updated.

## Not covered yet

Rate limiting / spam protection on `/admin` (anyone can submit), accounts and moderation, email/SMS reminders,
automated backups (cron), monitoring.
