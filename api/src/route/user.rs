use axum::{
    Router,
    routing::{delete, get, put},
};

use registry::AppRegistry;

use crate::handler::user::{
    change_password, change_role, delete_user, get_current_user, list_user, register_user,
};

pub fn build_user_repository() -> Router<AppRegistry> {
    let users_routers = Router::new()
        .route("/me", get(get_current_user))
        .route("/me/password", put(change_password))
        .route("/", get(list_user).post(register_user))
        .route("/{user_id}", delete(delete_user))
        .route("/{user_id}", put(change_role));

    Router::new().nest("/users", users_routers)
}
