-- Schema migration: Shipping Call hierarchy
-- 1 Shipping Call → N Shipments (bookings)
-- 1 Shipment (booking) → N Containers
-- Warehouse tracked per container (not per shipment)

-- Phase 1: shipping_calls (parent order)
CREATE TABLE IF NOT EXISTS shipping_calls (
    id              BIGSERIAL PRIMARY KEY,
    call_ref        VARCHAR(30) UNIQUE NOT NULL,
    sc_po_id        VARCHAR(30),
    sc_po_date      DATE,
    sc_po_by        VARCHAR(60),
    buyer_name      VARCHAR(100) NOT NULL,
    incoterms       VARCHAR(3) NOT NULL DEFAULT 'FOB',
    product_description TEXT,
    total_quantity  NUMERIC(12,2),
    quantity_unit   VARCHAR(20) DEFAULT 'tons',
    status          VARCHAR(30) NOT NULL DEFAULT 'OPEN',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Phase 2: call_warehouses (per-call warehouse loading plan)
CREATE TABLE IF NOT EXISTS call_warehouses (
    id                SERIAL PRIMARY KEY,
    shipping_call_id  BIGINT NOT NULL REFERENCES shipping_calls(id) ON DELETE CASCADE,
    warehouse_name    VARCHAR(120) NOT NULL,
    planned_quantity  NUMERIC(12,2),
    loaded_quantity   NUMERIC(12,2) DEFAULT 0,
    status            VARCHAR(20) DEFAULT 'PENDING',
    notes             TEXT
);

-- Phase 3: modify shipments
ALTER TABLE shipments ADD COLUMN IF NOT EXISTS shipping_call_id BIGINT REFERENCES shipping_calls(id) ON DELETE SET NULL;

-- Drop old single-container columns (data loss: containers info moves to new table)
-- Uncomment when ready to migrate:
-- ALTER TABLE shipments DROP COLUMN IF EXISTS container_number;
-- ALTER TABLE shipments DROP COLUMN IF EXISTS seal_number;

-- Phase 4: containers per booking (1 booking = N containers from different warehouses)
CREATE TABLE IF NOT EXISTS containers (
    id                SERIAL PRIMARY KEY,
    shipment_id       BIGINT NOT NULL REFERENCES shipments(id) ON DELETE CASCADE,
    container_number  VARCHAR(20) NOT NULL,
    seal_number       VARCHAR(20),
    warehouse_name    VARCHAR(120),
    weight_kg         NUMERIC(10,2),
    cbm               NUMERIC(8,3),
    loaded_quantity   NUMERIC(12,2),
    status            VARCHAR(20) DEFAULT 'PENDING',
    notes             TEXT
);

-- ponytail: no container_detail or multi-warehouse-per-container.
-- Add when one physical container loads from >1 warehouse.
