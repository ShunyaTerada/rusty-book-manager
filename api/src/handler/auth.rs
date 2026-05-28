use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use hyper::StatusCode;
use kernel::model::{auth::{AccessToken, event::CreateToken}, id::UserId};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{extractor::AuthorizedUser, model::auth::{AccessTokenResponse, LoginRequest}};

pub async fn login(
    State(registry): State<AppRegistry>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AccessTokenResponse>> {
    let LoginRequest { email, password } = req;
    let user_id = registry
        .auth_repository()
        .verify_user(&email, &password)
        .await?;

    let access_token = registry
        .auth_repository()
        .create_token(CreateToken::new(user_id))
        .await?;

    Ok(Json(AccessTokenResponse {
        user_id,
        access_token: access_token.0,
    }))
}

pub async fn logout(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let user: User = registry
        .user_repository()
        .fetch_user(user)
        .await
        .map_err(AppError::UnauthorizedError);

    registry
        .auth_repository()
        .delete_token(user.access_token)
        .await
        .map(|_|StatusCode::NO_CONTENT)
        .map_err(|e| AppError::TransactionError(e))
        
            
}
