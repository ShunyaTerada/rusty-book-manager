use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use garde::Validate;
use registry::AppRegistry;

use kernel::model::{id::UserId, user::event::DeleteUser};

use crate::{
    extractor::AuthorizedUser,
    model::user::{
        CreateUserRequest, UpdateUserPasswordRequest, UpdateUserPasswordRequestWithUserId,
        UpdateUserRoleRequest, UpdateUserRoleRequestWithUserId, UserResponse, UsersResponse,
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
        return Err(AppError::ForbiddenOperation);
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

pub async fn register_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    req.validate(&())?;

    let new_user = registry.user_repository().create(req.into()).await?;

    Ok(Json(new_user.into()))
}

pub async fn change_role(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Path(user_id): Path<UserId>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
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
    req.validate(&())?;

    let event = UpdateUserPasswordRequestWithUserId::new(user.id(), req);

    registry
        .user_repository()
        .update_password(event.into())
        .await?;

    Ok(StatusCode::OK)
}

pub async fn delete_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Path(user_id): Path<UserId>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    registry
        .user_repository()
        .delete(DeleteUser { user_id })
        .await?;

    Ok(StatusCode::OK)
}
