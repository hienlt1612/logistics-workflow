# Logistics Workflow — Docker Deployment Guide

Triển khai toàn bộ dự án (Rust API + Vue SPA + PostgreSQL) lên VPS bằng Docker.

## Yêu cầu hệ thống

- Ubuntu 22.04+ / Debian 12+
- 1 GB RAM, 2 GB disk
- Docker + Docker Compose v2
- Port 80 (HTTP)

## Triển khai nhanh

```bash
# 1. Clone repo
git clone https://github.com/hienlt1612/logistics-workflow.git
cd logistics-workflow

# 2. Cài Docker (nếu chưa có)
./deploy.sh setup

# 3. Build frontend + khởi động
./deploy.sh build-web
./deploy.sh start
```

App chạy tại `http://<VPS_IP>` — không cần expose port riêng.

## Cấu trúc Docker

```
┌─────────────────────────────────────────┐
│  Nginx (:80)                            │
│  ├── /          → Vue SPA (web/dist/)   │
│  ├── /api/*     → Rust API (:19876)     │
│  └── /health    → Health check          │
├─────────────────────────────────────────┤
│  Rust API (lw-api)                      │
│  ├── LW_BIND_HOST=0.0.0.0              │
│  ├── LW_DB_HOST=db                     │
│  └── LW_API_TOKEN=<from .env>          │
├─────────────────────────────────────────┤
│  PostgreSQL 17 (lw-db)                  │
│  ├── Volume: pgdata (persistent)       │
│  └── Seed: db-backup/seed.sql (auto)   │
└─────────────────────────────────────────┘
```

## Các lệnh quản lý

```bash
./deploy.sh start       # Khởi động tất cả services
./deploy.sh stop        # Dừng tất cả
./deploy.sh restart     # Khởi động lại
./deploy.sh status      # Xem trạng thái
./deploy.sh logs        # Xem log real-time

./deploy.sh db-dump     # Backup database → db-backup/
./deploy.sh db-restore  # Khôi phục từ backup
./deploy.sh clean       # Xoá tất cả (mất data!)
```

## Cấu hình (.env)

Copy từ template và sửa secrets:

```bash
cp env.example .env
nano .env
```

Các biến quan trọng:
| Biến | Mặc định | Mô tả |
|------|----------|-------|
| `DB_PASSWORD` | `change-me...` | Mật khẩu PostgreSQL |
| `API_TOKEN` | `change-me...` | Token bảo vệ API write |
| `PORT` | `80` | Port HTTP public |

## Tài khoản mặc định

Sau khi deploy lần đầu, database được seed tự động với 4 users:

| Username | Password | Role |
|----------|----------|------|
| `tp_admin` | `@tp_admin123` | admin (có quyền xoá + revert) |
| `manager` | `@manager123` | manager |
| `acc` | `@acc123` | accounting |
| `logistics` | `@logistics123` | logistics |

Admin có quyền xoá shipment và revert Telex Release.
Các user khác có thể tạo/sửa/toggle checklist nhưng không xoá được.

## Backup & Restore

```bash
# Backup định kỳ (thêm vào crontab)
./deploy.sh db-dump

# Restore về backup cụ thể
./deploy.sh db-restore db-backup/dump_20260701_120000.sql

# Hoặc restore từ latest
./deploy.sh db-restore
```

Dữ liệu PostgreSQL được lưu trong Docker volume `pgdata` — tồn tại qua restart.
Backup SQL lưu trong `db-backup/`.

## HTTPS (Let's Encrypt)

Cần một domain trỏ A-record về IP VPS và mở port 80 + 443.

```bash
./deploy.sh start                       # chạy HTTP trước
./deploy.sh ssl yourdomain.com you@mail.com   # xin cert + bật HTTPS
```

Lệnh `ssl` dùng certbot webroot (không downtime): xin cert qua `/.well-known/acme-challenge/`,
sinh `nginx.ssl.active.conf` từ `nginx-ssl.conf.template` (thay DOMAIN), set `NGINX_CONF`
trong `.env`, rồi reload nginx (:80 → redirect 301 sang :443).

Gia hạn tự động (thêm vào crontab):
```
0 3 * * * cd /path/to/logistics-workflow && docker compose run --rm certbot renew && docker compose restart nginx
```

Quay lại HTTP: xoá dòng `NGINX_CONF=` trong `.env` rồi `./deploy.sh restart`.

## Cập nhật

```bash
git pull
./deploy.sh build-web   # Build lại frontend
./deploy.sh build       # Build lại Rust backend
./deploy.sh restart     # Khởi động lại với code mới
```

## Troubleshooting

```bash
# Xem log chi tiết
./deploy.sh logs

# Kiểm tra health
curl http://localhost:19876/health

# Vào PostgreSQL trực tiếp
docker compose exec db psql -U lw_user -d logistics_workflow

# Reset hoàn toàn
./deploy.sh clean
./deploy.sh start
```
