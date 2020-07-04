use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Payload {
    pub target: String,
    pub datapoints: Vec<[i64; 2]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub from: chrono::DateTime<Utc>,
    pub to: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub target: Option<String>,

    #[serde(rename = "refId")]
    pub ref_id: String,

    #[serde(rename = "type")]
    pub _type: String,
}

//https://medium.com/@vsbabu/sqlite3-cte-tricks-for-time-series-analysis-196dbf3ffdf9

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    pub range: Range,

    #[serde(rename = "intervalMs")]
    pub interval_ms: u64,

    #[serde(rename = "maxDataPoints")]
    pub max_data_points: u64,

    pub targets: Vec<Target>,
}

#[derive(Debug)]
pub struct State {
    pub db_pool: Option<sqlx::SqlitePool>,
}
