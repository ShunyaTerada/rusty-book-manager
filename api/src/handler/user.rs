use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use registry::AppRegistry;

use kernel::model::{
    id::UserId,
    role::Role,
    user::event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole},
};
use utoipa::Path;

use crate::{
    extractor::AuthorizedUser,
    model::user::{
        CreateUserRequest, RoleName, UpdateUserPasswordRequest,
        UpdateUserPasswordRequestWithUserId, UpdateUserRoleRequest,
        UpdateUserRoleRequestWithUserId, UserResponse, UsersResponse,
    },
};

use shared::error::{AppError, AppResult};

pub async fn get_current_user(
    State(registry): State<AppRegistry>,
    Path(user_id): Path<UserId>,
) -> AppResult<Json<UserResponse>> {
    registry
        .user_repository()
        .find_current_user(user_id)
        .await?
        .map(|e| Json(UserResponse::from(e)))
        .and_then(|opt| match opt {
            Some(e) => Ok(e),
            None => AppError::EntityNotFound("アカウントが見つかりません".to_string()),
        })
}

pub async fn list_user() {
    todo!()
}

pub async fn register_user() {
    todo!()
}

pub async fn change_role() {
    todo!()
}

pub async fn change_password() {
    todo!()
}

pub async fn delete_user() {
    todo!()
}
