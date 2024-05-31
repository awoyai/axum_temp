use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GreeterReq {
    pub id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct GreeterInfo {
    pub id: i64,
    pub name: String,
}
