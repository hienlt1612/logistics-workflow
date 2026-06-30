use logistics_workflow::db;
use serde_json::json;

const DB_URL: &str = "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow";

#[tokio::test]
async fn test_update_shipment_fields() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let all = db::queries::list_shipments(&pool).await.expect("list");
    let s = all.first().expect("need at least one shipment");

    let original = s.buyer_name.clone();
    let new_name = format!("UpdatedBuyer-{}", chrono::Utc::now().timestamp());

    let fields: serde_json::Map<String, serde_json::Value> = {
        let mut m = serde_json::Map::new();
        m.insert("buyer_name".into(), json!(new_name));
        m
    };

    let updated = db::queries::update_shipment_fields(&pool, s.id, &fields)
        .await
        .expect("update");
    assert_eq!(updated.buyer_name.as_deref(), Some(new_name.as_str()));

    // Restore original
    if let Some(ref orig) = original {
        let mut restore = serde_json::Map::new();
        restore.insert("buyer_name".into(), json!(orig));
        db::queries::update_shipment_fields(&pool, s.id, &restore)
            .await
            .ok();
    }
}

#[tokio::test]
async fn test_update_invalid_field_rejected() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let all = db::queries::list_shipments(&pool).await.expect("list");
    let s = all.first().expect("need at least one shipment");

    let mut fields = serde_json::Map::new();
    fields.insert("nonexistent_field".into(), json!("value"));
    let result = db::queries::update_shipment_fields(&pool, s.id, &fields).await;
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid field"), "expected 'Invalid field', got: {err}");
}

#[tokio::test]
async fn test_update_bigdecimal_fields() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let all = db::queries::list_shipments(&pool).await.expect("list");
    let s = all.first().expect("need at least one shipment");

    let mut fields = serde_json::Map::new();
    fields.insert("total_value_usd".into(), json!("99999.99"));
    fields.insert("prepayment_amt".into(), json!("5000.00"));
    let updated = db::queries::update_shipment_fields(&pool, s.id, &fields)
        .await
        .expect("update");
    assert!(updated.total_value_usd.is_some(), "total_value_usd should be set");
    assert!(updated.prepayment_amt.is_some(), "prepayment_amt should be set");
}

#[tokio::test]
async fn test_update_date_fields() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let all = db::queries::list_shipments(&pool).await.expect("list");
    let s = all.first().expect("need at least one shipment");

    let mut fields = serde_json::Map::new();
    fields.insert("etd".into(), json!("2026-12-25"));
    fields.insert("customs_date".into(), json!("2026-11-15"));
    let updated = db::queries::update_shipment_fields(&pool, s.id, &fields)
        .await
        .expect("update");
    assert!(updated.etd.is_some(), "etd should be set");
    assert!(updated.customs_date.is_some(), "customs_date should be set");
}

#[tokio::test]
async fn test_update_status_field() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let all = db::queries::list_shipments(&pool).await.expect("list");
    // Find a DRAFT shipment to advance
    let draft = all
        .iter()
        .find(|s| s.status == "DRAFT")
        .expect("need a DRAFT shipment");

    let mut fields = serde_json::Map::new();
    fields.insert("status".into(), json!("DOCUMENTS_READY"));
    let updated = db::queries::update_shipment_fields(&pool, draft.id, &fields)
        .await
        .expect("update");
    assert_eq!(updated.status, "DOCUMENTS_READY");

    // Reset back to DRAFT
    let mut reset = serde_json::Map::new();
    reset.insert("status".into(), json!("DRAFT"));
    db::queries::update_shipment_fields(&pool, draft.id, &reset)
        .await
        .ok();
}
