mod auth_route;
mod user_route;

use crate::mw::{auth_mw, cors::create_cors};

use auth_route::routes_auth;
use axum::{middleware, Extension, Router};
use sea_orm::DatabaseConnection;
use service_utils_rs::services::jwt::Jwt;
use user_route::routes_user;

#[derive(Debug, Clone)]
pub struct ShareData {
    msg: String,
}

pub fn create_routes(database: DatabaseConnection, jwt: Jwt) -> Router {
    let cors = create_cors();

    Router::new()
        .nest("/user", routes_user())
        .route_layer(middleware::from_fn(auth_mw::auth))
        .nest("/auth", routes_auth())
        .layer(Extension(database))
        .layer(Extension(jwt))
        .layer(cors)
}
