use axum::Extension;
use axum::{http::StatusCode, Json};
use bcrypt::{hash, DEFAULT_COST};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use service_utils_rs::services::jwt::Jwt;

use crate::models::auth_model::{LoginRequest, LoginRespon, SignupRequest};
use crate::models::{CommonResponse, IntoSerdeJson};
use crate::orm::prelude::User;
use crate::orm::user;

pub async fn info(Extension(id): Extension<String>) -> Result<Json<CommonResponse>, StatusCode> {
    println!("info {:?}", id);
    let mut res = CommonResponse::default();
    Ok(Json(res))
}
