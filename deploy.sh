#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════
# Logistics Workflow — One-Click Deploy to VPS
# ═══════════════════════════════════════════════════════════════
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log()  { echo -e "${GREEN}[✓]${NC} $*"; }
info() { echo -e "${BLUE}[*]${NC} $*"; }
warn() { echo -e "${YELLOW}[!]${NC} $*"; }
err()  { echo -e "${RED}[✗]${NC} $*"; exit 1; }

# ── Usage ────────────────────────────────────────────────────
usage() {
    cat <<EOF
Usage: ./deploy.sh [COMMAND]

Commands:
  setup       Install Docker + deps on fresh VPS (Ubuntu/Debian)
  build       Build all Docker images
  build-web   Build frontend (Vue SPA → web/dist/)
  start       Start all services (db + api + nginx)
  stop        Stop all services
  restart     Restart all services
  logs        Tail logs from all services
  status      Show service status
  db-dump     Export database to ./db-backup/
  db-restore  Restore database from ./db-backup/
  ssl         Enable HTTPS via Let's Encrypt: ./deploy.sh ssl <domain> [email]
  clean       Stop and remove everything (volumes too!)
  help        Show this message

Quick start (fresh VPS):
  ./deploy.sh setup && ./deploy.sh build-web && ./deploy.sh start

Quick start (local dev):
  ./deploy.sh build-web start
EOF
}

# ── Prerequisite checks ──────────────────────────────────────
check_deps() {
    command -v docker >/dev/null 2>&1 || err "Docker not found. Run: ./deploy.sh setup"
    docker compose version >/dev/null 2>&1 || docker-compose version >/dev/null 2>&1 || \
        err "docker compose not found. Install Docker Compose v2."
}

compose_cmd() {
    if docker compose version >/dev/null 2>&1; then
        echo "docker compose"
    else
        echo "docker-compose"
    fi
}

# ── Commands ─────────────────────────────────────────────────
cmd_setup() {
    info "Installing Docker on $(lsb_release -ds 2>/dev/null || echo 'Linux')..."

    if command -v docker >/dev/null 2>&1; then
        log "Docker already installed: $(docker --version)"
    else
        curl -fsSL https://get.docker.com | bash
        sudo usermod -aG docker "$USER"
        log "Docker installed. You may need to log out and back in for group changes."
    fi

    # Ensure docker compose plugin
    if ! docker compose version >/dev/null 2>&1; then
        info "Installing docker compose plugin..."
        sudo apt-get update -qq && sudo apt-get install -y -qq docker-compose-plugin
    fi

    log "Setup complete. Run: ./deploy.sh start"
}

cmd_build() {
    check_deps
    info "Building Rust backend (this may take a few minutes)..."
    $(compose_cmd) build api --no-cache
    log "Backend image built."
}

cmd_build_web() {
    info "Building Vue frontend..."
    cd web
    if [ ! -d node_modules ]; then
        npm install
    fi
    npm run build-only
    cd ..
    log "Frontend built → web/dist/"
}

cmd_start() {
    check_deps
    if [ ! -f .env ]; then
        warn ".env not found — copying env.example as template"
        cp env.example .env
        warn "Edit .env with your production secrets, then re-run."
        exit 1
    fi

    if [ ! -d web/dist ]; then
        warn "web/dist/ not found — building frontend first..."
        cmd_build_web
    fi

    info "Starting all services..."
    $(compose_cmd) up -d --build
    log "Services starting..."
    sleep 3
    cmd_status
    echo ""
    log "App running at http://$(hostname -I 2>/dev/null | awk '{print $1}' || echo 'YOUR_VPS_IP'):${PORT:-80}"
}

cmd_stop() {
    check_deps
    info "Stopping all services..."
    $(compose_cmd) down
    log "All services stopped."
}

cmd_restart() {
    check_deps
    info "Restarting all services..."
    $(compose_cmd) down && $(compose_cmd) up -d
    log "Services restarted."
}

cmd_logs() {
    check_deps
    $(compose_cmd) logs -f --tail=100
}

cmd_status() {
    check_deps
    $(compose_cmd) ps
}

cmd_db_dump() {
    check_deps
    mkdir -p db-backup
    local ts=$(date +%Y%m%d_%H%M%S)
    local file="db-backup/dump_${ts}.sql"

    info "Dumping database to $file ..."
    $(compose_cmd) exec -T db pg_dump -U "${DB_USER:-lw_user}" "${DB_NAME:-logistics_workflow}" > "$file"
    log "Database dumped → $file ($(wc -c < "$file") bytes)"

    # Also save as latest.sql for auto-restore on fresh start
    cp "$file" db-backup/latest.sql
    log "Also saved as db-backup/latest.sql for auto-restore"
}

cmd_db_restore() {
    check_deps
    local file="${1:-db-backup/latest.sql}"
    if [ ! -f "$file" ]; then
        err "Backup not found: $file"
    fi
    info "Restoring database from $file ..."
    $(compose_cmd) exec -T db psql -U "${DB_USER:-lw_user}" -d "${DB_NAME:-logistics_workflow}" < "$file"
    log "Database restored."
}

# ── HTTPS via Let's Encrypt (webroot, no downtime) ──────────
cmd_ssl() {
    check_deps
    local domain="${1:-}" email="${2:-}"
    [ -n "$domain" ] || err "Usage: ./deploy.sh ssl <domain> [email]"
    email="${email:-admin@${domain}}"
    mkdir -p certbot/www certbot/conf

    info "Ensuring nginx is up on :80 for the ACME challenge..."
    $(compose_cmd) up -d nginx

    info "Requesting certificate for ${domain} (email: ${email})..."
    $(compose_cmd) run --rm certbot certonly --webroot -w /var/www/certbot \
        -d "$domain" --email "$email" --agree-tos --no-eff-email --non-interactive \
        || err "certbot failed — check DNS points $domain → this VPS and port 80 is open."

    info "Activating TLS nginx config..."
    sed "s/DOMAIN/${domain}/g" nginx-ssl.conf.template > nginx.ssl.active.conf
    if grep -q '^NGINX_CONF=' .env 2>/dev/null; then
        sed -i "s|^NGINX_CONF=.*|NGINX_CONF=nginx.ssl.active.conf|" .env
    else
        echo "NGINX_CONF=nginx.ssl.active.conf" >> .env
    fi
    $(compose_cmd) up -d nginx
    log "HTTPS enabled → https://${domain}"
    warn "Renewal: add to crontab → 0 3 * * * cd $SCRIPT_DIR && \$(compose_cmd) run --rm certbot renew && \$(compose_cmd) restart nginx"
}

cmd_clean() {
    check_deps
    warn "This will remove ALL containers, volumes, and data!"
    read -rp "Are you sure? [y/N] " confirm
    if [ "$confirm" != "y" ] && [ "$confirm" != "Y" ]; then
        info "Aborted."
        exit 0
    fi
    $(compose_cmd) down -v --remove-orphans
    log "Everything removed. Database data is gone."
}

# ── Dispatch ─────────────────────────────────────────────────
case "${1:-help}" in
    setup)       cmd_setup ;;
    build)       cmd_build ;;
    build-web)   cmd_build_web ;;
    start)       cmd_start ;;
    stop)        cmd_stop ;;
    restart)     cmd_restart ;;
    logs)        cmd_logs ;;
    status)      cmd_status ;;
    db-dump)     cmd_db_dump ;;
    db-restore)  cmd_db_restore "${2:-}" ;;
    ssl)         cmd_ssl "${2:-}" "${3:-}" ;;
    clean)       cmd_clean ;;
    help|--help|-h) usage ;;
    *)           usage; exit 1 ;;
esac
