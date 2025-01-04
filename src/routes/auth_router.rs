use axum::{routing::post, Router};

use crate::handlers::auth_handler::login;

pub fn routes_auth() -> Router {
    Router::new()
        // .route("/create_user", post(create_user))
        .route("/login", post(login))
}
