use registry::AppRegistry
use axum::{
    http::{
        status,
        StatusCode,
    },
    Json,
    RequestPartsExt,
    
};
use axum_extra::{
    
};

use kernel::model::{
    role::Role,
    id::UserId,
    user::{
        User,
        event::{
            CreateUser,
            UpdateUserRole,
            UpdateUserPassword,
            DeleteUser,
        }
    },
};

use crate::{
    extractor::AuthorizedUser,
    model::user::{
        RoleName,
        UserResponse,
        UsersResponse,
        CreateUserRequest,
        UpdateUserRoleRequest,
        UpdateUserRoleRequestWithUserId,
        UpdateUserPasswordRequest,
        UpdateUserPasswordRequestWithUserId,
    }
};

use shared::error::{AppResult, AppError};



