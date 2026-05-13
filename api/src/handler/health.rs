use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;
use crate::adapter::src::database::ConnectionPool;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}

#[sqlx::test]
async fn health_check_db_works(pool: sqlx::PgPool) {
    let connection_pool = ConnectionPool::new(pool);

    let registry = AppRegistry::new(connection_pool);

    let status_code = health_check_db(State(registry)).await;
    assert_eq!(status_code, StatusCode::OK);
}
