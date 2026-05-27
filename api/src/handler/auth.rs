use anyhow::Ok;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use kernel::model::{
    auth::{AccessToken, event::CreateToken},
    id::UserId,
};
use registry::AppRegistry;
use shared::error::AppResult;

use crate::model::auth::{AccessTokenResponse, LoginRequest};

pub async fn login(
    Json(req): Json<LoginRequest>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<AccessTokenResponse>> {
    let LoginRequest { email, password } = req;
    let user_id = registry
        .auth_repository()
        .verify_user(&email, &password)
        .await?;

    registry
        .auth_repository()
        .create_token(CreateToken::new(user_id))
        .await
        .map(|v| {
            Json(AccessTokenResponse {
                user_id,
                access_token: v.0,
            })
        })
}

pub async fn logout(
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    todo!()
}
