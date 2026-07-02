# Logistics Workflow — Production Build & Release Plan

**Goal:** Ship logistics-workflow v0.3.x to production on the VPS (62.72.42.178) via Docker, with real secrets, HTTPS, and verified health — reusing the existing deploy pack.

**Architecture:** Nginx(:80/443) → Vue SPA (`web/dist`) + reverse-proxy `/api/*` → Rust API (`:19876`) → PostgreSQL 17 (volume `pgdata`). 3-container `docker-compose`. Config via `LW_*` env from `.env`; DB auto-seeded from `db-backup/seed.sql` on first run.

**Tech Stack:** Rust (release), Vue 3 + Vite, PostgreSQL 17-alpine, nginx:alpine, certbot, Docker Compose v2.

**Current state (already done — do NOT redo):**
- Deploy pack exists: `Dockerfile`, `docker-compose.yml`, `deploy.sh` (setup/build/start/db-dump/db-restore/ssl), `nginx.conf`, `nginx-ssl.conf.template`, `DEPLOY.md`.
- Code uploaded to VPS `~/logistics-workflow` (rsync). `.env` currently holds DEV creds.
- Tests green: cargo 23, vitest 56, e2e API 0-fail. Version 0.3.1.
- DB seed = clean beginning-form (4 users, empty business tables).

---

## Phase 0 — Pre-flight fixes (local, MUST do before release)

These are real defects/gaps found in the current pack. Fix + commit before deploying.

### Task 0.1: Fix healthcheck — runtime image has no `curl`
`Dockerfile` runtime stage installs only `ca-certificates libssl3`; both healthchecks call `curl -f` → API container reports **unhealthy** forever, and `db: condition: service_healthy` style waits can hang.
- Modify `Dockerfile:21-23` — add `curl` to the runtime `apt-get install` line:
  `ca-certificates libssl3 curl`
- OR (leaner) change both healthchecks to the busybox `wget` already present in nginx/postgres images — but API runtime is debian, so adding `curl` is the one-line fix.
- Verify after deploy (Phase 4): `docker compose ps` shows `lw-api` = healthy.

### Task 0.2: Confirm `.dockerignore` excludes heavy dirs
`Dockerfile` does `COPY . .` → without ignores it copies `target/`, `web/node_modules/`, `web/dist/` into build context (slow, huge).
- Read `.dockerignore`; ensure it lists: `target/`, `web/node_modules/`, `web/dist/`, `.git/`, `db-backup/*.sql` (keep seed? seed not needed in image — API doesn't read it).
- Add any missing.

### Task 0.3: Remove `web/node_modules` accidentally uploaded to VPS
rsync left ~176M `web/node_modules` on the VPS (harmless, rebuilt by `build-web`).
- On VPS: `rm -rf ~/logistics-workflow/web/node_modules ~/logistics-workflow/web/dist`

### Task 0.4: Decide config source of truth
`config.docker.toml` has `change-me` creds but compose feeds `LW_*` env (per `src/config.rs`), so the file is likely unused in Docker. Confirm `src/config.rs` prefers `LW_*` env; if so, delete `config.docker.toml` to avoid a misleading second config. If the app falls back to a `config.toml` inside the container, ensure none is baked in.

### Task 0.5: Clean cargo warnings (optional, cosmetic)
2 warnings: unused macro var in export, dead `dirs_documents` fn. Remove dead fn. Non-blocking.

### Task 0.6: Commit pre-flight
```bash
git add Dockerfile .dockerignore
git commit -m "prod: add curl to runtime image for healthcheck; tighten dockerignore"
git tag -a v0.3.2 -m "v0.3.2 production hardening"
git push origin main --tags
```

---

## Phase 1 — Production secrets (on VPS, NEVER commit)

`.env` on the VPS currently has dev creds (`DB_PASSWORD=logistics2026`, `API_TOKEN=lw-secret-token-change-me`). Production MUST rotate these.

### Task 1.1: Generate strong secrets
```bash
openssl rand -base64 24   # → DB_PASSWORD
openssl rand -hex 32      # → API_TOKEN
```

### Task 1.2: Write VPS `.env` (do NOT reuse dev values)
```
DB_NAME=logistics_workflow
DB_USER=lw_user
DB_PASSWORD=<generated>
API_TOKEN=<generated>
RUST_LOG=info
PORT=80
```
Note: `seed.sql` was dumped from the dev DB (owner-stripped) — it restores schema + 4 app users (tp_admin etc.) whose **login passwords are unchanged**. Rotate app-user passwords via the app/SQL after first boot if these are production accounts. DB_PASSWORD/API_TOKEN above are infra secrets, separate from app-user logins.

### Task 1.3: Verify `.env` is gitignored
`git check-ignore .env` → must print `.env`. (It is, per `.gitignore`.)

---

## Phase 2 — Build (on VPS)

### Task 2.1: Install Docker (fresh VPS only)
```bash
cd ~/logistics-workflow && ./deploy.sh setup
```
Expected: Docker + compose plugin present. May need re-login for docker group.

### Task 2.2: Build frontend
```bash
./deploy.sh build-web
```
Expected: `web/dist/` produced, 0 TS errors.

### Task 2.3: Build backend image
```bash
./deploy.sh build         # docker compose build api --no-cache
```
Expected: release binary compiles (`cargo build --release --locked`), image built. Slow (~3-8 min first time).

---

## Phase 3 — Database

### Task 3.1: First boot auto-seeds
On first `start`, Postgres runs `/docker-entrypoint-initdb.d/seed.sql` (mounted from `db-backup/seed.sql`) into the empty `pgdata` volume → schema + 4 users. App `ensure_schema()` then runs idempotent `CREATE TABLE IF NOT EXISTS`.
- No action unless migrating real data (then: `./deploy.sh db-restore <dump.sql>` after start).

### Task 3.2: Backup cadence (post-release)
```bash
./deploy.sh db-dump    # → db-backup/dump_<ts>.sql
# crontab: 0 2 * * * cd ~/logistics-workflow && ./deploy.sh db-dump
```

---

## Phase 4 — Deploy & verify

### Task 4.1: Start stack
```bash
./deploy.sh start
```
Expected: `lw-db`, `lw-api`, `lw-nginx` up.

### Task 4.2: Health checks
```bash
docker compose ps                              # all Up; lw-api healthy (after Task 0.1)
curl -f http://localhost:19876/health          # 200
curl -s http://localhost/api/dashboard         # JSON (public GET)
curl -s -o /dev/null -w '%{http_code}\n' \
  -X POST http://localhost/api/shipments -d '{}'   # 401 (auth enforced)
```

### Task 4.3: Login smoke test
```bash
curl -s -X POST http://localhost/api/login \
  -H 'Content-Type: application/json' \
  -d '{"username":"tp_admin","password":"@tp_admin123"}'   # 200 → {token,role}
```

### Task 4.4: Browser smoke
Open `http://<VPS_IP>` → login → create call → add shipment → advance to Telex → export report (3-sheet xlsx downloads).

---

## Phase 5 — HTTPS (needs a domain)

Requires an A-record → VPS IP and open ports 80+443.

### Task 5.1: Point DNS
`yourdomain.com A <VPS_IP>`; wait for propagation (`dig +short yourdomain.com`).

### Task 5.2: Issue cert + enable TLS
```bash
./deploy.sh ssl yourdomain.com you@mail.com
```
Expected: certbot webroot succeeds, `nginx.ssl.active.conf` generated, `NGINX_CONF` set in `.env`, nginx serves 443 + 301 from 80.

### Task 5.3: Verify
```bash
curl -sI https://yourdomain.com | head -1        # HTTP/2 200
curl -sI http://yourdomain.com | grep -i location # 301 → https
```

### Task 5.4: Auto-renew
```bash
# crontab
0 3 * * * cd ~/logistics-workflow && docker compose run --rm certbot renew && docker compose restart nginx
```

---

## Phase 6 — Firewall / hardening (VPS)

### Task 6.1: UFW
```bash
sudo ufw allow 22222/tcp   # SSH (custom port)
sudo ufw allow 80,443/tcp
sudo ufw enable
```
DB port 5432 is bound to `127.0.0.1` in compose — not internet-exposed. Keep it that way.

### Task 6.2: Confirm no stray exposure
`sudo ss -tlnp | grep -E ':5432|:19876'` → both should be 127.0.0.1 only (api is on the docker network, not host-published).

---

## Phase 7 — Update / rollback

### Update
```bash
git pull
./deploy.sh build-web && ./deploy.sh build && ./deploy.sh restart
```

### Rollback
```bash
git checkout v<previous-tag>
./deploy.sh build-web && ./deploy.sh build && ./deploy.sh restart
./deploy.sh db-restore db-backup/dump_<pre-release>.sql   # only if schema/data changed
```

---

## Files likely to change (Phase 0 only)
- `Dockerfile` (add curl)
- `.dockerignore` (verify/extend)
- `src/bridge.rs` (optional: drop dead `dirs_documents`)
- delete `config.docker.toml` (if confirmed unused)

## Validation gates (block release if any fail)
- [ ] `cargo test` 23 pass, `cd web && npm run test:unit` 56 pass
- [ ] `cargo build --release --locked` clean
- [ ] `docker compose ps` → all Up, `lw-api` **healthy**
- [ ] `/health` 200; unauth POST → 401; login → 200
- [ ] browser: full workflow to Telex + export works
- [ ] `.env` NOT in git; dev creds rotated
- [ ] (if domain) HTTPS 200 + HTTP→HTTPS redirect

## Risks / open questions
- **Secrets:** dev `.env` must be rotated on VPS — biggest risk if skipped.
- **App-user passwords** shipped in seed are known/default — rotate if these accounts are real.
- **HTTPS blocked without a domain** — IP-only stays HTTP. Confirm domain availability.
- **First `cargo build --release` in Docker is slow** — expect several minutes; not a failure.
- **Excel outline grouping** still deferred (needs `rust_xlsxwriter` upgrade + network) — not a release blocker.
