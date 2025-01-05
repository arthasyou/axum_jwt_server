use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::user_handler::info;

pub fn routes_user() -> Router {
    Router::new().route("/info", get(info))
}
