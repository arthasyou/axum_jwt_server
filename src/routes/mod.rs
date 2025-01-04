mod auth_router;

use crate::mw::cors::create_cors;

use auth_router::routes_auth;
use axum::{Extension, Router};
use sea_orm::DatabaseConnection;
use service_utils_rs::services::jwt::Jwt;

#[derive(Debug, Clone)]
pub struct ShareData {
    msg: String,
}

pub fn create_routes(database: DatabaseConnection, jwt: Jwt) -> Router {
    let cors = create_cors();

    Router::new()
        .nest("/auth", routes_auth())
        .layer(Extension(database))
        .layer(Extension(jwt))
        .layer(cors)
}
