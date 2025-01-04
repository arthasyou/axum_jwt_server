use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRespon {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub refresh: String,
}
