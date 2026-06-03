use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use registry::AppRegistry;

use kernel::model::{
    id::UserId,
    role::Role,
    user::{
        User,
        event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole},
    },
};

use crate::{
    extractor::AuthorizedUser,
    model::user::{
        CreateUserRequest, RoleName, UpdateUserPasswordRequest,
        UpdateUserPasswordRequestWithUserId, UpdateUserRoleRequest,
        UpdateUserRoleRequestWithUserId, UserResponse, UsersResponse,
    },
};

use shared::error::{AppError, AppResult};

pub async fn get_current_user(user: AuthorizedUser) -> AppResult<Json<UserResponse>> {
    Ok(Json(UserResponse::from(user.user)))
}

pub async fn list_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<UsersResponse>> {
    if !user.id_admin() {
        return Err(AppError::UnauthorizedError);
    }

    let item: Vec<UserResponse> = registry
        .user_repository()
        .find_all()
        .await?
        .into_iter()
        .map(UserResponse::from)
        .collect();

    Ok(Json(UsersResponse { item }))
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
