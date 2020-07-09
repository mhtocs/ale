use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub id: i32,
    pub epoch: i64,
    pub value: i32,
}

#[derive(Debug)]
pub struct State {
    pub db_pool: Option<sqlx::SqlitePool>,
}
