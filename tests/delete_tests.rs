use logistics_workflow::db;

const DB_URL: &str = "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow";

#[tokio::test]
async fn test_delete_existing_shipment() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");

    // Create a shipment to delete
    let input = db::queries::CreateShipmentInput {
        sc_po_id: Some("DEL-TEST".into()),
        sc_po_date: Some("2026-01-01".into()),
        sc_po_by: Some("Delete Test".into()),
        buyer_name: Some("DeleteMe".into()),
        booking_number: None,
        shipping_line: None,
        origin_port: None,
        warehouse_loc: None,
        loading_plan: None,
    };
    let created = db::queries::create_shipment(&pool, &input)
        .await
        .expect("create");
    let id = created.id;

    let deleted = db::queries::delete_shipment(&pool, id).await.expect("delete");
    assert!(deleted, "delete should return true for existing shipment");

    // Verify gone
    let check = db::queries::get_shipment(&pool, id).await.expect("get");
    assert!(check.is_none(), "shipment should no longer exist");
}

#[tokio::test]
async fn test_delete_nonexistent() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let deleted = db::queries::delete_shipment(&pool, 99999).await.expect("delete");
    assert!(!deleted, "delete should return false for non-existent shipment");
}
