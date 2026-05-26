use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

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

#[cfg(test)]
mod tests {
    use super::*;
    use adapter::{database::ConnectionPool, redis::RedisClient};
    use shared::config::AppConfig;
    use shared::config::RedisConfig;
    use std::sync::Arc;

    #[tokio::test]
    async fn health_check_works() {
        let status_code = health_check().await;
        assert_eq!(status_code, StatusCode::OK);
    }

    #[ignore]
    #[sqlx::test]
    async fn health_check_db_works(pool: sqlx::PgPool, redis_client: &RedisConfig) {
        let connection_pool = ConnectionPool::new(pool);
        let connection_redis_client = Arc::new(RedisClient::new(redis_client).unwrap());
        let connection_app_config = AppConfig::new().unwrap();

        let registry = AppRegistry::new(
            connection_pool,
            connection_redis_client,
            connection_app_config,
        );

        let status_code = health_check_db(State(registry)).await;
        assert_eq!(status_code, StatusCode::OK);
    }
}
