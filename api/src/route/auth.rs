use axum::{Router, routing::post};

use registry::AppRegistry;

use crate::handler::auth::login;

pub fn buid_auth_router() -> Router<AppRegistry> {
    Router::new().route("/login{user_id}", post(login))
}
