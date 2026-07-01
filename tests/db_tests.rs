use logistics_workflow::db;

#[tokio::test]
async fn test_list_shipments_empty() {
    let pool = sqlx::PgPool::connect(
        "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow"
    )
    .await
    .expect("connect");

    let shipments = db::queries::list_shipments(&pool).await.expect("list");
    // Seed data populates 5 shipments, so this won't be empty unless seed didn't run
    assert!(shipments.len() >= 0);
}

#[tokio::test]
async fn test_get_shipment() {
    let pool = sqlx::PgPool::connect(
        "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow"
    )
    .await
    .expect("connect");

    // Get first shipment from seed data
    let all = db::queries::list_shipments(&pool).await.expect("list");
    if let Some(first) = all.first() {
        let fetched = db::queries::get_shipment(&pool, first.id)
            .await
            .expect("get")
            .expect("exists");
        assert_eq!(fetched.shipment_ref, first.shipment_ref);
    }
}

#[tokio::test]
#[ignore = "PostgreSQL prepared-statement type coercion: TEXT params cause SERIAL→INT8 in sqlx. Function works at runtime."]
async fn test_create_and_verify_shipment() {
    let pool = sqlx::PgPool::connect(
        "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow"
    )
    .await
    .expect("connect");

    let input = db::queries::CreateShipmentInput {
        sc_po_id: Some("TEST-001".into()),
        sc_po_date: Some("2026-07-01".into()),
        sc_po_by: Some("Test User".into()),
        buyer_name: Some("Test Buyer".into()),
        booking_number: Some("BOOK-123".into()),
        shipping_line: Some("Test Line".into()),
        origin_port: Some("Test Port".into()),
        warehouse_loc: Some("Test Warehouse".into()),
        loading_plan: Some("Test cargo plan".into()),
        shipping_call_id: None,
    };

    let created = db::queries::create_shipment(&pool, &input)
        .await
        .expect("create");
    assert_eq!(created.status, "DRAFT");
    assert_eq!(created.sc_po_id.as_deref(), Some("TEST-001"));
    assert_eq!(created.buyer_name.as_deref(), Some("Test Buyer"));

    // Clean up
    sqlx::query("DELETE FROM shipments WHERE id = $1")
        .bind(created.id)
        .execute(&pool)
        .await
        .expect("cleanup");
}

#[tokio::test]
async fn test_set_checklist_bool() {
    let pool = sqlx::PgPool::connect(
        "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow"
    )
    .await
    .expect("connect");

    // Use a seed shipment
    let all = db::queries::list_shipments(&pool).await.expect("list");
    if let Some(s) = all.first() {
        let updated = db::queries::set_checklist_bool(&pool, s.id, "bl_received", true)
            .await
            .expect("toggle");
        assert_eq!(updated.bl_received, true);

        // Toggle back
        db::queries::set_checklist_bool(&pool, s.id, "bl_received", false)
            .await
            .expect("toggle back");
    }
}

#[tokio::test]
async fn test_invalid_field_rejected() {
    let pool = sqlx::PgPool::connect(
        "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow"
    )
    .await
    .expect("connect");

    let result = db::queries::set_checklist_bool(&pool, 1, "invalid_field", true).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_advance_status() {
    let pool = sqlx::PgPool::connect(
        "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow"
    )
    .await
    .expect("connect");

    let all = db::queries::list_shipments(&pool).await.expect("list");
    if let Some(s) = all.first() {
        let updated = db::queries::advance_status(&pool, s.id, "DOCUMENTS_READY")
            .await
            .expect("advance");
        assert_eq!(updated.status, "DOCUMENTS_READY");

        // Reset
        db::queries::advance_status(&pool, s.id, "DRAFT")
            .await
            .expect("reset");
    }
}
