use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use kernel::model::id::UserId;
use registry::AppRegistry;
use shared::error::AppResult;

use crate::model::auth::LoginRequest;

pub async fn login(
    Json(req): Json<LoginRequest>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<UserId>> {
    let LoginRequest { email, password } = req;
    registry
        .auth_repository()
        .verify_user(&email, &password)
        .await
        .map(Json)
}

pub async fn logout(
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    todo!()
}
