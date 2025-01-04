use axum::Extension;
use axum::{http::StatusCode, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use service_utils_rs::services::jwt::Jwt;

use crate::models::auth_model::{LoginRequest, LoginRespon};
use crate::models::{CommonResponse, IntoSerdeJson};
use crate::orm::prelude::User;
use crate::orm::user;

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Extension(jwt): Extension<Jwt>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<CommonResponse>, StatusCode> {
    let db_user = get_current_user(payload.username, &db).await?;

    if !verify_password(payload.password, db_user.password.as_ref())? {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let (accece, refleash) = jwt
        .generate_token_pair(db_user.id.unwrap().to_string())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let data = LoginRespon {
        access_token: accece,
        refresh: refleash,
    };

    let mut res = CommonResponse::default();
    res.data = data.into_serde_json();
    Ok(Json(res))
}

async fn get_current_user(
    username: String,
    db: &DatabaseConnection,
) -> Result<user::ActiveModel, StatusCode> {
    let db_user = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let user = if let Some(db_user) = db_user {
        db_user.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(user)
}

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_err| StatusCode::UNAUTHORIZED)
}
