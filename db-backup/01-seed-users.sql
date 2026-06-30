-- Logistics Workflow — Seed Data for Fresh Deploy
-- Auto-executed by PostgreSQL on first container start (via /docker-entrypoint-initdb.d/)
-- Creates tables, indexes, seed users, and sample shipments.

-- Users
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(100) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO users (username, password, role) VALUES
    ('tp_admin',   '@tp_admin123',   'admin'),
    ('manager',    '@manager123',    'manager'),
    ('acc',        '@acc123',        'accounting'),
    ('logistics',  '@logistics123',  'logistics')
ON CONFLICT (username) DO NOTHING;

-- Shipments (schema auto-created by Rust backend on startup)
-- Sample data for demo — optional, can be empty
