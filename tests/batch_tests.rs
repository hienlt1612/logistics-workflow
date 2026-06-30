use logistics_workflow::db;

const DB_URL: &str = "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow";

#[tokio::test]
async fn test_batch_advance_status() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let all = db::queries::list_shipments(&pool).await.expect("list");

    // Find DRAFT shipments to test with
    let drafts: Vec<&db::queries::Shipment> =
        all.iter().filter(|s| s.status == "DRAFT").collect();
    if drafts.len() >= 2 {
        let ids: Vec<i64> = drafts.iter().take(2).map(|s| s.id).collect();
        let count = db::queries::batch_advance_status(&pool, &ids, "DOCUMENTS_READY")
            .await
            .expect("batch");
        assert_eq!(count, 2, "expected 2 rows affected");

        // Reset
        db::queries::batch_advance_status(&pool, &ids, "DRAFT")
            .await
            .ok();
    } else {
        // Fallback: test with whatever we have
        let ids: Vec<i64> = all.iter().take(1).map(|s| s.id).collect();
        let original = all.first().unwrap().status.clone();
        let count = db::queries::batch_advance_status(&pool, &ids, "DOCUMENTS_READY")
            .await
            .expect("batch");
        assert!(count >= 1);

        // Reset
        db::queries::batch_advance_status(&pool, &ids, &original)
            .await
            .ok();
    }
}

#[tokio::test]
async fn test_batch_invalid_status() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let result = db::queries::batch_advance_status(&pool, &[1], "INVALID_STATUS").await;
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid"), "expected 'Invalid', got: {err}");
}
