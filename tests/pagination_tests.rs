use logistics_workflow::db;

const DB_URL: &str = "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow";

#[tokio::test]
async fn test_list_shipments_paginated() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let (rows, total) = db::queries::list_shipments_paginated(&pool, None, 1, 2)
        .await
        .expect("paginated");
    // pageSize=2 => at most 2 rows
    assert!(rows.len() <= 2);
    assert!(total >= rows.len() as i64);
}

#[tokio::test]
async fn test_list_shipments_paginated_page2() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let (page1, _) = db::queries::list_shipments_paginated(&pool, None, 1, 2)
        .await
        .expect("page1");
    let (page2, _) = db::queries::list_shipments_paginated(&pool, None, 2, 2)
        .await
        .expect("page2");
    // Pages should not overlap if there are enough shipments
    if page1.len() == 2 && !page2.is_empty() {
        let ids1: Vec<i64> = page1.iter().map(|s| s.id).collect();
        let ids2: Vec<i64> = page2.iter().map(|s| s.id).collect();
        for id in &ids2 {
            assert!(
                !ids1.contains(id),
                "Page 2 shipment id={id} should not overlap with page 1"
            );
        }
    }
}

#[tokio::test]
async fn test_list_shipments_paginated_out_of_range() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let (rows, _) = db::queries::list_shipments_paginated(&pool, None, 999, 2)
        .await
        .expect("paginated");
    assert!(rows.is_empty());
}
