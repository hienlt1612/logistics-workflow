use sqlx::PgPool;

pub async fn ensure_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(CREATE_SHIPPING_CALLS).execute(pool).await?;
    sqlx::query(CREATE_CALL_WAREHOUSES).execute(pool).await?;
    sqlx::query(CREATE_SHIPMENTS).execute(pool).await?;
    sqlx::query(CREATE_CONTAINERS).execute(pool).await?;
    log::info!("Database schema verified");
    Ok(())
}

// ponytail: quantities are integer container counts. 1 container = 1 row in containers table.
const CREATE_SHIPPING_CALLS: &str = r#"
CREATE TABLE IF NOT EXISTS shipping_calls (
    id              BIGSERIAL PRIMARY KEY,
    call_ref        VARCHAR(30) UNIQUE NOT NULL,
    sc_po_id        VARCHAR(30),
    sc_po_date      DATE,
    sc_po_by        VARCHAR(60),
    buyer_name      VARCHAR(100) NOT NULL,
    incoterms       VARCHAR(3) NOT NULL DEFAULT 'FOB',
    product_description TEXT,
    total_containers INTEGER NOT NULL DEFAULT 0,
    status          VARCHAR(30) NOT NULL DEFAULT 'OPEN',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
"#;

const CREATE_CALL_WAREHOUSES: &str = r#"
CREATE TABLE IF NOT EXISTS call_warehouses (
    id                SERIAL PRIMARY KEY,
    shipping_call_id  BIGINT NOT NULL REFERENCES shipping_calls(id) ON DELETE CASCADE,
    warehouse_name    VARCHAR(120) NOT NULL,
    planned_containers INTEGER NOT NULL DEFAULT 0,
    loaded_containers  INTEGER NOT NULL DEFAULT 0,
    status            VARCHAR(20) DEFAULT 'PENDING',
    notes             TEXT
);
"#;

// ponytail: 1 shipment = 1 booking. 1 booking = N containers (different warehouses).
const CREATE_SHIPMENTS: &str = r#"
CREATE TABLE IF NOT EXISTS shipments (
    id              BIGSERIAL PRIMARY KEY,
    shipment_ref    VARCHAR(30) UNIQUE NOT NULL,
    status          VARCHAR(30) NOT NULL DEFAULT 'DRAFT',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    sc_po_id        VARCHAR(30),
    sc_po_date      DATE,
    sc_po_by        VARCHAR(60),
    buyer_name      VARCHAR(80),
    booking_number  VARCHAR(30),
    shipping_line   VARCHAR(50),
    origin_port     VARCHAR(80),
    warehouse_loc   VARCHAR(120),
    loading_plan    TEXT,
    shipper_name    VARCHAR(100),
    consignee_name  VARCHAR(100),
    etd             DATE,
    invoice_number  VARCHAR(30),
    invoice_date    DATE,
    total_value_usd NUMERIC(12,2),
    drafts_date     DATE,
    bill_of_lading  VARCHAR(30),
    customs_date    DATE,
    customs_number  VARCHAR(30),
    customs_status  VARCHAR(10),
    bl_received     BOOLEAN NOT NULL DEFAULT false,
    charges_paid    BOOLEAN NOT NULL DEFAULT false,
    co_received     BOOLEAN NOT NULL DEFAULT false,
    phyto_received  BOOLEAN NOT NULL DEFAULT false,
    docs_confirmed  BOOLEAN NOT NULL DEFAULT false,
    prepayment_date DATE,
    prepayment_amt  NUMERIC(12,2),
    remaining_amt   NUMERIC(12,2),
    originals_status VARCHAR(20),
    originals_sent  DATE,
    originals_description VARCHAR(256),
    telex_released  BOOLEAN NOT NULL DEFAULT false,
    payment_received BOOLEAN NOT NULL DEFAULT false,
    shipping_call_id BIGINT REFERENCES shipping_calls(id) ON DELETE SET NULL
);
"#;

// ponytail: 1 row = 1 container. warehouse tracks source. weight/cbm per container.
const CREATE_CONTAINERS: &str = r#"
CREATE TABLE IF NOT EXISTS containers (
    id                SERIAL PRIMARY KEY,
    shipment_id       BIGINT NOT NULL REFERENCES shipments(id) ON DELETE CASCADE,
    container_number  VARCHAR(20) NOT NULL,
    seal_number       VARCHAR(20),
    warehouse_name    VARCHAR(120),
    weight_kg         NUMERIC(10,2),
    cbm               NUMERIC(8,3),
    status            VARCHAR(20) DEFAULT 'PENDING',
    notes             TEXT
);
"#;

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_schema_creation() {
        let pool = PgPool::connect("postgres://mim_dev:***@127.0.0.1:5432/logistics_workflow")
            .await.expect("connect");
        ensure_schema(&pool).await.expect("schema");
        let row: (String,) = sqlx::query_as(
            "SELECT table_name FROM information_schema.tables WHERE table_name='containers'"
        ).fetch_one(&pool).await.expect("check");
        assert_eq!(row.0, "containers");
    }
}
