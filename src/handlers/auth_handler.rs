use axum::Extension;
use axum::{http::StatusCode, Json};
use bcrypt::{hash, DEFAULT_COST};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use service_utils_rs::services::jwt::Jwt;

use crate::models::auth_model::{LoginRequest, LoginRespon, SignupRequest};
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

pub async fn signup(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<CommonResponse>, StatusCode> {
    // 检查用户名是否已存在
    if is_username_taken(&payload.username, &db).await? {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 哈希密码
    let hashed_password =
        hash(&payload.password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 创建新用户
    let new_user = user::ActiveModel {
        username: Set(payload.username),
        password: Set(hashed_password),
        // roles: Set(Some(vec!["user".to_string()])),
        ..Default::default()
    };

    let _user = User::insert(new_user)
        .exec(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut res = CommonResponse::default();
    res.message = "User registered successfully".to_string();
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
        .map_err(|err| {
            // 打印错误信息
            eprintln!("Database query error: {:?}", err);
            StatusCode::NOT_FOUND
        })?;

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

async fn is_username_taken(username: &str, db: &DatabaseConnection) -> Result<bool, StatusCode> {
    let existing_user = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(existing_user.is_some())
}
