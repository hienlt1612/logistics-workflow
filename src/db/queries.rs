use chrono::{NaiveDate, DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use bigdecimal::BigDecimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Shipment {
    pub id: i64,
    pub shipment_ref: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub sc_po_id: Option<String>,
    pub sc_po_date: Option<NaiveDate>,
    pub sc_po_by: Option<String>,
    pub buyer_name: Option<String>,
    pub booking_number: Option<String>,
    pub shipping_line: Option<String>,
    pub origin_port: Option<String>,
    pub warehouse_loc: Option<String>,
    pub loading_plan: Option<String>,
    pub shipper_name: Option<String>,
    pub consignee_name: Option<String>,
    pub etd: Option<NaiveDate>,
    pub invoice_number: Option<String>,
    pub invoice_date: Option<NaiveDate>,
    pub total_value_usd: Option<BigDecimal>,
    pub drafts_date: Option<NaiveDate>,
    pub bill_of_lading: Option<String>,
    pub customs_date: Option<NaiveDate>,
    pub customs_number: Option<String>,
    pub customs_status: Option<String>,
    pub bl_received: bool,
    pub charges_paid: bool,
    pub co_received: bool,
    pub phyto_received: bool,
    pub docs_confirmed: bool,
    pub prepayment_date: Option<NaiveDate>,
    pub prepayment_amt: Option<BigDecimal>,
    pub remaining_amt: Option<BigDecimal>,
    pub originals_status: Option<String>,
    pub originals_sent: Option<NaiveDate>,
    pub originals_description: Option<String>,
    pub telex_released: bool,
    pub payment_received: bool,
    pub shipping_call_id: Option<i64>,
    pub containers_loaded: bool,
}

// ponytail: 1 row = 1 container. COUNT(*) on containers table = loaded count.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Container {
    pub id: i32,
    pub shipment_id: i64,
    pub container_number: String,
    pub seal_number: Option<String>,
    pub warehouse_name: Option<String>,
    pub loaded_date: Option<NaiveDate>,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ShippingCall {
    pub id: i64,
    pub call_ref: String,
    pub sc_po_id: Option<String>,
    pub sc_po_date: Option<NaiveDate>,
    pub sc_po_by: Option<String>,
    pub buyer_name: String,
    pub incoterms: String,
    pub product_description: Option<String>,
    pub total_containers: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CallWarehouse {
    pub id: i32,
    pub shipping_call_id: i64,
    pub warehouse_name: String,
    pub planned_containers: i32,
    pub loaded_containers: i32,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateShipmentInput {
    pub sc_po_id: Option<String>,
    pub sc_po_date: Option<String>,
    pub sc_po_by: Option<String>,
    pub buyer_name: Option<String>,
    pub booking_number: Option<String>,
    pub shipping_line: Option<String>,
    pub origin_port: Option<String>,
    pub warehouse_loc: Option<String>,
    pub loading_plan: Option<String>,
    pub shipping_call_id: Option<i64>,
}

pub async fn list_shipments(pool: &PgPool) -> Result<Vec<Shipment>, sqlx::Error> {
    sqlx::query_as::<_, Shipment>("SELECT * FROM shipments ORDER BY created_at DESC")
        .fetch_all(pool).await
}

// ponytail: one grouped query instead of N list_containers calls for the report
pub async fn count_containers_by_shipment(
    pool: &PgPool,
) -> Result<std::collections::HashMap<i64, i64>, sqlx::Error> {
    let rows: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT shipment_id, COUNT(*)::int8 FROM containers GROUP BY shipment_id",
    ).fetch_all(pool).await?;
    Ok(rows.into_iter().collect())
}

pub async fn list_shipments_paginated(
    pool: &PgPool, status_filter: Option<&str>, page: i64, page_size: i64,
) -> Result<(Vec<Shipment>, i64), sqlx::Error> {
    let offset = (page - 1) * page_size;

    // Count query
    let count_sql = if status_filter.is_some() {
        "SELECT COUNT(*) as cnt FROM shipments WHERE status = $1"
    } else {
        "SELECT COUNT(*) as cnt FROM shipments"
    };
    let total: (i64,) = if let Some(st) = status_filter {
        sqlx::query_as(count_sql).bind(st).fetch_one(pool).await?
    } else {
        sqlx::query_as(count_sql).fetch_one(pool).await?
    };

    // Data query
    let data_sql = if status_filter.is_some() {
        format!("SELECT * FROM shipments WHERE status = $1 ORDER BY created_at DESC LIMIT {page_size} OFFSET {offset}")
    } else {
        format!("SELECT * FROM shipments ORDER BY created_at DESC LIMIT {page_size} OFFSET {offset}")
    };
    let rows = if let Some(st) = status_filter {
        sqlx::query_as::<_, Shipment>(&data_sql).bind(st).fetch_all(pool).await?
    } else {
        sqlx::query_as::<_, Shipment>(&data_sql).fetch_all(pool).await?
    };

    Ok((rows, total.0))
}

pub async fn get_shipment(pool: &PgPool, id: i64) -> Result<Option<Shipment>, sqlx::Error> {
    sqlx::query_as::<_, Shipment>("SELECT * FROM shipments WHERE id = $1")
        .bind(id).fetch_optional(pool).await
}

pub async fn next_shipment_ref(pool: &PgPool) -> Result<String, sqlx::Error> {
    let year = chrono::Utc::now().format("%Y").to_string();
    let row: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(CAST(SUBSTRING(shipment_ref FROM '[0-9]+$') AS INTEGER))
         FROM shipments WHERE shipment_ref LIKE $1"
    ).bind(format!("SHP-{}-%", year)).fetch_one(pool).await?;
    let next = row.0.unwrap_or(0) as i64 + 1;
    Ok(format!("SHP-{}-{:03}", year, next))
}

pub async fn create_shipment(
    pool: &PgPool, input: &CreateShipmentInput,
) -> Result<Shipment, sqlx::Error> {
    let ref_num = next_shipment_ref(pool).await?;

    // Single INSERT with RETURNING id
    let row: (i64,) = sqlx::query_as(
        r#"INSERT INTO shipments (
            shipment_ref, sc_po_id, sc_po_date, sc_po_by, buyer_name,
            booking_number, shipping_line, origin_port, warehouse_loc, loading_plan, shipping_call_id
        ) VALUES ($1, $2, CASE WHEN $3 = '' THEN NULL ELSE $3::date END,
            $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id"#
    )
    .bind(&ref_num).bind(&input.sc_po_id).bind(&input.sc_po_date)
    .bind(&input.sc_po_by).bind(&input.buyer_name).bind(&input.booking_number)
    .bind(&input.shipping_line).bind(&input.origin_port).bind(&input.warehouse_loc)
    .bind(&input.loading_plan).bind(input.shipping_call_id)
    .fetch_one(pool)
    .await?;
    log::info!("Created shipment {} (id={})", ref_num, row.0);
    let sid: i64 = row.0;
    get_shipment(pool, sid).await.map(|s| s.expect("just inserted"))
}

pub async fn update_shipment_fields(
    pool: &PgPool, id: i64, fields: &serde_json::Map<String, serde_json::Value>,
) -> Result<Shipment, sqlx::Error> {
    const ALLOWED: &[&str] = &[
        "sc_po_id","sc_po_date","sc_po_by","buyer_name","booking_number",
        "shipping_line","origin_port","warehouse_loc","loading_plan",
        "shipper_name","consignee_name","etd","invoice_number","invoice_date",
        "total_value_usd","drafts_date","bill_of_lading",
        "customs_date","customs_number","customs_status",
        "bl_received","charges_paid","co_received","phyto_received","docs_confirmed",
        "prepayment_date","prepayment_amt","remaining_amt",
        "originals_status","originals_sent","originals_description","telex_released","status","payment_received","shipping_call_id","containers_loaded",
    ];
    let mut sets = Vec::new();
    for (col, _) in fields {
        if !ALLOWED.contains(&col.as_str()) {
            return Err(sqlx::Error::Protocol(format!("Invalid field: {col}")));
        }
    }
    let mut idx = 1;
    for col in fields.keys() {
        sets.push(format!("{col} = ${idx}")); idx += 1;
        if col == "telex_released" && fields[col].as_bool() == Some(true) {
            sets.push(format!("status = ${idx}")); idx += 1;
        }
        // ponytail: invariant telex_released ⟺ status=TELEX_RELEASED. Setting
        // status to anything else (a revert) must clear the telex flag, unless
        // the caller sets telex_released explicitly in the same payload.
        if col == "status" && fields[col].as_str() != Some("TELEX_RELEASED")
            && !fields.contains_key("telex_released") {
            sets.push(format!("telex_released = ${idx}")); idx += 1;
        }
    }
    sets.push(format!("updated_at = ${idx}")); idx += 1;
    let sql = format!(
        "UPDATE shipments SET {} WHERE id = ${idx} RETURNING *",
        sets.join(", ")
    );
    let mut query = sqlx::query_as::<_, Shipment>(&sql);
    for (col, val) in fields {
        match col.as_str() {
            "bl_received"|"charges_paid"|"co_received"|"phyto_received"|"docs_confirmed"|"telex_released"|"payment_received"|"containers_loaded" => {
                query = query.bind(val.as_bool().unwrap_or(false));
                if col == "telex_released" && val.as_bool() == Some(true) {
                    query = query.bind("TELEX_RELEASED");
                }
            }
            "total_value_usd"|"prepayment_amt"|"remaining_amt" => {
                let bd: Option<BigDecimal> = if val.is_null() { None }
                else if let Some(s) = val.as_str() { if s.is_empty() { None } else { s.parse().ok() } }
                else { val.to_string().parse().ok() };
                query = query.bind(bd);
            }
            "sc_po_date"|"etd"|"invoice_date"|"drafts_date"|"customs_date"|"prepayment_date"|"originals_sent" => {
                let s = val.as_str().unwrap_or("");
                let d: Option<chrono::NaiveDate> = if s.is_empty() { None } else { s.parse().ok() };
                query = query.bind(d);
            }
            "shipping_call_id" => {
                query = query.bind(val.as_i64());
            }
            "status" => {
                query = query.bind(val.as_str());
                if val.as_str() != Some("TELEX_RELEASED") && !fields.contains_key("telex_released") {
                    query = query.bind(false); // clear telex flag on status regression
                }
            }
            _ => { query = query.bind(val.as_str()); }
        }
    }
    query = query.bind(Utc::now()).bind(id);
    query.fetch_one(pool).await
}

pub async fn advance_status(pool: &PgPool, id: i64, new_status: &str) -> Result<Shipment, sqlx::Error> {
    const VALID: &[&str] = &["DRAFT","DOCUMENTS_READY","CUSTOMS_CLEARED","CHECKLIST_IN_PROGRESS","COMPLETE","TELEX_RELEASED"];
    if !VALID.contains(&new_status) {
        return Err(sqlx::Error::Protocol(format!("Invalid: {new_status}")));
    }
    sqlx::query_as::<_, Shipment>(
        "UPDATE shipments SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    ).bind(new_status).bind(id).fetch_one(pool).await
}

pub async fn batch_advance_status(pool: &PgPool, ids: &[i64], new_status: &str) -> Result<u64, sqlx::Error> {
    const VALID: &[&str] = &["DRAFT","DOCUMENTS_READY","CUSTOMS_CLEARED","CHECKLIST_IN_PROGRESS","COMPLETE","TELEX_RELEASED"];
    if !VALID.contains(&new_status) {
        return Err(sqlx::Error::Protocol(format!("Invalid status: {new_status}")));
    }
    if ids.is_empty() {
        return Ok(0);
    }
    // Build dynamic IN clause — safe because ids are i64
    let placeholders: Vec<String> = (1..=ids.len()).map(|i| format!("${i}")).collect();
    let sql = format!(
        "UPDATE shipments SET status = ${}, updated_at = NOW() WHERE id IN ({})",
        ids.len() + 1,
        placeholders.join(", ")
    );
    let mut query = sqlx::query(&sql);
    for id in ids { query = query.bind(id); }
    query = query.bind(new_status);
    let result = query.execute(pool).await?;
    Ok(result.rows_affected())
}

pub async fn delete_shipment(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM shipments WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// Authenticate user by username + plain-text password. Returns User on success.
pub async fn authenticate_user(pool: &PgPool, username: &str, password: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, username, password, role FROM users WHERE username = $1 AND password = $2")
        .bind(username)
        .bind(password)
        .fetch_optional(pool)
        .await
}

pub async fn set_checklist_bool(pool: &PgPool, id: i64, field: &str, value: bool) -> Result<Shipment, sqlx::Error> {
    let col = match field {
        "bl_received"|"charges_paid"|"co_received"|"phyto_received"|"docs_confirmed"|"telex_released"|"payment_received"|"containers_loaded" => field,
        _ => return Err(sqlx::Error::Protocol(format!("Invalid: {field}"))),
    };
    let mut sql = format!("UPDATE shipments SET {col} = $1, updated_at = NOW()");
    if col == "telex_released" && value { sql.push_str(", status = 'TELEX_RELEASED'"); }
    sql.push_str(" WHERE id = $2 RETURNING *");
    sqlx::query_as::<_, Shipment>(&sql).bind(value).bind(id).fetch_one(pool).await
}

// ── Shipping Calls ──
// ponytail: reuse same query pattern as shipments.

pub async fn list_shipping_calls(pool: &PgPool) -> Result<Vec<ShippingCall>, sqlx::Error> {
    sqlx::query_as::<_, ShippingCall>("SELECT * FROM shipping_calls ORDER BY created_at DESC")
        .fetch_all(pool).await
}

pub async fn get_shipping_call(pool: &PgPool, id: i64) -> Result<Option<ShippingCall>, sqlx::Error> {
    sqlx::query_as::<_, ShippingCall>("SELECT * FROM shipping_calls WHERE id = $1")
        .bind(id).fetch_optional(pool).await
}

pub async fn next_call_ref(pool: &PgPool) -> Result<String, sqlx::Error> {
    let row: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(CAST(SUBSTRING(call_ref FROM '[0-9]+$') AS INTEGER)) FROM shipping_calls WHERE call_ref LIKE 'CALL-%'"
    ).fetch_one(pool).await?;
    let next = row.0.unwrap_or(0) as i64 + 1;
    Ok(format!("CALL-{}-{:03}", chrono::Utc::now().format("%Y"), next))
}

pub async fn create_shipping_call(pool: &PgPool, call: &CreateShippingCallInput) -> Result<ShippingCall, sqlx::Error> {
    let ref_num = next_call_ref(pool).await?;
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO shipping_calls (call_ref, sc_po_id, sc_po_date, sc_po_by, buyer_name, incoterms, product_description, total_containers)
         VALUES ($1,$2,CASE WHEN $3='' THEN NULL ELSE $3::date END,$4,$5,$6,$7,$8) RETURNING id"
    ).bind(&ref_num).bind(&call.sc_po_id).bind(&call.sc_po_date)
     .bind(&call.sc_po_by).bind(&call.buyer_name).bind(&call.incoterms)
     .bind(&call.product_description).bind(call.total_containers)
     .fetch_one(pool).await?;
    get_shipping_call(pool, row.0).await.map(|s| s.expect("just inserted"))
}

// ponytail: update editable fields only — buyer, incoterms, product, sc_po, total_containers
pub async fn update_shipping_call(pool: &PgPool, id: i64, input: &UpdateShippingCallInput) -> Result<Option<ShippingCall>, sqlx::Error> {
    sqlx::query(
        "UPDATE shipping_calls SET sc_po_id=$1, sc_po_date=CASE WHEN $2='' THEN NULL ELSE $2::date END, sc_po_by=$3, buyer_name=$4, incoterms=$5, product_description=$6, total_containers=$7, status=COALESCE($9, status), updated_at=NOW() WHERE id=$8"
    ).bind(&input.sc_po_id).bind(&input.sc_po_date).bind(&input.sc_po_by)
     .bind(&input.buyer_name).bind(&input.incoterms).bind(&input.product_description)
     .bind(input.total_containers).bind(id).bind(&input.status)
     .execute(pool).await?;
    get_shipping_call(pool, id).await
}

// ponytail: delete call — shipments set to NULL (ON DELETE SET NULL), warehouses cascade
pub async fn delete_shipping_call(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("DELETE FROM shipping_calls WHERE id = $1").bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

// ponytail: sync call status based on linked shipment count
pub async fn sync_call_status(pool: &PgPool, call_id: i64) -> Result<(), sqlx::Error> {
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM shipments WHERE shipping_call_id = $1"
    ).bind(call_id).fetch_one(pool).await?;
    let new_status = if count > 0 { "ON_LOADING" } else { "OPEN" };
    sqlx::query(
        "UPDATE shipping_calls SET status = $1, updated_at = NOW() WHERE id = $2 AND status NOT IN ('CLOSED')"
    ).bind(new_status).bind(call_id).execute(pool).await?;
    Ok(())
}

// ── Call Warehouses ──

pub async fn list_call_warehouses(pool: &PgPool, call_id: i64) -> Result<Vec<CallWarehouse>, sqlx::Error> {
    // ponytail: compute loaded_containers dynamically from containers table
    sqlx::query_as::<_, CallWarehouse>(
        "SELECT cw.id, cw.shipping_call_id, cw.warehouse_name, cw.planned_containers,
                COALESCE(ct.loaded, 0)::int as loaded_containers, cw.status, cw.notes
         FROM call_warehouses cw
         LEFT JOIN (
             SELECT c.warehouse_name, COUNT(*) as loaded
             FROM containers c
             JOIN shipments s ON s.id = c.shipment_id
             WHERE s.shipping_call_id = $1
             GROUP BY c.warehouse_name
         ) ct ON ct.warehouse_name = cw.warehouse_name
         WHERE cw.shipping_call_id = $1
         ORDER BY cw.id"
    ).bind(call_id).fetch_all(pool).await
}

pub async fn create_call_warehouse(pool: &PgPool, w: &CreateWarehouseInput) -> Result<CallWarehouse, sqlx::Error> {
    sqlx::query_as::<_, CallWarehouse>(
        "INSERT INTO call_warehouses (shipping_call_id, warehouse_name, planned_containers) VALUES ($1,$2,$3) RETURNING *"
    ).bind(w.shipping_call_id).bind(&w.warehouse_name).bind(w.planned_containers)
     .fetch_one(pool).await
}

// ponytail: delete all warehouses for a call (used in update to replace atomically)
pub async fn delete_call_warehouses(pool: &PgPool, call_id: i64) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM call_warehouses WHERE shipping_call_id = $1")
        .bind(call_id).execute(pool).await.map(|r| r.rows_affected())
}

// ponytail: check warehouse capacity for a call — reject if loaded >= planned
pub async fn check_warehouse_capacity(pool: &PgPool, call_id: i64, warehouse_name: &str) -> Result<(i32, i32), sqlx::Error> {
    let row: (i32, i32) = sqlx::query_as(
        "SELECT cw.planned_containers, COALESCE(ct.loaded, 0)::int
         FROM call_warehouses cw
         LEFT JOIN (
             SELECT c.warehouse_name, COUNT(*) as loaded
             FROM containers c JOIN shipments s ON s.id = c.shipment_id
             WHERE s.shipping_call_id = $1 AND c.warehouse_name = $2
             GROUP BY c.warehouse_name
         ) ct ON ct.warehouse_name = cw.warehouse_name
         WHERE cw.shipping_call_id = $1 AND cw.warehouse_name = $2"
    ).bind(call_id).bind(warehouse_name).fetch_one(pool).await?;
    Ok(row)
}

// ── Containers ──

pub async fn list_containers(pool: &PgPool, shipment_id: i64) -> Result<Vec<Container>, sqlx::Error> {
    sqlx::query_as::<_, Container>("SELECT * FROM containers WHERE shipment_id = $1 ORDER BY id")
        .bind(shipment_id).fetch_all(pool).await
}

pub async fn create_container(pool: &PgPool, c: &CreateContainerInput) -> Result<Container, sqlx::Error> {
    sqlx::query_as::<_, Container>(
        "INSERT INTO containers (shipment_id, container_number, seal_number, warehouse_name, loaded_date) VALUES ($1,$2,$3,$4,$5::date) RETURNING *"
    ).bind(c.shipment_id).bind(&c.container_number).bind(&c.seal_number)
     .bind(&c.warehouse_name).bind(&c.loaded_date)
     .fetch_one(pool).await
}

// ponytail: delete + update — same pattern as call warehouses
pub async fn delete_container(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("DELETE FROM containers WHERE id = $1").bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}

pub async fn update_container(pool: &PgPool, id: i64, c: &UpdateContainerInput) -> Result<Option<Container>, sqlx::Error> {
    sqlx::query(
        "UPDATE containers SET container_number=$1, seal_number=$2, warehouse_name=$3, loaded_date=$4::date WHERE id=$5"
    ).bind(&c.container_number).bind(&c.seal_number).bind(&c.warehouse_name)
     .bind(&c.loaded_date).bind(id)
     .execute(pool).await?;
    sqlx::query_as::<_, Container>("SELECT * FROM containers WHERE id = $1").bind(id).fetch_optional(pool).await
}

// ── Input structs ──
// ponytail: reusing existing Deserialize pattern from CreateShipmentInput.

#[derive(Debug, Deserialize)]
pub struct CreateShippingCallInput {
    pub sc_po_id: Option<String>,
    pub sc_po_date: Option<String>,
    pub sc_po_by: Option<String>,
    pub buyer_name: String,
    pub incoterms: String,
    pub product_description: Option<String>,
    pub total_containers: i32,
    // ponytail: warehouses inline — no shipping_call_id needed (backend fills from call.id)
    pub warehouses: Option<Vec<InlineWarehouse>>,
}

#[derive(Debug, Deserialize)]
pub struct InlineWarehouse {
    pub warehouse_name: String,
    pub planned_containers: i32,
}

// ponytail: UpdateShippingCallInput — editable fields + optional warehouse replace
#[derive(Debug, Deserialize)]
pub struct UpdateShippingCallInput {
    pub sc_po_id: Option<String>,
    pub sc_po_date: Option<String>,
    pub sc_po_by: Option<String>,
    pub buyer_name: String,
    pub incoterms: String,
    pub product_description: Option<String>,
    pub total_containers: i32,
    pub warehouses: Option<Vec<InlineWarehouse>>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWarehouseInput {
    pub shipping_call_id: i64,
    pub warehouse_name: String,
    pub planned_containers: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateContainerInput {
    pub shipment_id: i64,
    pub container_number: String,
    pub seal_number: Option<String>,
    pub warehouse_name: Option<String>,
    pub loaded_date: Option<String>,
}

// ponytail: update — same fields without shipment_id (immutable)
#[derive(Debug, Deserialize)]
pub struct UpdateContainerInput {
    pub container_number: String,
    pub seal_number: Option<String>,
    pub warehouse_name: Option<String>,
    pub loaded_date: Option<String>,
}
