use serde::Serialize;

pub mod auth_model;

pub trait IntoSerdeJson {
    fn into_serde_json(self) -> serde_json::Value;
}

impl<T> IntoSerdeJson for T
where
    T: Serialize,
{
    fn into_serde_json(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to convert to serde_json::Value")
    }
}

#[derive(Debug, Serialize)]
pub struct CommonResponse {
    pub code: u16,
    pub data: serde_json::Value,
    pub message: String,
}
impl Default for CommonResponse {
    fn default() -> Self {
        CommonResponse {
            code: 0,
            data: serde_json::Value::Null,
            message: String::from("Success"),
        }
    }
}
