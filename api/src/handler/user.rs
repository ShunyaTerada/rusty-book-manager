use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use hyper::StatusCode;
use registry::AppRegistry;

use kernel::model::{id::UserId, role::Role, user::User};

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
    if !user.is_admin() {
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

pub async fn change_role(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Path(user_id): Path<UserId>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {
        return Err(AppError::UnauthorizedError);
    }

    let event = UpdateUserRoleRequestWithUserId::new(user_id, req);

    registry.user_repository().update_role(event.into()).await?;

    Ok(StatusCode::OK)
}

pub async fn change_password(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserPasswordRequest>,
) -> AppResult<StatusCode> {
    let event = UpdateUserPasswordRequestWithUserId::new(user.id(), req);

    registry
        .user_repository()
        .update_password(event.into())
        .await?;

    Ok(StatusCode::OK)
}

pub async fn delete_user() {
    todo!()
}
