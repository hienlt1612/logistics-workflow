use logistics_workflow::db;

const DB_URL: &str = "postgres://mim_dev:logistics2026@127.0.0.1:5432/logistics_workflow";

#[tokio::test]
async fn test_authenticate_user_valid() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let result = db::queries::authenticate_user(&pool, "tp_admin", "@tp_admin123")
        .await
        .expect("auth");
    assert!(result.is_some());
    let user = result.unwrap();
    assert_eq!(user.username, "tp_admin");
    assert_eq!(user.role, "admin");
}

#[tokio::test]
async fn test_authenticate_user_invalid() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let result = db::queries::authenticate_user(&pool, "tp_admin", "wrong_password")
        .await
        .expect("auth");
    assert!(result.is_none());
}

#[tokio::test]
async fn test_authenticate_user_nonexistent() {
    let pool = sqlx::PgPool::connect(DB_URL).await.expect("connect");
    let result = db::queries::authenticate_user(&pool, "no_such_user", "any_password")
        .await
        .expect("auth");
    assert!(result.is_none());
}
