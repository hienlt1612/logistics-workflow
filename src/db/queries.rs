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
}

pub async fn list_shipments(pool: &PgPool) -> Result<Vec<Shipment>, sqlx::Error> {
    sqlx::query_as::<_, Shipment>("SELECT * FROM shipments ORDER BY created_at DESC")
        .fetch_all(pool).await
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
            booking_number, shipping_line, origin_port, warehouse_loc, loading_plan
        ) VALUES ($1, $2, CASE WHEN $3 = '' THEN NULL ELSE $3::date END,
            $4, $5, $6, $7, $8, $9, $10)
        RETURNING id"#
    )
    .bind(&ref_num).bind(&input.sc_po_id).bind(&input.sc_po_date)
    .bind(&input.sc_po_by).bind(&input.buyer_name).bind(&input.booking_number)
    .bind(&input.shipping_line).bind(&input.origin_port).bind(&input.warehouse_loc)
    .bind(&input.loading_plan)
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
        "originals_status","originals_sent","originals_description","telex_released","status","payment_received",
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
    }
    sets.push(format!("updated_at = ${idx}")); idx += 1;
    let sql = format!(
        "UPDATE shipments SET {} WHERE id = ${idx} RETURNING *",
        sets.join(", ")
    );
    let mut query = sqlx::query_as::<_, Shipment>(&sql);
    for (col, val) in fields {
        match col.as_str() {
            "bl_received"|"charges_paid"|"co_received"|"phyto_received"|"docs_confirmed"|"telex_released"|"payment_received" => {
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
        "bl_received"|"charges_paid"|"co_received"|"phyto_received"|"docs_confirmed"|"telex_released"|"payment_received" => field,
        _ => return Err(sqlx::Error::Protocol(format!("Invalid: {field}"))),
    };
    let mut sql = format!("UPDATE shipments SET {col} = $1, updated_at = NOW()");
    if col == "telex_released" && value { sql.push_str(", status = 'TELEX_RELEASED'"); }
    sql.push_str(" WHERE id = $2 RETURNING *");
    sqlx::query_as::<_, Shipment>(&sql).bind(value).bind(id).fetch_one(pool).await
}
